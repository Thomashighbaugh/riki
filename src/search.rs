// search.rs
use std::fs;
use std::path::Path;
use std::fs::File; 
use std::io::Read;

pub fn search_wikis(args: &[String]) {
    let search_term = &args[0];

    // Read wiki directories from config
    let config_path = Path::new("./.riki_config.json");
    let mut file = File::open(config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let wiki_dirs: Vec<String> = config["wikis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|wiki| wiki.as_str().unwrap().to_string())
        .collect();

    // Loop through each wiki directory
    for wiki_dir in wiki_dirs {
        let wiki_path = Path::new(&wiki_dir);

        // Iterate through files in the wiki directory
        if let Ok(entries) = fs::read_dir(wiki_path) {
            // Handle the result correctly using `if let`
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let file_name_str = file_name.to_str().unwrap();

                // Check if the file is a Markdown file
                if file_name_str.ends_with(".md") {
                    let file_path = entry.path();
                    let mut file = File::open(file_path).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();

                    // Search for the term in the file contents
                    if contents.contains(search_term) {
                        println!("{}: {}", wiki_dir, file_name_str);
                    }
                }
            }
        } else {
            println!("Error reading directory '{}'.", wiki_path.display());
        }
    }
}
