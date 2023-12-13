//! This module contains the functions to parse the config file.
use std::{error::Error, fs};

use dirs;
use serde::Deserialize;
use toml;

const CONFIG_FILE_PATH: &str = ".config/fulbo/config.toml";

#[derive(Deserialize)]
struct Data {
    config: Config,
}

/// Config file structure
#[derive(Deserialize)]
pub struct Config {
    /// Home team name
    pub team: Option<String>,
    /// Home league name
    pub league: Option<String>,
}

/// Get the configuration from the config file located at
/// ~/.config/fulbo/config.toml
pub fn get_config() -> Result<Config, Box<dyn Error>> {
    println!("Trying to read the file");
    let home_dir_result = dirs::home_dir();
    let mut home_dir = match home_dir_result {
        Some(path) => path,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            )))
        }
    };
    home_dir.push(CONFIG_FILE_PATH);

    let config_str = fs::read_to_string(home_dir)?;
    println!("Read successfully");
    let data: Data = toml::from_str(&config_str)?;
    Ok(data.config)
}
