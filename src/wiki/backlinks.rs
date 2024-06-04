// src/wiki/backlinks.rs

use std::collections::HashMap;
use regex::Regex;
use crate::config::Config;

pub struct Wiki {
    // ... other fields
}

impl Wiki {
    // ... other methods

    // Update process_wikilinks to populate backlinks
    fn process_wikilinks(&mut self, content: &str, config: &Config, current_page: &str) -> String {
        let re = Regex::new(r#"\\[\\[(?:([^:]+):)?([^\\[\\]]+)\\]\\]"#).unwrap();

        re.replace_all(content, |caps: &Captures| {
            let wiki_name = caps.get(1).map(|m| m.as_str());
            let linked_page = caps.get(2).unwrap().as_str();

            // Track backlinks
            if let Some(wiki_name) = wiki_name {
                // Inter-wiki link (currently no backlinks for inter-wiki)
            } else {
                // Normal wikilink (within the current wiki)
                self.backlinks.entry(linked_page.to_string())
                    .or_insert_with(Vec::new)
                    .push(current_page.to_string());
            }

            // Construct the link based on whether it's an inter-wiki link
            if let Some(wiki_name) = wiki_name {
                // Inter-wiki link
                if let Some(wiki_path) = config.wiki_paths.get(wiki_name) {
                    let linked_page_path = wiki_path.join(format!("{}.md", linked_page));
                    let link_text = if linked_page_path.exists() {
                        format!("[{}]({})", linked_page, linked_page_path.display())
                    } else {
                        format!("[{} (not created yet)]({})", linked_page, linked_page_path.display())
                    };
                    link_text
                } else {
                    format!("[[{}:{} (wiki not found)]]", wiki_name, linked_page)
                }
            } else {
                // Normal wikilink (within the current wiki)
                let linked_page_path = self.get_page_path(linked_page);
                let link_text = if linked_page_path.exists() {
                    format!("[{}]({})", linked_page, linked_page_path.display())
                } else {
                    format!("[{} (not created yet)]({})", linked_page, linked_page_path.display())
                };
                link_text
            }
        }).to_string()
    }

    pub fn read_page(&mut self, page_name: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        // ... (Get page path and content - remains the same) 

        // Reset backlinks for the current page
        self.backlinks.remove(page_name);

        // Process wikilinks and update backlinks
        let processed_content = self.process_wikilinks(&content, config, page_name);

        Ok(processed_content)
    }

    // Add a new method to get backlinks for a page
    pub fn get_backlinks(&self, page_name: &str) -> Vec<String> {
        self.backlinks.get(page_name).cloned().unwrap_or_default()
    }

    // ... (other methods)
}

// ... (Helper functions: create_or_load_index, get_schema_fields, get_snippet, sanitize_tag)
