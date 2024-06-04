// src/cli/add_wiki.rs

use crossterm::{
    cursor,
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand,
};
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::config::{save_config, Config};

pub fn print_add_wiki_menu(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;

        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Add Wiki").bold().with(Color::Cyan));
        println!("");

        println!(
            "{}",
            style("Enter the path to the wiki directory:").with(Color::Green)
        );
        stdout.execute(cursor::MoveTo(0, 3))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        print!("{}", style("> ").with(Color::DarkGrey));
        stdout.flush()?;

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        let path = PathBuf::from(input);

        if !path.exists() {
            println!(
                "{}",
                style(format!(
                    "Error: The path '{}' does not exist.",
                    path.display()
                ))
                .with(Color::Red)
            );
            stdout.execute(cursor::MoveTo(width as u16, 10))?;
            stdout.execute(cursor::MoveTo(0, 5))?;
        } else {
            // Prompt for a wiki name
            println!(
                "{}",
                style("Enter a name for this wiki:").with(Color::Green)
            );
            stdout.execute(cursor::MoveTo(0, 5))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            // Get user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let wiki_name = input.trim();

            if config.wiki_paths.contains_key(wiki_name) {
                println!(
                    "{}",
                    style(format!(
                        "Info: A wiki with the name '{}' already exists.",
                        wiki_name
                    ))
                    .with(Color::Yellow)
                );
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 7))?;
            } else {
                config.wiki_paths.insert(wiki_name.to_string(), path);
                save_config(&config)?;
                println!(
                    "{}",
                    style(format!(
                        "Wiki path '{}' added as '{}' successfully!",
                        path.display(),
                        wiki_name
                    ))
                    .with(Color::Green)
                );
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 9))?;
                println!("Press Enter to return to main menu.");
                io::stdin().read_line(&mut input)?;
                break;
            }
        }
    }

    Ok(())
}
