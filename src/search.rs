use std::error::Error;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

pub fn search_wikis(term: &str) -> Result<(), Box<dyn Error>> {
    let wiki_dir = PathBuf::from(".riki");
    if !wiki_dir.exists() {
        println!("No wikis configured. Run `riki config <wiki_url>` first.");
        return Ok(());
    }

    let wiki_files = fs::read_dir(wiki_dir)?;
    for wiki_file in wiki_files {
        let wiki_file = wiki_file?;
        let wiki_path = wiki_file.path(); // Borrow the path
        let wiki_name = wiki_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".json");
        let wiki_data: Value = serde_json::from_reader(fs::File::open(&wiki_path)?)?; // Pass a reference
        let pages = wiki_data["pages"].as_object().unwrap();

        for (page_name, page_data) in pages {
            if page_data["content"].as_str().unwrap().contains(term) {
                println!("{}: {}", wiki_name, page_name);
            }
        }
    }

    Ok(())
}
