use crate::constants;
use crate::utils::fs_utils::change_to_proj_root;
use crate::utils::shell_utils::exec_raw;

pub fn handle_clean() -> std::io::Result<()> {
    change_to_proj_root()?;

    let cmd = format!("rm -r {}", constants::BUILD_DIR);

    println!("> {cmd}");
    exec_raw(&cmd)?;
    println!("Build artifacts successfully removed.");

    Ok(())
}
