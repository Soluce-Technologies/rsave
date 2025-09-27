mod cli;
mod commands;
mod config;
mod tasks;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use crate::config::RsaveConfig;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Backup) => commands::backup::handle_backup(),
        Some(Commands::Status) => {
            println!("Status");
            let config = RsaveConfig::load();
            config.list_destinations_secure();
        },
        Some(Commands::History) => println!("History"),
        Some(Commands::Config { list, add }) => {
            if list && add {
                eprintln!("Error: You cannot use --list and --add at the same time.");
                std::process::exit(1);
            }
            if list {
                commands::config::handle_list_configs();
            }
            if add {
                commands::config::handle_add_config();
            }
            commands::config::handle_choice_config()
        }
        None => {
            Cli::command()
                .print_help()
                .expect("An error occurred while printing help");
        }
    }
}
