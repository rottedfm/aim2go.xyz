
use clap::Parser;

mod cli;
mod config;

use crate::cli::{Cli, create_new_game_folder, edit_config, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New { name }) => {
            let _ = create_new_game_folder(name);
        }
        Some(Commands::Settings { name }) => {
            let _ = edit_config(name);
        }
        _ => {}
    }
}
