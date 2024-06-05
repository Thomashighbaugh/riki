// THIS IS THE FILE: src/cli/backlinks.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn backlinks(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to view backlinks for: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let backlinks = wiki.get_backlinks(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Backlinks for '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for backlink in backlinks {
        writeln!(stdout, "{}", style::style(backlink).dim())?;
        stdout.flush()?;
    }

    Ok(())
}// THIS IS THE FILE: src/cli/backlinks.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn backlinks(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to view backlinks for: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let backlinks = wiki.get_backlinks(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Backlinks for '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for backlink in backlinks {
        writeln!(stdout, "{}", style::style(backlink).dim())?;
        stdout.flush()?;
    }

    Ok(())
}
