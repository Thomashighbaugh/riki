use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::Value;

pub fn edit_page(page_name: &str) -> Result<(), Box<dyn Error>> {
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

        if let Some(page_data) = pages.get_mut(page_name) {
            println!("{}: {}", wiki_name, page_name);
            println!("{}", page_data["content"].as_str().unwrap());

            println!("Enter new content (leave blank to cancel):");
            let mut new_content = String::new();
            io::stdin().read_line(&mut new_content)?;
            new_content = new_content.trim().to_string();

            if !new_content.is_empty() {
                page_data["content"] = Value::String(new_content); 
                let wiki_data_json = serde_json::to_string(&wiki_data)?;
                fs::write(&wiki_path, wiki_data_json)?;
                println!("Page updated.");
            }
            return Ok(());
        }
    }

    println!("Page not found");
    Ok(())
}
