#[cfg(feature = "calculator-build")]
use crate::prelude::*;

use thiserror::Error;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[cfg(feature = "calculator-build")]
    #[error("You need at least two save files with the extension '.sav'. Place them in the top level directory.")]
    MissingFiles,

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    #[error("You need at least two save files with the extension '.sav'. Place them on your desktop.")]
    MissingFiles,

    #[cfg(target_os = "linux")]
    #[error("You need at least two save files with the extension '.sav'. Place them in your home directory, for example ~/emerald.sav")]
    MissingFiles,

    #[error("Filesystem error: {0}")]
    FsError(String),

    #[error("Pokemon not found.")]
    PokemonNotFound,

    #[error("Save error: {0}")]
    SaveError(pkmn_savedata::SaveError),

    #[error("{0}")]
    Custom(String),
}

impl From<pkmn_savedata::SaveError> for AppError {
    fn from(err: pkmn_savedata::SaveError) -> Self {
        AppError::SaveError(err)
    }
}
