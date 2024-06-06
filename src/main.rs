// main.rs
use std::env;
use std::process;

mod config;
mod search;
mod add;
mod edit;
mod view;
mod delete;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help(); // Print help if no command is provided
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "config" => {
            if args.len() < 3 {
                println!("Please provide at least one wiki directory to configure.");
                process::exit(1);
            }
            config::configure_wikis(&args[2..])
        },
        "search" => {
            if args.len() < 2 {
                println!("Please provide a search term.");
                process::exit(1);
            }
            search::search_wikis(&args[2..])
        },
        "add" => {
            if args.len() < 3 {
                println!("Please provide a wiki directory and page name.");
                process::exit(1);
            }
            add::add_page(&args[2..])
        },
        "edit" => {
            if args.len() < 3 {
                println!("Please provide a wiki directory and page name.");
                process::exit(1);
            }
            edit::edit_page(&args[2..])
        },
        "view" => {
            if args.len() < 3 {
                println!("Please provide a wiki directory and page name.");
                process::exit(1);
            }
            view::view_page(&args[2..])
        },
        "delete" => {
            if args.len() < 3 {
                println!("Please provide a wiki directory and page name.");
                process::exit(1);
            }
            delete::delete_page(&args[2..])
        },
        _ => {
            println!("Invalid command. Use 'config', 'search', 'add', 'edit', 'view', or 'delete'.");
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  config  - Configure wiki directories");
    println!("  search  - Search wikis for a term");
    println!("  add     - Add a new page to a wiki");
    println!("  edit    - Edit an existing page");
    println!("  view    - View the content of a page");
    println!("  delete  - Delete a page from a wiki");
}
