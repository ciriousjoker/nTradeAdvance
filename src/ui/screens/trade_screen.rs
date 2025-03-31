use pkmn_savedata::gba::{GbaSave, LanguageGBA};

use crate::prelude::*;
use crate::ui::rendering::IntoWidget;
use core::cmp::min;

#[derive(PartialEq, Eq, Clone)]
enum Trainer {
    Trainer1,
    Trainer2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Action {
    Back,
    Trade,
}

#[derive(Clone)]
pub struct TradeScreen {
    saves: Option<(String, String)>,
    selected_pokemon_trainer1: Option<usize>,
    selected_pokemon_trainer2: Option<usize>,
    pokemon_list_trainer1: Vec<String>,
    pokemon_list_trainer2: Vec<String>,
    current_trainer: Trainer,
    trainer_name1: String,
    trainer_name2: String,
    cursor_position: usize,
    selected_action: Option<Action>,
    message: String,
    progress: Option<f32>,
}

impl TradeScreen {
    pub fn new() -> Self {
        TradeScreen {
            saves: None,
            selected_pokemon_trainer1: None,
            selected_pokemon_trainer2: None,
            pokemon_list_trainer1: vec![
                "Bulbasaur".to_string(),
                "Charmander".to_string(),
                "Squirtle".to_string(),
                "Pikachu".to_string(),
            ],
            pokemon_list_trainer2: vec![
                "Eevee".to_string(),
                "Vaporeon".to_string(),
                "Jolteon".to_string(),
                "Flareon".to_string(),
            ],
            trainer_name1: String::new(),
            trainer_name2: String::new(),
            current_trainer: Trainer::Trainer1,
            cursor_position: 0,
            selected_action: None,
            message: String::new(),
            progress: None,
        }
    }

    /// Animates the progress value from `from` to `to` over `duration_ms` milliseconds,
    /// using a cubic ease‑out curve (ease(t)=1 - (1-t)³). During the animation, the screen
    /// is cleared, the UI is rebuilt, and the console is flushed.
    pub fn animate_progress(&mut self, from: f32, to: f32, duration_ms: u32) {
        #[cfg(feature = "calculator-build")]
        // Frame times on a calculator are nowhere near 60 FPS.
        // If we picked a higher number than achievable, our calculations will be off
        // and the animation takes longer than intended.
        const FPS: u32 = 30;

        #[cfg(feature = "desktop")]
        const FPS: u32 = 60;

        let frame_time: u32 = 1000 / FPS;
        // Calculate the total number of frames (ignoring any remainder).
        let total_frames = duration_ms / frame_time;

        for frame in 0..total_frames {
            // Normalized time in [0,1].
            let t = (frame as f32) / (total_frames as f32);
            // Cubic ease-out function.
            let ease = 1.0 - (1.0 - t).powi(3);
            self.progress = Some(from + (to - from) * ease);
            console::clear_screen();
            self.build();
            console::flush();
            sleep(frame_time);
        }
        // Ensure the progress value is exactly 'to' at the end.
        self.progress = Some(to);
        console::clear_screen();
        self.build();
        console::flush();
    }
}

impl Screen for TradeScreen {
    fn init(&mut self) -> Result<NavAction> {
        let dir = get_dir();
        let files = read_dir(&dir)?;

        let (name1, name2) = find_first_two_sav_tns(&files)?;

        self.saves = Some((name1.clone(), name2.clone()));

        let save1 = load_save(&name1)?;
        let save2 = load_save(&name2)?;

        self.trainer_name1 = trainer_name(&save1);
        self.trainer_name2 = trainer_name(&save2);

        self.pokemon_list_trainer1 = party_names(&save1)?;
        self.pokemon_list_trainer2 = party_names(&save2)?;

        Ok(NavAction::None)
    }

