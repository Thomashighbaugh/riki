use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::{json, Value}; // Import the `json!` macro

pub fn add_page(page_name: &str) -> Result<(), Box<dyn Error>> {
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
        let mut wiki_data: Value = serde_json::from_reader(fs::File::open(&wiki_path)?)?;
        let pages = wiki_data["pages"].as_object_mut().unwrap();

        if !pages.contains_key(page_name) {
            println!("{}: {}", wiki_name, page_name);
            println!("Enter new content:");
            let mut new_content = String::new();
            io::stdin().read_line(&mut new_content)?;
            new_content = new_content.trim().to_string();

            pages.insert(page_name.to_string(), json!({ "content": new_content })); // Now it works
            let wiki_data_json = serde_json::to_string(&wiki_data)?;
            fs::write(&wiki_path, wiki_data_json)?;
            println!("Page added.");
            return Ok(());
        }
    }

    println!("Page already exists");
    Ok(())
}
