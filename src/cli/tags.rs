// src/cli/tags.rs

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

pub fn print_tags_menu(stdout: &mut io::Stdout, config: &mut Config) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Tags").bold().with(Color::Cyan));
        println!("");

        println!("{}", style("Select an option:").with(Color::Green));
        println!("  1. List All Tags");
        println!("  2. Modify Tags on a Page");
        stdout.execute(cursor::MoveTo(0, 5))?;
        println!("  q. Back to main menu");
        stdout.execute(cursor::MoveTo(width as u16, 10))?;

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().chars().next() {
            Some('1') => {
                // List Tags
                let mut wiki = Wiki::new(PathBuf::new(), config.templates_dir.clone(), &config)?;
                wiki.list_all_tags()?;
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 7))?;
                println!("Press Enter to return to Tags menu.");
                io::stdin().read_line(&mut input)?;
            }
            Some('2') => {
                // Modify Tags
                print_modify_tags_menu(stdout, config)?;
            }
            Some('q') => {
                // Return to main menu
                break;
            }
            _ => {
                println!(
                    "{}",
                    style("Invalid choice. Please enter 1, 2, or q.").with(Color::Yellow)
                );
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 7))?;
            }
        }
    }

    Ok(())
}

fn print_modify_tags_menu(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Modify Tags").bold().with(Color::Cyan));
        println!("");

        // Print the list of wikis
        println!(
            "{}",
            style("Select a wiki to modify tags for:").with(Color::Green)
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

            // Print the list of pages in the wiki
            println!(
                "{}",
                style("Select a page to modify tags for:").with(Color::Green)
            );
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
            let mut pages = Vec::new();
            for (i, entry) in walkdir::WalkDir::new(&wiki_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
                .enumerate()
            {
                let page_name = entry.path().file_stem().unwrap().to_str().unwrap();
                pages.push(page_name.to_string());
                println!(
                    "{}",
                    style(format!("  {}. {}", i + 1, page_name)).with(Color::Blue)
                );
            }
            stdout.execute(cursor::MoveTo(
                0,
                5 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1,
            ))?;

            // Get user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let choice: usize = input
                .trim()
                .parse()
                .map_err(|_| "Invalid choice. Please enter a number.")?;

            if choice > 0 && choice <= pages.len() {
                let page_name = pages[choice - 1].clone();

                // Prompt for tags to add
                println!(
                    "{}",
                    style("Enter tags to add (comma-separated, or press Enter to skip):")
                        .with(Color::Green)
                );
                stdout.execute(cursor::MoveTo(
                    0,
                    7 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1,
                ))?;
                stdout.execute(Clear(ClearType::CurrentLine))?;
                print!("{}", style("> ").with(Color::DarkGrey));
                stdout.flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let add_tags: Vec<String> = input
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                // Prompt for tags to remove
                println!(
                    "{}",
                    style("Enter tags to remove (comma-separated, or press Enter to skip):")
                        .with(Color::Green)
                );
                stdout.execute(cursor::MoveTo(
                    0,
                    9 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1,
                ))?;
                stdout.execute(Clear(ClearType::CurrentLine))?;
                print!("{}", style("> ").with(Color::DarkGrey));
                stdout.flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let remove_tags: Vec<String> = input
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                // Modify tags
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.modify_page_tags(&page_name, &add_tags, &remove_tags)?;

                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(
                    0,
                    11 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1,
                ))?;
                println!("Press Enter to return to Tags menu.");
                io::stdin().read_line(&mut input)?;
                break;
            } else {
                println!(
                    "{}",
                    style("Invalid choice. Please enter a number from the list.")
                        .with(Color::Yellow)
                );
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 7 + config.wiki_paths.len() as u16 + 1))?;
            }
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
