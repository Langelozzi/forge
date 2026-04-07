use clap::Args;
use std::path::Path;

use crate::constants::INCLUDE_DIR;
use crate::constants::SRC_DIR;
use crate::templates::add_templates;
use crate::utils::fs_utils::change_to_proj_root;
use crate::utils::fs_utils::write_file;

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    pub module_name: String,
}

pub fn handle_add(args: &AddArgs) -> std::io::Result<()> {
    change_to_proj_root()?;

    generate_module_c(&args.module_name)?;
    generate_module_h(&args.module_name)?;

    Ok(())
}

pub fn generate_module_c(mod_name: &str) -> std::io::Result<()> {
    let mod_lower = mod_name.to_lowercase();
    let filename = format!("{}.c", mod_lower);
    let path = Path::new(SRC_DIR).join(filename);

    let contents = add_templates::MODULE_C.replace("{mod_name}", &mod_lower);

    write_file(&path, &contents)?;
    Ok(())
}

pub fn generate_module_h(mod_name: &str) -> std::io::Result<()> {
    let filename = mod_name.to_lowercase() + ".h";
    let path = Path::new(INCLUDE_DIR).join(filename);

    let contents = add_templates::MODULE_H.replace("{mod_name}", &mod_name.to_uppercase());

    write_file(&path, &contents)?;
    Ok(())
}
