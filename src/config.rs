// config.rs
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json;

pub fn configure_wikis(args: &[String]) {
    let config_path = Path::new("./.riki_config.json");

    // Load existing configuration (if any)
    let mut config: serde_json::Value = match config_path.exists() {
        true => {
            let mut file = File::open(config_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap()
        },
        false => serde_json::from_str("{}").unwrap(),
    };

    // Handle arguments for adding or removing directories
    for arg in args {
        let wiki_dir = arg.trim();

        // Check if the directory exists
        if !Path::new(wiki_dir).exists() {
            println!("Error: Directory '{}' does not exist.", wiki_dir);
            continue;
        }

        // Add or remove based on the argument
        if arg.starts_with("+") {
            // Add wiki directory
            let wiki_dir = wiki_dir[1..].to_string(); // Remove the '+'
            let wikis = config["wikis"].as_array_mut().unwrap(); // Fix: remove mut
            wikis.push(serde_json::Value::String(wiki_dir));
        } else if arg.starts_with("-") {
            // Remove wiki directory
            let wiki_dir = wiki_dir[1..].to_string(); // Remove the '-'
            let wikis = config["wikis"].as_array_mut().unwrap();
            wikis.retain(|wiki| {
                wiki.as_str().unwrap() != &wiki_dir 
            });
        } else {
            println!("Invalid argument: '{}'. Use '+' to add or '-' to remove.", arg);
            continue;
        }
    }

    // Save the updated configuration
    let updated_config = serde_json::to_string(&config).unwrap();
    let mut file = File::create(config_path).unwrap();
    file.write_all(updated_config.as_bytes()).unwrap();

    println!("Configuration updated.");
}
