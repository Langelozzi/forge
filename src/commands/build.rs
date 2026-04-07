use clap::Args;

use crate::config::ForgeConfig;
use crate::constants;
use crate::utils::fs_utils::assert_toml_in_root;
use crate::utils::fs_utils::create_dir_if_not_exists;
use crate::utils::fs_utils::get_files_in_dir_recursive;
use crate::utils::shell_utils::exec;
use crate::utils::vec_utils::difference;

#[derive(Args, Debug, Clone)]
pub struct BuildArgs {
    // pub module_name: String,
}

pub fn handle_build(args: &BuildArgs) -> std::io::Result<()> {
    assert_toml_in_root()?;

    let config = ForgeConfig::load()?;

    // Check if the build directory exists and if not create it
    create_dir_if_not_exists(constants::BUILD_DIR)?;

    let compiler = config.build.compiler;
    let default_flags = constants::DEFAULT_FLAGS.join(" ");
    let additional_flags = config.build.flags.join(" ");
    let ignored_files = config.build.ignore_files;
    let included_src_files = get_included_src_files(ignored_files)?.join(" ");
    let exe_path = format!("{}/{}", constants::BUILD_DIR, config.project.name);

    // The build command should look something like this:
    // gcc main.c utils.c -Wall -g -I./include -o my_project
    // <toml_compiler> <...files in src - ignored> -Wall -g -I./include <toml_flags> -o <toml_proj_name>
    let build_cmd =
        format!("{compiler} {included_src_files} {default_flags} {additional_flags} -o {exe_path}");

    println!("{}", build_cmd);
    exec(&build_cmd)?;

    println!("Project built successfully!");

    Ok(())
}

fn get_included_src_files(ignored_files: Vec<String>) -> std::io::Result<Vec<String>> {
    let all_files = get_files_in_dir_recursive(constants::SRC_DIR, true)?;
    Ok(difference(all_files, ignored_files))
}
