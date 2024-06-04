// src/cli/delete_page.rs

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

pub fn print_delete_page_menu(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Delete Page").bold().with(Color::Cyan));
        println!("");

        // Print the list of wikis
        println!(
            "{}",
            style("Select a wiki to delete a page from:").with(Color::Green)
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
            println!("{}", style("Select a page to delete:").with(Color::Green));
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

                // Delete the page
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                println!(
                    "Are you sure you want to delete page '{}'? (yes/no)",
                    page_name
                );
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "yes" {
                    if wiki.delete_page(&page_name)? {
                        println!(
                            "Page '{}' deleted successfully from '{}'!",
                            page_name,
                            wiki_path.display()
                        );
                    }
                } else {
                    println!("Deletion cancelled.");
                }

                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(
                    0,
                    7 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1,
                ))?;
                println!("Press Enter to return to main menu.");
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
