// delete.rs// delete.rs
use std::fs;
use std::path::Path;
use std::io;

pub fn delete_page(args: &[String]) {
    if args.len() < 2 {
        println!("Please provide a wiki directory and page name.");
        return;
    }

    let wiki_dir = &args[0];
    let page_name = &args[1];

    let page_path = Path::new(wiki_dir).join(format!("{}.md", page_name));

    if !page_path.exists() {
        println!("Page '{}' not found in '{}'.", page_name, wiki_dir);
        return;
    }

    println!("Are you sure you want to delete page '{}' from '{}'? (y/n)", page_name, wiki_dir);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        if let Err(e) = fs::remove_file(&page_path) {
            println!("Error deleting page: {}", e);
        } else {
            println!("Page '{}' deleted successfully.", page_name);
        }
    } else {
        println!("Deletion cancelled.");
    }
}
