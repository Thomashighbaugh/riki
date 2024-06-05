// THIS IS THE FILE: src/cli/templates.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn templates(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let templates = Templates::new(config).list_templates()?;

    writeln!(
        stdout,
        "{}",
        style::style("Available Templates:").bold().green()
    )?;
    stdout.flush()?;

    for template in templates {
        writeln!(stdout, "{}", style::style(template).dim())?;
        stdout.flush()?;
    }

    Ok(())
}
