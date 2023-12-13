//! Module: parser

pub mod cli;
pub mod configfile;

pub use cli::{Cli, Commands};
pub use configfile::get_config;
