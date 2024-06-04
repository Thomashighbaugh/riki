// src/cli/edit_page.rs

use std::io::{self, Write};
use std::error::Error;
use std::path::PathBuf;
use crossterm::{
    cursor,
    style::{self, Color, PrintStyledContent, Print},
    terminal::{self, ClearType, Clear, size},
    ExecutableCommand,
};

use crate::config::{Config};
use crate::wiki::Wiki;

pub fn print_edit_page_menu(stdout: &mut io::Stdout, config: &mut Config) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("Edit Page").bold().with(Color::Cyan));
        println!("");

        // Print the list of wikis
        println!("{}", style("Select a wiki to edit a page in:").with(Color::Green));
        stdout.execute(cursor::MoveTo(0, 3))?;
        for (i, (wiki_name, _)) in config.wiki_paths.iter().enumerate() {
            println!("{}", style(format!("  {}. {}", i + 1, wiki_name)).with(Color::Blue));
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
            let wiki_path = config
                .wiki_paths
                .values()
                .nth(choice - 1)
                .unwrap()
                .clone();

            // Print the list of pages in the wiki
            println!("{}", style("Select a page to edit:").with(Color::Green));
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
            let mut pages = Vec::new();
            for (i, entry) in walkdir::WalkDir::new(&wiki_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map_or(false, |ext| ext == "md")
                })
                .enumerate()
            {
                let page_name = entry.path().file_stem().unwrap().to_str().unwrap();
                pages.push(page_name.to_string());
                println!("{}", style(format!("  {}. {}", i + 1, page_name)).with(Color::Blue));
            }
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1))?;

            // Get user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let choice: usize = input
                .trim()
                .parse()
                .map_err(|_| "Invalid choice. Please enter a number.")?;

            if choice > 0 && choice <= pages.len() {
                let page_name = pages[choice - 1].clone();

                // Edit the page
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.edit_page(&page_name, &config)?;

                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 7 + config.wiki_paths.len() as u16 + 1 + pages.len() as u16 + 1))?;
                println!("Press Enter to return to main menu.");
                io::stdin().read_line(&mut input)?;
                break;
            } else {
                println!("{}", style("Invalid choice. Please enter a number from the list.").with(Color::Yellow));
                stdout.execute(cursor::MoveTo(width as u16, 10))?;
                stdout.execute(cursor::MoveTo(0, 7 + config.wiki_paths.len() as u16 + 1))?;
            }
        } else {
            println!("{}", style("Invalid choice. Please enter a number from the list.").with(Color::Yellow));
            stdout.execute(cursor::MoveTo(width as u16, 10))?;
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
        }
    }
    Ok(())
}
