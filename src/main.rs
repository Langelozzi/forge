use clap::{Parser, Subcommand};

mod commands;
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
    // Run,
    // Build,
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
    }
}
