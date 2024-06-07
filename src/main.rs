use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::PrintStyledContent,
    terminal,
};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use tui::{backend::TermionBackend, Terminal};

mod add;
mod config;
mod delete;
mod edit;
mod search;
mod ui;
mod view;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = std::io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = read().unwrap()
        {
            if code == KeyCode::Char('q') {
                break;
            }
            tx.send(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            })
            .unwrap(); // Include all fields
        }
    });

    loop {
        let key = rx.recv().unwrap();
        let args: Vec<String> = std::env::args().collect();
        if key.code == KeyCode::Char('q') {
            break;
        }
        match key.code {
            KeyCode::Char('h') => {
                ui::display_help()?;
            }
            KeyCode::Char('c') => {
                if args.len() > 2 {
                    handle_command(args[1].clone())?;
                }
            }
            _ => {}
        }

        // Clear and redraw the screen
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Riki").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
    }
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}

fn handle_command(command: String) -> Result<(), Box<dyn Error>> {
    match command.as_str() {
        "config" => {
            if let Some(args) = std::env::args().nth(2) {
                config::configure_wikis(&args)?;
            } else {
                println!("Missing wiki URL");
            }
        }
        "search" => {
            if let Some(args) = std::env::args().nth(2) {
                search::search_wikis(&args)?;
            } else {
                println!("Missing search term");
            }
        }
        "add" => {
            if let Some(args) = std::env::args().nth(2) {
                add::add_page(&args)?;
            } else {
                println!("Missing page name");
            }
        }
        "edit" => {
            if let Some(args) = std::env::args().nth(2) {
                edit::edit_page(&args)?;
            } else {
                println!("Missing page name");
            }
        }
        "view" => {
            if let Some(args) = std::env::args().nth(2) {
                view::view_page(&args)?;
            } else {
                println!("Missing page name");
            }
        }
        "delete" => {
            if let Some(args) = std::env::args().nth(2) {
                delete::delete_page(&args)?;
            } else {
                println!("Missing page name");
            }
        }
        _ => println!("Invalid command"),
    }
    Ok(())
}
