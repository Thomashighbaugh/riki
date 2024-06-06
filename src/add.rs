// add.rs
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

pub fn add_page(args: &[String]) {
    let wiki_dir = &args[0];
    let page_name = &args[1];

    // Check for existing page
    let page_path = Path::new(wiki_dir).join(format!("{}.md", page_name));
    if page_path.exists() {
        println!(
            "A page with the name '{}' already exists in this wiki. Overwrite? (y/n)",
            page_name
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("Page creation cancelled.");
            return;
        }
    }

    // Create new page file
    let mut file = File::create(&page_path).unwrap_or_else(|err| {
        println!("Error creating page '{}': {}", page_name, err);
        process::exit(1); // Exit if the file creation fails
    });

    // Optionally use a template or create a blank file
    println!("Choose a template or create a blank page:");
    println!("1. Basic template");
    println!("2. Blank page");
    println!("Enter your choice (1 or 2):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => {
            // Use basic template
            let template = "# My New Page\n\nThis is a basic template for your new page. You can start editing right away.";
            file.write_all(template.as_bytes()).unwrap();
            println!("Page '{}' created using basic template.", page_name);
        }
        2 => {
            // Create a blank page
            println!("Page '{}' created as blank.", page_name);
        }
        _ => {
            println!("Invalid choice. Page creation cancelled.");
        }
    }
}
