//! Server cli impls.

use clap::{Parser, Subcommand};

/// Collect config values from cli
#[derive(Clone, Parser)]
pub struct ConfigCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
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
