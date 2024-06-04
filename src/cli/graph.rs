// src/cli/graph.rs

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

pub fn print_generate_graph_menu(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Generate Graph").bold().with(Color::Cyan));
        println!("");

        // Print the list of wikis
        println!(
            "{}",
            style("Select a wiki to generate a graph for:").with(Color::Green)
        );
        stdout.execute(cursor::MoveTo(0, 3))?;
        for (i, (wiki_name, _)) in config.wiki_paths.iter().enumerate() {
            println!(
                "{}",
                style(format!("  {}. {}", i + 1, wiki_name)).with(Color::Blue)
            );
        }
        stdout.execute(cursor::MoveTo(0, 3 + config.wiki_paths.len() as u16 + 1))?;

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice: usize = input
            .trim()
            .parse()
            .map_err(|_| "Invalid choice. Please enter a number.")?;

        if choice > 0 && choice <= config.wiki_paths.len() {
            // Get the wiki path
            let wiki_path = config.wiki_paths.values().nth(choice - 1).unwrap().clone();

            // Prompt for output file name (optional)
            println!(
                "{}",
                style("Enter a name for the output file (or press Enter to use the default):")
                    .with(Color::Green)
            );
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let output_file = if input.trim().is_empty() {
                None
            } else {
                Some(PathBuf::from(input.trim()))
            };

            // Generate the graph
            let mut wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
            wiki.generate_graph(output_file)?;

            stdout.execute(cursor::MoveTo(width as u16, 10))?;
            stdout.execute(cursor::MoveTo(0, 7 + config.wiki_paths.len() as u16 + 1))?;
            println!("Press Enter to return to main menu.");
            io::stdin().read_line(&mut input)?;
            break;
        } else {
            println!(
                "{}",
                style("Invalid choice. Please enter a number from the list.").with(Color::Yellow)
            );
            stdout.execute(cursor::MoveTo(width as u16, 10))?;
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
        }
    }
    Ok(())
}
