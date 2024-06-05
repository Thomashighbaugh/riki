// THIS IS THE FILE: src/wiki/tags.rs

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;

#[derive(Debug)]
pub struct Tags {
    pub tags: HashMap<String, HashSet<String>>,
}

impl Tags {
    pub fn new() -> Tags {
        Tags {
            tags: HashMap::new(),
        }
    }

    pub fn update(&mut self, wiki: &Wiki, config: &Config) {
        self.tags.clear();

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

                    // Extract tags from the page content
                    let tags = Self::extract_tags(&contents);

                    // Update the tags map
                    for tag in tags {
                        self.tags
                            .entry(tag)
                            .or_insert_with(HashSet::new)
                            .insert(page_name.clone());
                    }
                }
            }
        }
    }

    /// Extracts tags from a Markdown page
    fn extract_tags(contents: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // Match tags using a regex pattern
        for capture in regex::Regex::new(r"tags:\s*-\s*(.*?)\s*").unwrap().captures_iter(contents) {
            let tag = capture[1].trim().to_string();
            tags.push(tag);
        }

        tags
    }

    pub fn add_tag(&mut self, page_name: &str, tag_name: &str) {
        self.tags
            .entry(tag_name.to_string())
            .or_insert_with(HashSet::new)
            .insert(page_name.to_string());
    }

    pub fn remove_tag(&mut self, page_name: &str, tag_name: &str) {
        if let Some(pages) = self.tags.get_mut(tag_name) {
            pages.remove(page_name);
        }
    }

    pub fn list_tags(&self) -> Vec<String> {
        self.tags.keys().cloned().collect()
    }

    pub fn list_tags_for_page(&self, page_name: &str) -> Vec<String> {
        self.tags
            .iter()
            .filter_map(|(tag, pages)| {
                if pages.contains(page_name) {
                    Some(tag.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn list_pages_with_tag(&self, tag_name: &str) -> Vec<String> {
        self.tags
            .get(tag_name)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    pub fn save(&self, wiki: &Wiki, config: &Config) {
        let tags_path = config.index_dir.join("tags.yaml");
        let tags_str = serde_yaml::to_string(self).unwrap();
        fs::write(tags_path, tags_str).unwrap();
    }
}
