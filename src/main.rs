// THIS IS THE FILE: src/main.rs

use config::Config;
use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};
use crossterm::{
    cursor,
    style::{self, PrintStyledContent, Print},
    terminal::{self, Clear, ClearType},
};

use riki::cli::{
    add_wiki,
    backlinks,
    delete_page,
    edit_page,
    graph,
    history,
    import_pages,
    list_pages,
    main_menu,
    new_page,
    search,
    tags,
    templates,
    revert_page,
    export_pages,
};
use riki::config::{install_default_templates, load_config, save_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Riki Wiki")
        .version("0.1.0")
        .author("Thomas Leon Highbaugh <me@thomasleonhighbaugh.me>")
        .about("Your Personal Knowledge Wiki Management Solution")
        .arg(
            Arg::new("wiki")
                .short('w')
                .long("wiki")
                .value_name("WIKI_NAME")
                .help("The name of the wiki to use (from config.yaml)")
                .takes_value(true),
        )
        .get_matches();

    let config_path = dirs::config_dir().unwrap().join("riki/config.yaml");
    let mut config = if config_path.exists() {
        load_config(&config_path)
    } else {
        // Install default templates if config.yaml is not found
        let templates_dir = dirs::config_dir().unwrap().join("riki/templates");
        install_default_templates(&templates_dir)?;
        Config::default()
    };

    let wiki_name = matches.value_of("wiki");

    if let Some(wiki_name) = wiki_name {
        if config.wiki_paths.contains_key(wiki_name) {
            config.wiki_paths
                .get_mut(wiki_name)
                .and_then(|wiki_path| {
                    // Update the current wiki
                    config.wiki_paths.insert("main".to_string(), wiki_path.clone());
                    Some(())
                });
        } else {
            println!("Invalid Wiki name.");
            return Ok(());
        }
    }

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    // Clear the terminal
    terminal::Clear(ClearType::All)?;

    // Print welcome message
    let (_width, height) = terminal::size()?;
    writeln!(
        stdout,
        "{}",
        style::style("Welcome to Riki Wiki!").bold().green()
    )?;
    stdout.flush()?;

    cursor::MoveTo(0, height - 1)?;

    main_menu(&mut stdout, &mut config)?;

    // Disable raw mode on exit
    terminal::disable_raw_mode()?;

    Ok(())
}
