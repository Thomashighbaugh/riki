// THIS IS THE FILE: src/cli/list_pages.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn list_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let pages = wiki.list_pages(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Pages in the current wiki:").bold().green()
    )?;
    stdout.flush()?;

    for page in pages {
        writeln!(stdout, "{}", style::style(page).dim())?;
        stdout.flush()?;
    }

    Ok(())
}
