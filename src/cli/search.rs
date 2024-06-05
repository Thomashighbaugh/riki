// THIS IS THE FILE: src/cli/search.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn search(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for search query
    writeln!(
        stdout,
        "{}",
        style::style("Enter your search query: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut query = String::new();
    io::stdin().read_line(&mut query)?;
    query = query.trim().to_string();

    // Ask for tags
    writeln!(
        stdout,
        "{}",
        style::style("Enter any tags (comma-separated, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut tags = String::new();
    io::stdin().read_line(&mut tags)?;
    let tags: Vec<&str> = tags.trim().split(',').collect();

    // Ask for directories
    writeln!(
        stdout,
        "{}",
        style::style("Enter any directories (comma-separated, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut directories = String::new();
    io::stdin().read_line(&mut directories)?;
    let directories: Vec<&str> = directories.trim().split(',').collect();

    // Ask for date range (optional)
    writeln!(
        stdout,
        "{}",
        style::style("Enter a date range (YYYY-MM-DD, YYYY-MM-DD, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut date_range = String::new();
    io::stdin().read_line(&mut date_range)?;
    let date_range: Vec<&str> = date_range.trim().split(',').collect();

    // Perform the search
    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let search_results = wiki.search(
        &query,
        &tags,
        &directories,
        &date_range,
        config.snippet_length,
    )?;

    // Display the search results
    terminal::Clear(ClearType::All)?;
    writeln!(
        stdout,
        "{}",
        style::style(format!("Search results for '{}':", query)).bold().green()
    )?;
    stdout.flush()?;

    for result in search_results {
        writeln!(
            stdout,
            "{}",
            style::style(format!("{} - {}", result.page_name, result.snippet))
                .dim()
        )?;
        stdout.flush()?;
    }

    Ok(())
}
