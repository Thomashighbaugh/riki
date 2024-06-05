// src/config.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub wiki_paths: HashMap<String, PathBuf>,
    pub templates_dir: PathBuf,
    pub index_dir: PathBuf,
    pub editor: Option<String>,
    pub snippet_length: usize,
}

impl Config {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| {
            eprintln!("Error: Could not determine home directory.");
            std::process::exit(1);
        });
        let riki_dir = home_dir.join(".riki");
        let templates_dir = riki_dir.join("templates");
        let config_path = riki_dir.join("config.yaml");

        // Create the .riki directory if it doesn't exist
        if !riki_dir.exists() {
            fs::create_dir_all(&riki_dir).unwrap();
        }

        // Install templates if the templates directory doesn't exist
        if !templates_dir.exists() {
            install_default_templates(&templates_dir).unwrap();
        }

        Config {
            wiki_paths: HashMap::new(),
            templates_dir,
            index_dir: home_dir.join(".riki").join("index"),
            editor: None,
            snippet_length: 150,
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::new(
            ErrorKind::Other,
            "Error: Could not determine the user's home directory. \
             Make sure your system is configured properly.",
        )
    })?;
    let riki_dir = home_dir.join(".riki");
    let config_path = riki_dir.join("config.yaml");

    if !config_path.exists() {
        // Create a default config file if it doesn't exist
        let config = Config::new();
        save_config(&config)?;
        return Ok(config);
    }

    // Handle potential errors during file reading
    let config_str = fs::read_to_string(config_path).map_err(|err| {
        Error::new(
            ErrorKind::Other,
            format!(
                "Error: Could not read configuration file '{}': {}",
                config_path.display(),
                err
            ),
        )
    })?;

    let mut config: Config = serde_yaml::from_str(&config_str).map_err(|err| {
        Error::new(
            ErrorKind::Other,
            format!(
                "Error: Could not parse configuration file '{}': {}",
                config_path.display(),
                err
            ),
        )
    })?;

    // 1. Validate `wiki_paths`
    for (wiki_name, wiki_path) in &mut config.wiki_paths {
        if !wiki_path.exists() {
            return Err(format!(
                "Error: Wiki path '{}' specified for wiki '{}' does not exist.",
                wiki_path.display(),
                wiki_name
            )
            .into());
        }
        if !wiki_path.is_dir() {
            return Err(format!(
                "Error: Wiki path '{}' specified for wiki '{}' is not a directory.",
                wiki_path.display(),
                wiki_name
            )
            .into());
        }
    }

    // 2. Validate `templates_dir`
    if !config.templates_dir.exists() {
        return Err(format!(
            "Error: Templates directory '{}' does not exist.",
            config.templates_dir.display()
        )
        .into());
    }
    if !config.templates_dir.is_dir() {
        return Err(format!(
            "Error: Templates directory '{}' is not a directory.",
            config.templates_dir.display()
        )
        .into());
    }

    // 3. Validate `index_dir`
    if !config.index_dir.exists() {
        return Err(format!(
            "Error: Index directory '{}' does not exist.",
            config.index_dir.display()
        )
        .into());
    }
    if !config.index_dir.is_dir() {
        return Err(format!(
            "Error: Index directory '{}' is not a directory.",
            config.index_dir.display()
        )
        .into());
    }

    // 4. Validate `snippet_length`
    if config.snippet_length == 0 {
        return Err("Error: `snippet_length` in the configuration file cannot be 0.".into());
    }

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not determine home directory."))?;
    let config_path = home_dir.join(".riki").join("config.yaml");

    let config_str = serde_yaml::to_string(config)?;
    fs::write(config_path, config_str)?;

    Ok(())
}
