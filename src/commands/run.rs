use clap::Args;

use crate::commands::build::build;
use crate::config::ForgeConfig;
use crate::constants;
use crate::utils::fs_utils::change_to_proj_root;
use crate::utils::shell_utils::exec_raw;

#[derive(Args, Debug, Clone)]
pub struct RunArgs {}

pub fn handle_run(args: &RunArgs) -> std::io::Result<()> {
    change_to_proj_root()?;

    build()?;

    let config = ForgeConfig::load()?;
    let exe_path = format!("{}/{}", constants::BUILD_DIR, config.project.name);

    let run_cmd = format!("./{exe_path}");

    println!("> {}\n", run_cmd);
    exec_raw(&run_cmd)?;

    Ok(())
}
