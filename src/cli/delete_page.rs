// THIS IS THE FILE: src/cli/delete_page.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn delete_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to delete: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Confirm deletion
    writeln!(
        stdout,
        "{}",
        style::style(format!("Are you sure you want to delete '{}'? (y/n): ", page_name))
            .bold()
            .red()
    )?;
    stdout.flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    if input == "y" {
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut wiki = wiki;
        wiki.delete_page(&page_name)?;
        writeln!(
            stdout,
            "{}",
            style::style(format!("Page '{}' deleted successfully.", page_name))
                .bold()
                .green()
        )?;
        stdout.flush()?;
    } else {
        writeln!(
            stdout,
            "{}",
            style::style("Deletion cancelled.").bold()
        )?;
        stdout.flush()?;
    }

    Ok(())
}
