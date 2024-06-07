use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::Value;

pub fn delete_page(page_name: &str) -> Result<(), Box<dyn Error>> {
    let wiki_dir = PathBuf::from(".riki");
    if !wiki_dir.exists() {
        println!("No wikis configured. Run `riki config <wiki_url>` first.");
        return Ok(());
    }

    let wiki_files = fs::read_dir(wiki_dir)?;
    for wiki_file in wiki_files {
        let wiki_file = wiki_file?;
        let wiki_path = wiki_file.path();
        let wiki_name = wiki_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".json");

        // Make `wiki_data` mutable
        let mut wiki_data: Value = serde_json::from_reader(fs::File::open(&wiki_path)?)?; 

        // Get a mutable reference to the `pages` field
        let pages = wiki_data["pages"].as_object_mut().unwrap();

        if pages.contains_key(page_name) {
            println!("{}: {}", wiki_name, page_name);
            println!("Are you sure you want to delete this page? (y/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = input.trim().to_string();

            if input.to_lowercase() == "y" {
                pages.remove(page_name); // Now we can modify the `pages` map
                let wiki_data_json = serde_json::to_string(&wiki_data)?;
                fs::write(&wiki_path, wiki_data_json)?;
                println!("Page deleted.");
            } else {
                println!("Deletion canceled.");
            }
            return Ok(());
        }
    }

    println!("Page not found");
    Ok(())
}
