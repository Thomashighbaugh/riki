use std::error::Error;
use std::fs;
use std::path::PathBuf;

use reqwest::blocking; // Import the blocking module specifically
use serde_json::Value;

pub fn configure_wikis(wiki_url: &str) -> Result<(), Box<dyn Error>> {
    let wiki_dir = PathBuf::from(".riki");
    if !wiki_dir.exists() {
        fs::create_dir(wiki_dir.clone())?;
    }

    let wiki_name = wiki_url
        .split('/')
        .last()
        .unwrap()
        .trim_end_matches(".json")
        .to_string();
    let wiki_path = wiki_dir.join(format!("{}.json", wiki_name));

    if wiki_path.exists() {
        println!("Wiki already configured.");
        return Ok(());
    }

    let response = blocking::get(wiki_url)?;
    let wiki_data: Value = serde_json::from_str(&response.text()?)?;

    let wiki_data_json = serde_json::to_string(&wiki_data)?;
    fs::write(wiki_path, wiki_data_json)?;
    println!("Wiki configured successfully.");

    Ok(())
}
