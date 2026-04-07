use crate::constants::CONFIG_FILENAME;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub fn assert_toml_in_root() -> Result<()> {
    match fs::exists(CONFIG_FILENAME) {
        // Case 1: File exists
        Ok(true) => Ok(()),

        // Case 2: File is missing
        Ok(false) => {
            eprintln!(
                "Error: Could not find {} in the current directory.",
                CONFIG_FILENAME
            );
            eprintln!("Are you sure you are in a Forge project root?");

            // Return a clean error
            Err(Error::new(ErrorKind::NotFound, "Forge config not found"))
        }

        // Case 3: We can't even check (e.g., permission denied)
        Err(e) => Err(e),
    }
}

pub fn create_dir_if_not_exists(dir_name: &str) -> std::io::Result<()> {
    let path = Path::new(dir_name);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_files_in_dir_recursive(dir_name: &str, include_path: bool) -> Result<Vec<String>> {
    let path = Path::new(dir_name);
    let mut filenames = Vec::new();
    visit_dir(path, &mut filenames, include_path)?;
    Ok(filenames)
}

fn visit_dir(path: &Path, filenames: &mut Vec<String>, include_path: bool) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursive call for subdirectories
                visit_dir(&path, filenames, include_path)?;
            } else if path.is_file() {
                if include_path {
                    // Convert the full path to a string
                    // to_string_lossy handles non-utf8 characters safely
                    filenames.push(path.to_string_lossy().to_string());
                } else {
                    // Just get the filename
                    if let Some(name_str) = path.file_name().and_then(|f| f.to_str()) {
                        filenames.push(name_str.to_string());
                    }
                }
            }
        }
    }
    Ok(())
}
