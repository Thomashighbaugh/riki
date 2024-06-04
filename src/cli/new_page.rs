// src/cli/new_page.rs

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

pub fn print_new_page_menu(stdout: &mut io::Stdout, config: &mut Config) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;

    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;

        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("{}", style("New Page").bold().with(Color::Cyan));
        println!("");

        // Print the list of wikis
        println!("{}", style("Select a wiki to create a new page in:").with(Color::Green));
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

            // Prompt for the page name
            println!("{}", style("Enter a name for the new page:").with(Color::Green));
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let page_name = input.trim();

            // Prompt for template selection (optional)
            println!("{}", style("Use a template? (yes/no):").with(Color::Green));
            stdout.execute(cursor::MoveTo(0, 7 + config.wiki_paths.len() as u16 + 1))?;
            stdout.execute(Clear(ClearType::CurrentLine))?;
            print!("{}", style("> ").with(Color::DarkGrey));
            stdout.flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let use_template = input.trim().to_lowercase() == "yes";

            let template_name = if use_template {
                // Print the list of templates
                println!("{}", style("Select a template:").with(Color::Green));
                stdout.execute(cursor::MoveTo(0, 9 + config.wiki_paths.len() as u16 + 1))?;
                let mut templates = Vec::new();
                for (i, (template_name, _)) in config.wiki_paths.iter().enumerate() {
                    templates.push(template_name.to_string());
                    println!("{}", style(format!("  {}. {}", i + 1, template_name)).with(Color::Blue));
                }
                stdout.execute(cursor::MoveTo(0, 9 + config.wiki_paths.len() as u16 + 1 + templates.len() as u16 + 1))?;

                // Get user input
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let choice: usize = input
                    .trim()
                    .parse()
                    .map_err(|_| "Invalid choice. Please enter a number.")?;

                if choice > 0 && choice <= templates.len() {
                    Some(templates[choice - 1].clone())
                } else {
                    None
                }
            } else {
                None
            };

            // Create the page
            let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
            match template_name {
                Some(template_name) => {
                    wiki.create_page_from_template(page_name, &template_name)?;
                    println!(
                        "Page '{}' created successfully in '{}' using template '{}'!",
                        page_name,
                        wiki_path.display(),
                        template_name
                    );
                },
                None => {
                    wiki.create_page_interactive(page_name)?;
                    println!(
                        "Page '{}' created successfully in '{}'!",
                        page_name,
                        wiki_path.display()
                    );
                }
            }
            if let Err(err) = wiki.index_page(page_name.to_string()) {
                eprintln!("Warning: Failed to index page '{}'. Error: {}", page_name, err);
            }
            stdout.execute(cursor::MoveTo(width as u16, 10))?;
            stdout.execute(cursor::MoveTo(0, 12 + config.wiki_paths.len() as u16 + 1 + templates.len() as u16 + 1))?;
            println!("Press Enter to return to main menu.");
            io::stdin().read_line(&mut input)?;
            break;
        } else {
            println!("{}", style("Invalid choice. Please enter a number from the list.").with(Color::Yellow));
            stdout.execute(cursor::MoveTo(width as u16, 10))?;
            stdout.execute(cursor::MoveTo(0, 5 + config.wiki_paths.len() as u16 + 1))?;
        }
    }
    Ok(())
}
