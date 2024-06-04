// src/cli/search.rs

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

pub fn print_search_menu(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Search").bold().with(Color::Cyan));
        println!("");

        println!("{}", style("Enter your search query:").with(Color::Green));
        stdout.execute(cursor::MoveTo(0, 3))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        print!("{}", style("> ").with(Color::DarkGrey));
        stdout.flush()?;

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let query = input.trim();

        // Optional tag filter
        println!("{}", style("Filter by tag? (yes/no):").with(Color::Green));
        stdout.execute(cursor::MoveTo(0, 5))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        print!("{}", style("> ").with(Color::DarkGrey));
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let use_tag = input.trim().to_lowercase() == "yes";

        let tag = if use_tag {
            // Prompt for tag input
            println!("{}", style("Enter the tag:").with(Color::Green));
            stdout.execute(cursor::MoveTo(0, 7))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            Some(input.trim().to_string())
        } else {
            None
        };

        // Optional directory filter
        println!(
            "{}",
            style("Filter by directory? (yes/no):").with(Color::Green)
        );
        stdout.execute(cursor::MoveTo(0, 9))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        print!("{}", style("> ").with(Color::DarkGrey));
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let use_directory = input.trim().to_lowercase() == "yes";

        let directory = if use_directory {
            // Prompt for directory input
            println!(
                "{}",
                style("Enter the directory (relative to wiki root):").with(Color::Green)
            );
            stdout.execute(cursor::MoveTo(0, 11))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            Some(PathBuf::from(input.trim()))
        } else {
            None
        };

        // Optional date range filter
        println!(
            "{}",
            style("Filter by date range? (yes/no):").with(Color::Green)
        );
        stdout.execute(cursor::MoveTo(0, 13))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        print!("{}", style("> ").with(Color::DarkGrey));
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let use_date_range = input.trim().to_lowercase() == "yes";

        let date_from = if use_date_range {
            // Prompt for start date input
            println!(
                "{}",
                style("Enter the start date (YYYY-MM-DD):").with(Color::Green)
            );
            stdout.execute(cursor::MoveTo(0, 15))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            Some(input.trim().to_string())
        } else {
            None
        };

        let date_to = if use_date_range {
            // Prompt for end date input
            println!(
                "{}",
                style("Enter the end date (YYYY-MM-DD):").with(Color::Green)
            );
            stdout.execute(cursor::MoveTo(0, 17))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            Some(input.trim().to_string())
        } else {
            None
        };

        // Perform the search
        for wiki_path in config.wiki_paths.values() {
            let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
            wiki.search_pages(
                query,
                &config,
                tag.as_deref(),
                directory.as_deref(),
                date_from.as_deref(),
                date_to.as_deref(),
            )?;
        }

        stdout.execute(cursor::MoveTo(width as u16, 10))?;
        stdout.execute(cursor::MoveTo(0, 19))?;
        println!("Press Enter to return to main menu.");
        io::stdin().read_line(&mut input)?;
        break;
    }

    Ok(())
}
