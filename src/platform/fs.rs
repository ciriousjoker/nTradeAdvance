#[cfg(feature = "calculator-build")]
mod calculator {
    use crate::prelude::*;
    // We need EXT_SAVEFILE here for the calculator-specific stem logic
    use crate::saves::EXT_SAVEFILE;
    use ndless::alloc::string::String;
    use ndless::alloc::vec::Vec;
    use ndless::fs::File;
    use ndless::io::prelude::*;

    /// Returns the directory for save files on the calculator.
    pub fn get_dir() -> String {
        String::from("/documents")
    }

    /// Reads the directory entries at the given path.
    pub fn read_dir(path: &str) -> Result<Vec<String>> {
        ndless::fs::read_dir(path)
            .map_err(|e| AppError::FsError(format!("Failed to read directory: {}", e)))
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok())
                    .map(|e| e.path().to_string_lossy().into_owned())
                    .collect()
            })
    }

    /// Reads the contents of a file.
    pub fn read_file(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)
            .map_err(|e| AppError::FsError(format!("Failed to open file: {}", e)))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| AppError::FsError(format!("Failed to read file: {}", e)))?;
        Ok(buffer)
    }

    /// Writes data to a file.
    pub fn write_file(path: &str, buf: &[u8]) -> Result<()> {
        let mut file = File::create(path)
            .map_err(|e| AppError::FsError(format!("Failed to create file: {}", e)))?;
        file.write_all(buf)
            .map_err(|e| AppError::FsError(format!("Failed to write file: {}", e)))?;
        Ok(())
    }

    /// Gets the file stem (name without extension) using calculator path logic.
    pub fn get_file_basename(full_path: &str) -> String {
        let filename = full_path.rsplit('/').next().unwrap_or(full_path);
        filename
            .strip_suffix(EXT_SAVEFILE)
            .unwrap_or(filename)
            .to_string()
    }

    /// Joins path components using calculator path logic (forward slashes).
    pub fn path_join(base_dir: &str, filename_with_ext: &str) -> String {
        format!("{}/{}", base_dir, filename_with_ext)
    }
}

#[cfg(not(feature = "calculator-build"))]
mod desktop {
    use crate::prelude::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Returns the standard directory based on the platform.
    pub fn get_dir() -> String {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            dirs::desktop_dir()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| String::from("."))
        }

        #[cfg(target_os = "linux")]
        {
            dirs::home_dir()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| String::from("."))
        }
    }

    /// Reads directory entries at the given path.
    pub fn read_dir(path: &str) -> Result<Vec<String>> {
        fs::read_dir(Path::new(path))
            .map_err(|e| AppError::FsError(e.to_string()))
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok().map(|e| e.path().to_string_lossy().into_owned()))
                    .collect()
            })
    }

    /// Reads a file's contents.
    pub fn read_file(path: &str) -> Result<Vec<u8>> {
        fs::read(Path::new(path)).map_err(|e| AppError::FsError(e.to_string()))
    }

    /// Writes data to a file.
    pub fn write_file(path: &str, buf: &[u8]) -> Result<()> {
        fs::write(Path::new(path), buf).map_err(|e| AppError::FsError(e.to_string()))
    }

    /// Gets the file stem (name without extension) using standard library path logic.
    pub fn get_file_basename(full_path: &str) -> String {
        Path::new(full_path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            // Fallback: if no stem (e.g., hidden file ".sav"), use the filename itself
            .unwrap_or_else(|| {
                Path::new(full_path)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| full_path.to_string()) // Final fallback to full path if needed
            })
    }

    /// Joins path components using standard library path logic.
    pub fn path_join(base_dir: &str, filename_with_ext: &str) -> String {
        let mut path_buf = PathBuf::from(base_dir);
        path_buf.push(filename_with_ext);
        path_buf.to_string_lossy().into_owned()
    }
}

#[cfg(feature = "calculator-build")]
pub use calculator::*;

#[cfg(not(feature = "calculator-build"))]
pub use desktop::*;
