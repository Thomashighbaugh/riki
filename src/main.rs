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
                        if args.len() > 2 {
                            handle_command(String::from("config"), &args[2..])?; // Pass args
                        } else {
                            println!("Missing wiki URL");
                        }
                    }
                    1 => {
                        if args.len() > 2 {
                            handle_command(String::from("search"), &args[2..])?; // Pass args
                        } else {
                            println!("Missing search term");
                        }
                    }
                    2 => {
                        if args.len() > 2 {
                            handle_command(String::from("add"), &args[2..])?; // Pass args
                        } else {
                            println!("Missing page name");
                        }
                    }
                    3 => {
                        if args.len() > 2 {
                            handle_command(String::from("edit"), &args[2..])?; // Pass args
                        } else {
                            println!("Missing page name");
                        }
                    }
                    4 => {
                        if args.len() > 2 {
                            handle_command(String::from("view"), &args[2..])?; // Pass args
                        } else {
                            println!("Missing page name");
                        }
                    }
                    5 => {
                        if args.len() > 2 {
                            handle_command(String::from("delete"), &args[2..])?; // Pass args
                        } else {
                            println!("Missing page name");
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
            if let Some(arg) = args.get(0) { // Get the first argument
                config::configure_wikis(arg)?; // Use the argument
            } else {
                println!("Missing wiki URL");
            }
        }
        "search" => {
            if let Some(arg) = args.get(0) { // Get the first argument
                search::search_wikis(arg)?; // Use the argument
            } else {
                println!("Missing search term");
            }
        }
        "add" => {
            if let Some(arg) = args.get(0) { // Get the first argument
                add::add_page(arg)?; // Use the argument
            } else {
                println!("Missing page name");
            }
        }
        "edit" => {
            if let Some(arg) = args.get(0) { // Get the first argument
                edit::edit_page(arg)?; // Use the argument
            } else {
                println!("Missing page name");
            }
        }
        "view" => {
            if let Some(arg) = args.get(0) { // Get the first argument
                view::view_page(arg)?; // Use the argument
            } else {
                println!("Missing page name");
            }
        }
        "delete" => {
            if let Some(arg) = args.get(0) { // Get the first argument
                delete::delete_page(arg)?; // Use the argument
            } else {
                println!("Missing page name");
            }
        }
        _ => println!("Invalid command"),
    }
    Ok(())
}