    fn build(&mut self) {
        let data = self.clone();

        let ui = border(column(widget_vec![
            sizedbox(
                align(text("Trading"))
                    .horizontal(AlignHorizontal::Center)
                    .vertical(AlignVertical::Center),
            )
            .height(3),
            align(row(widget_vec![border(row(widget_vec![
                sizedbox(column(widget_vec![
                    align(text(&format!("{}", self.trainer_name1)))
                        .horizontal(AlignHorizontal::Center),
                    divider('-'),
                    column(
                        self.pokemon_list_trainer1
                            .iter()
                            .enumerate()
                            .map(|(i, name)| {
                                let highlighted = self.current_trainer == Trainer::Trainer1
                                    && self.cursor_position == i
                                    && self.selected_action.is_none();
                                let selected = self.selected_pokemon_trainer1 == Some(i);
                                let pokemon_name = if selected && highlighted {
                                    format!(" [>{:10}<] ", name)
                                } else if selected {
                                    format!("  >{:10}<  ", name)
                                } else if highlighted {
                                    format!(" [ {:10} ] ", name)
                                } else {
                                    format!("   {:10}   ", name)
                                };

                                text(pokemon_name)
                            })
                            .collect(),
                    ),
                ]))
                .width(16),
                divider('|').vertical(),
                sizedbox(column(widget_vec![
                    align(text(&format!("{}", self.trainer_name2)))
                        .horizontal(AlignHorizontal::Center),
                    divider('-'),
                    column(
                        self.pokemon_list_trainer2
                            .iter()
                            .enumerate()
                            .map(|(i, name)| {
                                let highlighted = self.current_trainer == Trainer::Trainer2
                                    && self.cursor_position == i
                                    && self.selected_action.is_none();
                                let selected = self.selected_pokemon_trainer2 == Some(i);
                                let pokemon_name = if selected && highlighted {
                                    format!(" [>{:10}<] ", name)
                                } else if selected {
                                    format!("  >{:10}<  ", name)
                                } else if highlighted {
                                    format!(" [ {:10} ] ", name)
                                } else {
                                    format!("   {:10}   ", name)
                                };

                                text(pokemon_name)
                            })
                            .collect(),
                    ),
                ]))
                .width(16),
            ]))
            .corners(CORNERS_ROUND),]))
            .horizontal(AlignHorizontal::Center),
            text(""),
            align(row(widget_vec![
                sizedbox(button("Back").selected(self.selected_action == Some(Action::Back)))
                    .width(12),
                text("   "),
                sizedbox(button("Trade").selected(self.selected_action == Some(Action::Trade)))
                    .width(12),
            ]),),
            flexible(1, align(text(&self.message)),),
            padding(builder(move || {
                if let Some(progress) = data.progress {
                    return border(progress_bar(progress, '/', '|', ' '))
                        .corners(CORNERS_ROUND)
                        .into_widget();
                }
                return text("").into_widget();
            }))
            .horizontal(2)
            .bottom(1),
        ]))
        .borders(Borders {
            left: Some('|'),
            right: Some('|'),
            top: Some('-'),
            bottom: Some('-'),
        })
        .corners(Corners {
            top_left: Some('+'),
            top_right: Some('+'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
        });

        let output = render_ui(ui);
        console::print(&output);
        console::flush();
    }

    fn handle_input(&mut self) -> Result<NavAction> {
        let input = wait_input();
        self.message = String::new();

        match input {
            InputKey::Up => {
                if self.selected_action.is_some() {
                    self.selected_action = None;
                } else if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            InputKey::Down => {
                if self.selected_action.is_none()
                    && ((self.current_trainer == Trainer::Trainer1
                        && self.cursor_position == self.pokemon_list_trainer1.len() - 1)
                        || (self.current_trainer == Trainer::Trainer2
                            && self.cursor_position == self.pokemon_list_trainer2.len() - 1))
                {
                    self.selected_action = Some(Action::Back);
                } else {
                    let max_index = match self.current_trainer {
                        Trainer::Trainer1 => self.pokemon_list_trainer1.len() - 1,
                        Trainer::Trainer2 => self.pokemon_list_trainer2.len() - 1,
                    };
                    if self.cursor_position < max_index {
                        self.cursor_position += 1;
                    }
                }
            }
            InputKey::Left => {
                if self.selected_action == Some(Action::Trade) {
                    self.selected_action = Some(Action::Back);
                } else {
                    self.current_trainer = Trainer::Trainer1;
                    self.cursor_position =
                        min(self.cursor_position, self.pokemon_list_trainer1.len() - 1);
                    self.selected_action = None;
                }
            }
            InputKey::Right => {
                if self.selected_action == Some(Action::Back) {
                    self.selected_action = Some(Action::Trade);
                } else {
                    self.current_trainer = Trainer::Trainer2;
                    self.cursor_position =
                        min(self.cursor_position, self.pokemon_list_trainer2.len() - 1);
                    self.selected_action = None;
                }
            }
            InputKey::Enter => {
                if let Some(action) = &self.selected_action {
                    return match action {
                        Action::Back => Ok(NavAction::Pop),
                        Action::Trade => {
                            if let (Some(p1), Some(p2)) = (
                                self.selected_pokemon_trainer1,
                                self.selected_pokemon_trainer2,
                            ) {
                                let animation_targets = [0.32, 0.5, 0.68, 1.0];

                                self.message = String::from("Trading...");
                                self.progress = Some(0.0);
                                console::clear_screen();
                                self.build();
                                console::flush();

                                self.animate_progress(0.0, animation_targets[0], 750);

                                let saves = self
                                    .saves
                                    .clone()
                                    .ok_or(AppError::Custom("The save files that were loaded into memory are no longer there. wtf?".into()))?;

                                let mut save1 = load_save(&saves.0)?;
                                let mut save2 = load_save(&saves.1)?;

                                trade_pokemon(&mut save1, &mut save2, p1, p2)?;

                                self.animate_progress(
                                    animation_targets[0],
                                    animation_targets[1],
                                    750,
                                );

                                let buf1 = save1.to_bytes().map_err(|_| {
                                    AppError::Custom(format!(
                                        "Failed to serialize save file {}. Don't worry, nothing has been written yet.",
                                        saves.0
                                    ))
                                })?;
                                let buf2 = save2.to_bytes().map_err(|_| {
                                    AppError::Custom(format!(
                                        "Failed to serialize save file {}. Don't worry, nothing has been written yet.",
                                        saves.1
                                    ))
                                })?;

                                self.animate_progress(
                                    animation_targets[1],
                                    animation_targets[2],
                                    750,
                                );

                                write_save(&saves.0, &buf1)?;
                                write_save(&saves.1, &buf2)?;

                                self.animate_progress(
                                    animation_targets[2],
                                    animation_targets[3],
                                    750,
                                );

                                sleep(100);
                                self.message = String::from("Trade successful!");
                                console::clear_screen();
                                self.build();
                                console::flush();

                                let input = wait_input();
                                match input {
                                    InputKey::Escape => Ok(NavAction::Pop),
                                    _ => Ok(NavAction::Go(Box::new(TradeScreen::new()))),
                                }
                            } else {
                                self.message =
                                    String::from("Please select a pokemon from each trainer.");
                                Ok(NavAction::None)
                            }
                        }
                    };
                }

                // Toggle Pokémon selection for the current trainer.
                match self.current_trainer {
                    Trainer::Trainer1 => {
                        if self.selected_pokemon_trainer1 == Some(self.cursor_position) {
                            self.selected_pokemon_trainer1 = None;
                        } else {
                            self.selected_pokemon_trainer1 = Some(self.cursor_position);
                        }
                    }
                    Trainer::Trainer2 => {
                        if self.selected_pokemon_trainer2 == Some(self.cursor_position) {
                            self.selected_pokemon_trainer2 = None;
                        } else {
                            self.selected_pokemon_trainer2 = Some(self.cursor_position);
                        }
                    }
                }
            }
            InputKey::Escape => {
                return Ok(NavAction::Pop);
            }
        }
        Ok(NavAction::None)
    }
}

fn load_save(name: &str) -> Result<GbaSave> {
    let buf = read_file(&save_file_path(name))
        .map_err(|e| AppError::Custom(format!("Failed to read save file {}: {:?}", name, e)))?;
    GbaSave::from_bytes(&buf, LanguageGBA::English)
        .map_err(|_| AppError::Custom(format!("Failed to parse save file {}.", name)))
}

fn write_save(name: &str, buf: &[u8]) -> Result<()> {
    let path = save_file_path(name);
    write_file(&path, buf).map_err(|e| {
        AppError::Custom(format!(
            "Failed to write save file {}. Data might be corrupt now. Error: {:?}",
            name, e
        ))
    })?;
    Ok(())
}
