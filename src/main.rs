// src/main.rs

mod config;
mod wiki;
mod cli;

use cli::RikiCLI;
use config::Config;

fn main() {
    let cli = RikiCLI::parse();

    // Load configuration
    let config = config::load_config().unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        std::process::exit(1);
    });

    // Process commands and interact with wikis
    if let Err(err) = cli.run(config) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
