mod cli;
mod tasks;
mod config;
mod commands;
// mod config;
// mod storage;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};

// use config::RsaveConfig;

#[warn(unused_variables, dead_code)]
fn main() {
    // let mut cfg = RsaveConfig::load();
    //
    // cfg.add_destination_secure("prod", "my-bucket", "eu-west-1", "AKIA123", "SECRET456");
    // let prod_credentials = cfg.get_credentials("prod");
    // match prod_credentials {
    //     Some(prod_credentials) => {
    //         println!("Prod credentials. {:?}",prod_credentials);
    //     },
    //     None => {
    //         println!("No Prod credentials.");
    //     }
    // }
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Backup { name }) => println!("Backup: {}", name),
        Some(Commands::Status) => println!("Status"),
        Some(Commands::History) => println!("History"),
        Some(Commands::Config {list, add}) => {
            if list {
                commands::handle_list_configs()
            }
        },
        None => {
            Cli::command()
                .print_help()
                .expect("An error occurred while printing help");
        }
    }
}
