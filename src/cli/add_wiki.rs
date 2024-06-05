// THIS IS THE FILE: src/cli/add_wiki.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};

use crate::config::{install_default_templates, load_config, save_config, Config};

pub fn add_wiki(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for wiki name
    writeln!(
        stdout,
        "{}",
        style::style("Enter a name for your new wiki: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut wiki_name = String::new();
    io::stdin().read_line(&mut wiki_name)?;
    wiki_name = wiki_name.trim().to_string();

    // Ask for wiki path
    writeln!(
        stdout,
        "{}",
        style::style("Enter the path to your new wiki directory: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut wiki_path_str = String::new();
    io::stdin().read_line(&mut wiki_path_str)?;
    let wiki_path_str = wiki_path_str.trim();
    let wiki_path = PathBuf::from(wiki_path_str);

    // Add the wiki to the configuration
    config.wiki_paths.insert(wiki_name, wiki_path);
    save_config(config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Wiki '{}' added successfully.", wiki_name)).bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}
