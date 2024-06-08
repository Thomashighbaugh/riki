use std::error::Error;
use std::sync::mpsc;
use std::thread;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::PrintStyledContent,
    terminal,
};
use tui::{backend::TermionBackend, Terminal};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use std::io::{stdin, stdout, Write};

mod add;
mod config;
mod delete;
mod edit;
mod search;
mod ui;
mod view;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout(); // Get the stdout object
    let backend = TermionBackend::new(stdout.lock()); // Lock the stdout
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Event::Key(KeyEvent { code, modifiers, kind, state }) = read().unwrap() { 
            if code == KeyCode::Char('q') {
                break;
            }
            tx.send(KeyEvent { code, modifiers, kind, state }).unwrap();
        }
    });

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(size);

            let items = vec![
                ListItem::new("Config"),
                ListItem::new("Search"),
                ListItem::new("Add"),
                ListItem::new("Edit"),
                ListItem::new("View"),
                ListItem::new("Delete"),
                ListItem::new("Help"),
                ListItem::new("Quit"),
            ];

            let list = List::new(items)
                .block(Block::default().title("Riki").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::LightGreen))
                .highlight_symbol("> ");

            f.render_stateful_widget(list, chunks[0], &mut state);
        })?;

        let key = rx.recv().unwrap();
        let args: Vec<String> = std::env::args().collect();
        if key.code == KeyCode::Char('q') {
            break;
        }
        match key.code {
            KeyCode::Char('h') => {
                ui::display_help()?;
            }
            KeyCode::Up => {
                state.select(Some(state.selected().unwrap().saturating_sub(1)));
            }
            KeyCode::Down => {
                state.select(Some((state.selected().unwrap() + 1) % 8));
            }
            KeyCode::Enter => {
                let selected = state.selected().unwrap();
                match selected {
                    0 => {
                        // Prompt for Wiki URL
                        println!("Enter Wiki URL:");
                        stdout.flush().unwrap(); // Flush the stdout object
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if !input.is_empty() {
                            config::configure_wikis(input)?;
                        }
                    }
                    1 => {
                        // Prompt for Search Term
                        println!("Enter Search Term:");
                        stdout.flush().unwrap(); // Flush the stdout object
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if !input.is_empty() {
                            search::search_wikis(input)?;
                        }
                    }
                    2 => {
                        // Prompt for Page Name
                        println!("Enter Page Name:");
                        stdout.flush().unwrap(); // Flush the stdout object
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if !input.is_empty() {
                            add::add_page(input)?;
                        }
                    }
                    3 => {
                        // Prompt for Page Name
                        println!("Enter Page Name:");
                        stdout.flush().unwrap(); // Flush the stdout object
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if !input.is_empty() {
                            edit::edit_page(input)?;
                        }
                    }
                    4 => {
                        // Prompt for Page Name
                        println!("Enter Page Name:");
                        stdout.flush().unwrap(); // Flush the stdout object
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if !input.is_empty() {
                            view::view_page(input)?;
                        }
                    }
                    5 => {
                        // Prompt for Page Name
                        println!("Enter Page Name:");
                        stdout.flush().unwrap(); // Flush the stdout object
                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if !input.is_empty() {
                            delete::delete_page(input)?;
                        }
                    }
                    6 => {
                        ui::display_help()?;
                    }
                    7 => {
                        break;
                    }
                    _ => {}
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


fn handle_command(command: String, args: &[String]) -> Result<(), Box<dyn Error>> {
    match command.as_str() {
        "config" => {
            if let Some(arg) = args.get(0) { 
                config::configure_wikis(arg)?; 
            } else {
                println!("Missing wiki URL");
            }
        }
        "search" => {
            if let Some(arg) = args.get(0) { 
                search::search_wikis(arg)?; 
            } else {
                println!("Missing search term");
            }
        }
        "add" => {
            if let Some(arg) = args.get(0) { 
                add::add_page(arg)?; 
            } else {
                println!("Missing page name");
            }
        }
        "edit" => {
            if let Some(arg) = args.get(0) { 
                edit::edit_page(arg)?; 
            } else {
                println!("Missing page name");
            }
        }
        "view" => {
            if let Some(arg) = args.get(0) { 
                view::view_page(arg)?; 
            } else {
                println!("Missing page name");
            }
        }
        "delete" => {
            if let Some(arg) = args.get(0) { 
                delete::delete_page(arg)?; 
            } else {
                println!("Missing page name");
            }
        }
        _ => println!("Invalid command"),
    }
    Ok(())
}
