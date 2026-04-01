use clap::Args;

use crate::utils::fs_utils::assert_toml_in_root;

#[derive(Args, Debug, Clone)]
pub struct BuildArgs {
    // pub module_name: String,
}

pub fn handle_build(args: &BuildArgs) -> std::io::Result<()> {
    assert_toml_in_root()?;

    // The build command should look something like this:
    // gcc main.c utils.c -Wall -g -I./include -o my_project
    // <toml_compiler> <...files in src - ignored> -Wall -g -I./include <toml_flags> -o <toml_proj_name>

    Ok(())
}
