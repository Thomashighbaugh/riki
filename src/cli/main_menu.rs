// src/cli/main_menu.rs

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand,
};
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn main_menu(config: &mut Config) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    // Main loop for TUI menu
    loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;

        // Print the main menu
        print_menu(&mut stdout, &config)?;

        // Get user input
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('1') => {
                    // Add Wiki
                    print_add_wiki_menu(&mut stdout, config)?;
                }
                KeyCode::Char('2') => {
                    // New Page
                    print_new_page_menu(&mut stdout, config)?;
                }
                KeyCode::Char('3') => {
                    // Search
                    print_search_menu(&mut stdout, config)?;
                }
                KeyCode::Char('4') => {
                    // Edit
                    print_edit_page_menu(&mut stdout, config)?;
                }
                KeyCode::Char('5') => {
                    // Delete
                    print_delete_page_menu(&mut stdout, config)?;
                }
                KeyCode::Char('6') => {
                    // List
                    print_list_pages_menu(&mut stdout, config)?;
                }
                KeyCode::Char('7') => {
                    // Templates
                    print_templates_menu(&mut stdout, config)?;
                }
                KeyCode::Char('8') => {
                    // Tags
                    print_tags_menu(&mut stdout, config)?;
                }
                KeyCode::Char('9') => {
                    // Backlinks
                    print_backlinks_menu(&mut stdout, config)?;
                }
                KeyCode::Char('0') => {
                    // Graph
                    print_generate_graph_menu(&mut stdout, config)?;
                }
                KeyCode::Char('q') => {
                    // Quit
                    break;
                }
                _ => {}
            }
        }
    }

    // Disable raw mode
    terminal::disable_raw_mode()?;

    Ok(())
}

fn print_menu(stdout: &mut io::Stdout, config: &Config) -> Result<(), Box<dyn Error>> {
    let (width, _) = terminal::size()?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    println!(
        "{}",
        style("Riki - Your Command-line Wiki")
            .bold()
            .with(Color::Cyan)
    );
    println!("");
    println!("  1. Add Wiki");
    println!("  2. New Page");
    println!("  3. Search");
    println!("  4. Edit");
    println!("  5. Delete");
    println!("  6. List Pages");
    println!("  7. Templates");
    println!("  8. Tags");
    println!("  9. Backlinks");
    println!("  0. Generate Graph");
    println!("");
    println!("  q. Quit");
    println!("");
    println!(
        "{}",
        style("------------------------------------").with(Color::DarkGrey)
    );
    stdout.execute(cursor::MoveTo(width - 10, 10))?;
    println!("{}", style("Current Wiki:",).with(Color::Green));
    for (wiki_name, _) in config.wiki_paths.iter() {
        println!("{}", style(format!("  - {}", wiki_name)).with(Color::Blue));
    }
    Ok(())
}
