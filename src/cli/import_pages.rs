// THIS IS THE FILE: src/cli/import_pages.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn import_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for import source
    writeln!(
        stdout,
        "{}",
        style::style("Choose an import source (files/wiki/url): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut source = String::new();
    io::stdin().read_line(&mut source)?;
    source = source.trim().to_lowercase();

    match source.as_str() {
        "files" => {
            // Ask for import directory
            writeln!(
                stdout,
                "{}",
                style::style("Enter the path to the directory to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut import_dir_str = String::new();
            io::stdin().read_line(&mut import_dir_str)?;
            let import_dir_str = import_dir_str.trim();
            let import_dir = PathBuf::from(import_dir_str);

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_files(&import_dir, config)?;
            writeln!(
                stdout,
                "{}",
                style::style("Pages imported successfully.").bold().green()
            )?;
            stdout.flush()?;
        }
        "wiki" => {
            // Ask for wiki URL
            writeln!(
                stdout,
                "{}",
                style::style("Enter the Wiki URL to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut wiki_url = String::new();
            io::stdin().read_line(&mut wiki_url)?;
            wiki_url = wiki_url.trim().to_string();

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_wiki(&wiki_url, config)?;
            writeln!(
                stdout,
                "{}",
                style::style(format!("Page imported successfully from '{}'.", wiki_url))
                    .bold()
                    .green()
            )?;
            stdout.flush()?;
        }
        "url" => {
            // Ask for URL
            writeln!(
                stdout,
                "{}",
                style::style("Enter the URL to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut url = String::new();
            io::stdin().read_line(&mut url)?;
            url = url.trim().to_string();

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_url(&url, config)?;
            writeln!(
                stdout,
                "{}",
                style::style(format!("Page imported successfully from '{}'.", url))
                    .bold()
                    .green()
            )?;
            stdout.flush()?;
        }
        _ => {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid import source.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        }
    }

    Ok(())
}
