use clap::{Parser, Subcommand};

mod commands;
mod config;
mod constants;
mod templates;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init(commands::init::InitArgs),
    Add(commands::add::AddArgs),
    Build(commands::build::BuildArgs),
    Run(commands::run::RunArgs),
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
