mod cli;
mod commands;
mod config;
mod tasks;
mod utils;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Run beacon if called with --beacon
    if args.contains(&"--beacon".to_string()) {
        utils::session::session_cleaner();
        return;
    }

    // Spawn beacon on first CLI run
    utils::session::spawn_beacon();

    let cli = Cli::parse();

    let mut cfg = commands::config::handle_config();

    match cli.command {
        Some(Commands::Backup) => commands::backup::handle_backup(),
        Some(Commands::Status) => {
            println!("Status");
        }
        Some(Commands::History) => println!("History"),
        Some(Commands::Config { list, add }) => {
            if list && add {
                eprintln!("Error: You cannot use --list and --add at the same time.");
                std::process::exit(1);
            }
            if list {
                commands::config::handle_list_configs(&mut cfg);
            }
            if add {
                commands::config::handle_add_config(&mut cfg);
            }
            commands::config::handle_choice_config(&mut cfg)
        }
        None => {
            Cli::command()
                .print_help()
                .expect("An error occurred while printing help");
        }
    }
}
