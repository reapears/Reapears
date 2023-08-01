//! Server cli impls.

use clap::{Parser, Subcommand};

/// Collect config values from cli
#[derive(Clone, Parser)]
pub struct ConfigCli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    // /// if true, create an api key
    // /// and prints it on the console.
    // pub api_key: bool,
}

/// Cli subcommands.
#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Creates new superuser.
    WithSuperuser {
        #[arg(short, long)]
        email: String,

        #[arg(short, long)]
        password: String,
    },
}
