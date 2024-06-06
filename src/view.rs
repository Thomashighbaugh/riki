// view.rs
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

pub fn view_page(args: &[String]) {
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

    let mut file = File::open(&page_path).unwrap_or_else(|err| {
        println!("Error opening page '{}': {}", page_name, err);
        process::exit(1); // Exit if the file opening fails
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Basic Markdown Parsing (for headings, lists, and bold/italic)
    let mut in_list = false; // Flag to track if we are currently inside a list
    for line in contents.lines() {
        if line.starts_with("#") {
            let level = line.chars().take_while(|c| *c == '#').count();
            let heading = line.trim_start_matches('#').trim();
            println!("{}{}", "#".repeat(level), heading);
        } else if line.starts_with("- ") && !in_list {
            // Start of a list
            println!("- {}", line.trim_start_matches("- ").trim());
            in_list = true;
        } else if line.starts_with("  - ") && in_list {
            // Continued list
            println!("  - {}", line.trim_start_matches("  - ").trim());
        } else if line.starts_with("*") && line.ends_with("*") {
            // Bold text
            println!("**{}**", &line[1..line.len() - 1]);
        } else if line.starts_with("_") && line.ends_with("_") {
            // Italic text
            println!("_{}_", &line[1..line.len() - 1]);
        } else if line.is_empty() && in_list {
            // End of list
            in_list = false;
            println!("");
        } else {
            println!("{}", line);
        }
    }
}
