// THIS IS THE FILE: src/cli/new_page.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn new_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter a name for your new page: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for template name
    let templates = Templates::new(config).list_templates()?;
    if templates.is_empty() {
        writeln!(
            stdout,
            "{}",
            style::style("No templates available. Creating a blank page.").bold()
        )?;
        stdout.flush()?;
    } else {
        writeln!(
            stdout,
            "{}",
            style::style("Choose a template (enter 'q' to create a blank page): ").bold().dim()
        )?;
        stdout.flush()?;

        for (i, template) in templates.iter().enumerate() {
            writeln!(
                stdout,
                "{}",
                style::style(format!("{}: {}", i + 1, template)).dim()
            )?;
            stdout.flush()?;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let template = if input == "q" {
            None
        } else if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= templates.len() {
                Some(templates[index - 1].as_str())
            } else {
                writeln!(
                    stdout,
                    "{}",
                    style::style("Invalid template choice.").bold().red()
                )?;
                stdout.flush()?;
                return Ok(());
            }
        } else {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid input.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        };

        // Create the new page
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut wiki = wiki;
        wiki.create_page(&page_name, "", template)?;

        // Open the page in the default editor
        if let Some(editor) = &config.editor {
            let mut cmd = std::process::Command::new(editor);
            cmd.arg(wiki.get_page_path(&page_name));
            let _ = cmd.spawn();
        }
    }

    Ok(())
}
