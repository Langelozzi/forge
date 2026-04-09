use serde_json::json;
use std::fs;

use crate::config::ForgeConfig;
use crate::constants;
use crate::utils::fs_utils::get_files_in_dir_recursive;
use crate::utils::vec_utils::difference;

pub fn generate_compile_commands() -> std::io::Result<()> {
    let config = ForgeConfig::load()?;
    let current_dir = std::env::current_dir()?;

    let compiler = &config.build.compiler;
    let default_flags = constants::DEFAULT_FLAGS.join(" ");
    let additional_flags = config.build.flags.join(" ");
    let ignored_files = config.build.ignore_files;
    let included_src_files = get_included_src_files(ignored_files)?;
    let exe_path = format!("{}/{}", constants::BUILD_DIR, config.project.name);

    let build_cmd = format!(
        "{} {} {} {} -o {}",
        compiler,
        included_src_files.join(" "),
        default_flags,
        additional_flags,
        exe_path
    );

    let mut commands = Vec::new();

    // Add entry for each source file
    for src_file in &included_src_files {
        commands.push(json!({
            "directory": current_dir.to_string_lossy().to_string(),
            "command": build_cmd,
            "file": src_file
        }));
    }

    // Add entries for all header files
    if let Ok(header_files) = get_files_in_dir_recursive(constants::INCLUDE_DIR, true) {
        for header_file in header_files {
            commands.push(json!({
                "directory": current_dir.to_string_lossy().to_string(),
                "command": build_cmd,
                "file": header_file
            }));
        }
    }

    let json_output = serde_json::to_string_pretty(&commands)?;
    fs::write("compile_commands.json", json_output)?;

    Ok(())
}

fn get_included_src_files(ignored_files: Vec<String>) -> std::io::Result<Vec<String>> {
    let all_files = get_files_in_dir_recursive(constants::SRC_DIR, true)?;
    Ok(difference(all_files, ignored_files))
}
