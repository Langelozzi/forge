use crate::constants::CONFIG_FILENAME;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

pub fn change_to_proj_root() -> std::io::Result<()> {
    if let Some(root) = find_project_root() {
        env::set_current_dir(&root)?;
        Ok(())
    } else {
        eprintln!(
            "Error: No forge project found. Could not find {}.\nUse `forge init` to start a new forge project.",
            CONFIG_FILENAME
        );
        std::process::exit(1);
    }
}

pub fn find_project_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;

    // Backtrack until we find the forge.toml
    loop {
        // Does forge.toml exist here?
        if current_dir.join(CONFIG_FILENAME).exists() {
            return Some(current_dir);
        }

        // Try the parent directory
        if !current_dir.pop() {
            // Reached the system root (/) without finding it
            return None;
        }
    }
}

pub fn create_dir_if_not_exists(dir_name: &str) -> std::io::Result<()> {
    let path = Path::new(dir_name);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn write_file(path: &Path, contents: &str) -> std::io::Result<()> {
    let existed = path.exists();

    fs::write(path, contents)?;

    if !existed {
        println!("New file added: {}", path.to_string_lossy());
    } else {
        println!("File updated: {}", path.to_string_lossy());
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
