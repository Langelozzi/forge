use clap::{Parser, Subcommand};

mod commands;
mod templates;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init(commands::init::InitArgs),
    // Run,
    // Build,
    // Add,
}

fn main() {
    let args = Cli::parse();

    match &args.cmd {
        Commands::Init(init_args) => {
            if let Err(e) = commands::init::handle_init(init_args) {
                eprintln!("Error initializing project: {}", e);
            }
        }
    }
}
