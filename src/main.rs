use clap::{Parser, Subcommand};

mod commands;
mod config;
mod constants;
mod templates;
mod utils;

#[derive(Parser, Debug)]
#[command(
    author = "Forge Contributors",
    version,
    about = "A lightweight C project build tool",
    long_about = "Forge is a simple yet powerful build tool for C projects.\n\n\
                  It provides an easy way to initialize, build, and manage C projects \
                  with automatic LSP configuration generation (compile_commands.json)."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "Initialize a new C project")]
    Init(commands::init::InitArgs),
    #[command(about = "Add a new module (creates .c and .h files)")]
    Add(commands::add::AddArgs),
    #[command(about = "Build the project")]
    Build(commands::build::BuildArgs),
    #[command(about = "Build and run the project")]
    Run(commands::run::RunArgs),
    #[command(about = "Clean build artifacts")]
    Clean,
}

fn main() {
    let args = Cli::parse();

    match &args.cmd {
        Commands::Init(init_args) => {
            if let Err(e) = commands::init::handle_init(init_args) {
                eprintln!("Error initializing project: {}", e);
            }
        }
        Commands::Add(add_args) => {
            if let Err(e) = commands::add::handle_add(add_args) {
                eprintln!("Error adding module: {}", e);
            }
        }
        Commands::Build(build_args) => {
            if let Err(e) = commands::build::handle_build(build_args) {
                eprintln!("Error building project: {}", e);
            }
        }
        Commands::Run(run_args) => {
            if let Err(e) = commands::run::handle_run(run_args) {
                eprintln!("Error running project: {}", e);
            }
        }
        Commands::Clean => {
            if let Err(e) = commands::clean::handle_clean() {
                eprintln!("Error cleaning up build artifacts: {}", e);
            }
        }
    }
}
