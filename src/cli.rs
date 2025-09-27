use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rsave",
    version = env!("CARGO_PKG_VERSION"),
    about = "Rust-based CLI for backup task and save them to s3 storage."
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configuration CLI command.
    Config {
        /// List all storage config.
        #[arg(short = 'l', long = "list")]
        list: bool,
        /// Add a new storage config.
        #[arg(short = 'a', long = "add")]
        add: bool,
    },
    /// Backup dir of files.
    Backup,
    /// Show backup history (done/failed)
    History,
    /// Show in-progress backups tasks.
    Status,
}
