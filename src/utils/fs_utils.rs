use crate::constants::CONFIG_FILENAME;
use std::fs;
use std::io::{Error, ErrorKind, Result};

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
