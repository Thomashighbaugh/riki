// THIS IS THE FILE: src/cli/revert_page.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn revert_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to revert: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for commit hash
    writeln!(
        stdout,
        "{}",
        style::style("Enter the commit hash to revert to: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut commit_hash = String::new();
    io::stdin().read_line(&mut commit_hash)?;
    commit_hash = commit_hash.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    wiki.revert_page(&page_name, &commit_hash, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Page '{}' reverted successfully.", page_name))
            .bold()
            .green()
    )?;
    stdout.flush()?;

    Ok(())
}
