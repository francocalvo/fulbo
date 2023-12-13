//! Purpose: Main library file for the project.

use clap::Parser;
use parser::{Cli, Commands};

pub mod datasource;
pub mod domain;
pub mod parser;

/// Test
fn print_branch(comm: &str, branch: &str) {
    println!("{}: {}", comm, branch);
}

/// Run the application.
pub fn run() {
    let config_result = parser::configfile::get_config();
    let config = match config_result {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let cli_args = Cli::parse();

    match cli_args.command {
        Some(Commands::Team {
            team,
        }) => match team {
            Some(team) => {
                print_branch("team", &team);
            }
            None => match config.team {
                Some(team) => {
                    print_branch("team", &team);
                }
                None => {
                    println!("No team was specified");
                }
            },
        },
        Some(Commands::League {
            league,
        }) => match league {
            Some(league) => print_branch("league", &league),
            None => match config.team {
                Some(league) => print_branch("league", &league),
                None => {
                    println!("No league was specified")
                }
            },
        },
        None => {
            println!("No subcommand was used");
        }
    }
}
