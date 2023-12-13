//! Command line arguments
use clap::{Parser, Subcommand};

/// Command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sub commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Sub command arguments
#[derive(Subcommand)]
pub enum Commands {
    /// Team logic
    Team {
        /// Team name
        team: Option<String>,
    },

    /// League logic
    League {
        /// League name
        league: Option<String>,
    },
}
