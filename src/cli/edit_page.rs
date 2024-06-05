// THIS IS THE FILE: src/cli/edit_page.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn edit_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to edit: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Open the page in the default editor
    if let Some(editor) = &config.editor {
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut cmd = std::process::Command::new(editor);
        cmd.arg(wiki.get_page_path(&page_name));
        let _ = cmd.spawn();
    }

    Ok(())
}
