// THIS IS THE FILE: src/wiki/backlinks.rs

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;

pub struct Backlinks {
    pub backlinks: HashMap<String, Vec<String>>,
}

impl Backlinks {
    pub fn new() -> Backlinks {
        Backlinks {
            backlinks: HashMap::new(),
        }
    }

    pub fn update(&mut self, wiki: &mut Wiki, config: &Config) {
        self.backlinks.clear();

        // Iterate through all wiki pages
        for entry in walkdir::WalkDir::new(wiki.root_dir.clone()) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    let page_name = entry
                        .path()
                        .strip_prefix(&wiki.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();

                    // Read the page content
                    let contents =
                        fs::read_to_string(entry.path()).expect("Failed to read file content");

                    // Extract linked pages
                    let linked_pages =
                        Self::extract_linked_pages(&contents, &wiki.root_dir, config);

                    // Update backlinks for linked pages
                    for linked_page in linked_pages {
                        self.backlinks
                            .entry(linked_page.to_string())
                            .or_insert_with(Vec::new)
                            .push(page_name.clone());
                    }
                }
            }
        }
    }

    /// Extracts linked pages from a Markdown page
    fn extract_linked_pages(
        contents: &str,
        root_dir: &PathBuf,
        config: &Config,
    ) -> Vec<String> {
        let mut linked_pages = Vec::new();

        for capture in regex::Regex::new(r"\[\[(.*?)\]\]").unwrap().captures_iter(contents) {
            let linked_page = capture[1].to_string();

            // Check if it's a valid internal link or a cross-wiki link
            if linked_page.contains(":") {
                let parts: Vec<&str> = linked_page.split(":").collect();
                if parts.len() == 2 {
                    let wiki_name = parts[0].trim();
                    let page_name = parts[1].trim();

                    if let Some(wiki_path) = config.wiki_paths.get(wiki_name) {
                        linked_pages.push(wiki_path.join(format!("{}.md", page_name)).to_str().unwrap().to_string());
                    }
                }
            } else {
                // Internal link within the wiki
                linked_pages.push(
                    root_dir
                        .join(format!("{}.md", linked_page))
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }
        }

        linked_pages
    }

    pub fn get_backlinks(&self, page_name: &str) -> Vec<String> {
        self.backlinks
            .get(page_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn remove_backlinks(&mut self, page_name: &str) {
        self.backlinks.remove(page_name);
    }
}
