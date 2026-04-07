use crate::constants;
use crate::templates::init_templates;
use crate::utils::sys_utils;
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    pub name: String, // Name of the project
}

pub fn handle_init(args: &InitArgs) -> std::io::Result<()> {
    let project_path = Path::new(&args.name);

    // TODO: Commented out for development purposes
    // if project_path.exists() {
    //     eprintln!("Error: Directory '{}' already exists.", args.name);
    //     // Returning an error here stops the function immediately
    //     return Err(std::io::Error::new(
    //         std::io::ErrorKind::AlreadyExists,
    //         "Destination already exists",
    //     ));
    // }

    generate_proj_structure(project_path)?;
    generate_config_file(project_path, &args.name)?;
    generate_main_file(project_path)?;

    println!("Project '{}' initialized.", args.name);
    println!("To get started:");
    println!("  > cd {}", args.name);
    println!("  > {} run", constants::CMD_NAME);
    Ok(())
}

fn generate_proj_structure(project_path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("include"))?;
    Ok(())
}

fn generate_config_file(project_path: &Path, proj_name: &str) -> std::io::Result<()> {
    let compiler_path = &sys_utils::get_default_c_compiler();
    let content = init_templates::FORGE_TOML
        .replace("{name}", proj_name)
        .replace("{compiler}", compiler_path);
    fs::write(project_path.join(constants::CONFIG_FILENAME), &content)?;
    Ok(())
}

fn generate_main_file(project_path: &Path) -> std::io::Result<()> {
    let content = init_templates::MAIN_C;
    let main_path = project_path.join("src").join("main.c");
    fs::write(main_path, content)?;
    Ok(())
}
