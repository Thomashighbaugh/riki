// THIS IS THE FILE: src/cli/export_pages.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn export_pages(
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
    let mut export = Export::new(&wiki);

    // Ask for export format
    writeln!(
        stdout,
        "{}",
        style::style("Choose an export format (html/pdf/txt): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut format = String::new();
    io::stdin().read_line(&mut format)?;
    format = format.trim().to_lowercase();

    match format.as_str() {
        "html" => export.export_html(config)?,
        "pdf" => export.export_pdf(config)?,
        "txt" => export.export_txt(config)?,
        _ => {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid export format.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        }
    }

    writeln!(
        stdout,
        "{}",
        style::style(format!("Pages exported successfully to '{}'.", config.wiki_paths.get("main").unwrap().join("export").display())).bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}
