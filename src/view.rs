use std::error::Error;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

pub fn view_page(page_name: &str) -> Result<(), Box<dyn Error>> {
    let wiki_dir = PathBuf::from(".riki");

    if !wiki_dir.exists() {
        return Err("No wikis configured. Run `riki config <wiki_url>` first.".into());
    }

    for wiki_file in fs::read_dir(wiki_dir)? {
        let wiki_file = wiki_file?;
        let wiki_path = wiki_file.path();
        let wiki_name = wiki_path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(|name| name.trim_end_matches(".json"))
            .ok_or("Failed to parse wiki name")?;

        let wiki_data: Value = serde_json::from_reader(fs::File::open(&wiki_path)?)?;
        let pages = wiki_data["pages"].as_object().ok_or("Failed to parse wiki pages")?;

        if let Some(page_data) = pages.get(page_name) {
            let content = page_data["content"].as_str().ok_or("Failed to parse page content")?;
            println!("{}:", wiki_name);
            println!("{}", content);
            return Ok(());
        }
    }

    Err("Page not found".into())
}

