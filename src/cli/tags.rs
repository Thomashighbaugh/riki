// THIS IS THE FILE: src/cli/tags.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn tags(
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
    let tags = wiki.list_tags(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Tags in the current wiki:").bold().green()
    )?;
    stdout.flush()?;

    for tag in tags {
        writeln!(stdout, "{}", style::style(tag).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn add_tag(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to add a tag to: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for tag name
    writeln!(stdout, "{}", style::style("Enter the tag name: ").bold().dim())?;
    stdout.flush()?;

    let mut tag_name = String::new();
    io::stdin().read_line(&mut tag_name)?;
    tag_name = tag_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let mut wiki = wiki;
    wiki.add_tag(&page_name, &tag_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Tag '{}' added to page '{}' successfully.", tag_name, page_name))
            .bold()
            .green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn remove_tag(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to remove a tag from: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for tag name
    writeln!(stdout, "{}", style::style("Enter the tag name: ").bold().dim())?;
    stdout.flush()?;

    let mut tag_name = String::new();
    io::stdin().read_line(&mut tag_name)?;
    tag_name = tag_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let mut wiki = wiki;
    wiki.remove_tag(&page_name, &tag_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Tag '{}' removed from page '{}' successfully.", tag_name, page_name))
            .bold()
            .green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn list_tags_for_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to list tags for: ").bold().dim()
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
    let tags = wiki.list_tags_for_page(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Tags for page '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for tag in tags {
        writeln!(stdout, "{}", style::style(tag).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn list_pages_with_tag(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for tag name
    writeln!(stdout, "{}", style::style("Enter the tag name: ").bold().dim())?;
    stdout.flush()?;

    let mut tag_name = String::new();
    io::stdin().read_line(&mut tag_name)?;
    tag_name = tag_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let pages = wiki.list_pages_with_tag(&tag_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Pages with tag '{}':", tag_name)).bold().green()
    )?;
    stdout.flush()?;

    for page in pages {
        writeln!(stdout, "{}", style::style(page).dim())?;
        stdout.flush()?;
    }

    Ok(())
}
