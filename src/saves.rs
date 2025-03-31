use crate::prelude::*;
pub const EXT_SAVEFILE: &str = {
    #[cfg(feature = "calculator-build")]
    {
        ".sav.tns"
    }
    #[cfg(feature = "desktop")]
    {
        ".sav"
    }
};

pub fn find_first_two_sav_tns(files: &[String]) -> Result<(String, String)> {
    let mut matches = files.iter().filter(|f| f.ends_with(EXT_SAVEFILE)).take(2);

    let first_full_path = matches.next().ok_or_else(|| AppError::MissingFiles)?;
    let second_full_path = matches.next().ok_or_else(|| AppError::MissingFiles)?;

    // Use the abstracted function from platform::fs
    let first_stripped = get_file_basename(first_full_path);
    let second_stripped = get_file_basename(second_full_path);

    Ok((first_stripped, second_stripped))
}

use pkmn_savedata::{core_types::PokemonSpecies, gba::GbaSave};

pub fn save_file_path(name: &str) -> String {
    // Construct filename with extension
    let filename_with_ext = format!("{}{}", name, EXT_SAVEFILE);
    // Use the abstracted function from platform::fs
    path_join(&get_dir(), &filename_with_ext)
}

pub fn trainer_name(save: &GbaSave) -> String {
    save.game_state().trainer_name().decode(true)
}

pub fn party_names(save: &GbaSave) -> Result<Vec<String>> {
    let game_state = save.game_state();
    let mut party_names = Vec::new();
    for pkm in game_state.party_iter() {
        let species_national = pkm.decode().species().ok_or(AppError::Custom(
            "Pokemon has unknown species (code 1).".to_string(),
        ))?;
        let species = PokemonSpecies::try_from(species_national)
            .map_err(|_| AppError::Custom("Pokemon has unknown species (code 2).".to_string()))?;

        let pokemon_name = species.to_string();
        party_names.push(pokemon_name);
    }
    Ok(party_names)
}

pub fn trade_pokemon(
    save1: &mut GbaSave,
    save2: &mut GbaSave,
    pokemon1: usize,
    pokemon2: usize,
) -> Result<()> {
    let state1 = save1.game_state_mut();
    let state2 = save2.game_state_mut();

    let pkm1 = state1
        .party_iter_mut()
        .nth(pokemon1)
        .ok_or(AppError::PokemonNotFound)?
        .clone();
    let pkm2 = state2
        .party_iter_mut()
        .nth(pokemon2)
        .ok_or(AppError::PokemonNotFound)?
        .clone();

    // Remove both Pokemon from their party
    state1.party_remove(pokemon1 as u32)?;
    state2.party_remove(pokemon2 as u32)?;

    // Save 1 <-- Pokemon 2
    state1.party_append(&pkm2)?;
    let species_national2 = pkm2.decode().species().ok_or(AppError::Custom(
        "Pokemon 2 couln't be added to the Pokedex of 1 (code 1).".to_string(),
    ))?;
    let species2 = PokemonSpecies::try_from(species_national2).map_err(|_| {
        AppError::Custom("Pokemon 2 couln't be added to the Pokedex of 1 (code 2).".to_string())
    })?;
    state1.set_pokedex_species(species2);

    // Pokemon1 --> Save 2
    state2.party_append(&pkm1)?;
    let species_national1 = pkm1.decode().species().ok_or(AppError::Custom(
        "Pokemon 1 couln't be added to the Pokedex of 2 (code 1).".to_string(),
    ))?;
    let species1 = PokemonSpecies::try_from(species_national1).map_err(|_| {
        AppError::Custom("Pokemon 1 couln't be added to the Pokedex of 2 (code 2).".to_string())
    })?;
    state2.set_pokedex_species(species1);

    Ok(())
}
