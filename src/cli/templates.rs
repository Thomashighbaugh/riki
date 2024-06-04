// src/cli/templates.rs

use crossterm::{
    cursor,
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand,
};
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::config::Config;
use crate::wiki::Wiki;

pub fn print_templates_menu(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Templates").bold().with(Color::Cyan));
        println!("");

        println!("{}", style("Select an option:").with(Color::Green));
        println!("  1. List Templates");
        stdout.execute(cursor::MoveTo(0, 4))?;
        println!("  q. Back to main menu");
        stdout.execute(cursor::MoveTo(width as u16, 10))?;

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().chars().next() {
            Some('1') => {
                // List Templates
                let wiki = Wiki::new(PathBuf::new(), config.templates_dir.clone(), &config)?;
                wiki.list_templates()?;
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 6))?;
                println!("Press Enter to return to Templates menu.");
                io::stdin().read_line(&mut input)?;
            }
            Some('q') => {
                // Return to main menu
                break;
            }
            _ => {
                println!(
                    "{}",
                    style("Invalid choice. Please enter 1 or q.").with(Color::Yellow)
                );
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 6))?;
            }
        }
    }

    Ok(())
}
