// src/wiki/import.rs

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use url::Url;
use reqwest::blocking::get;

use crate::config::Config;

pub struct Wiki {
    // ... other fields
}

impl Wiki {
    // ... other methods

    pub fn import_page(
        &self,
        file: &Path,
        format: &str,
        page_name: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();

        let mut file = File::open(file)?;
        file.read_to_string(&mut content)?;

        match format {
            "text" => {
                // Import plain text
                if let Some(page_name) = page_name {
                    self.create_page(page_name, &content, None)?;
                    println!("Imported plain text to page: {}", page_name);
                } else {
                    // If no page name is specified, suggest one based on the file name
                    let suggested_page_name = file
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .trim_end_matches(".txt");
                    println!("Enter a page name (leave blank to use '{}'):", suggested_page_name);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let page_name = input.trim();
                    if page_name.is_empty() {
                        self.create_page(suggested_page_name, &content, None)?;
                        println!(
                            "Imported plain text to page: '{}'",
                            suggested_page_name
                        );
                    } else {
                        self.create_page(page_name, &content, None)?;
                        println!("Imported plain text to page: {}", page_name);
                    }
                }
            }
            "markdown" => {
                // Import Markdown
                if let Some(page_name) = page_name {
                    self.create_page(page_name, &content, None)?;
                    println!("Imported Markdown to page: {}", page_name);
                } else {
                    let suggested_page_name = file
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .trim_end_matches(".md");
                    println!("Enter a page name (leave blank to use '{}'):", suggested_page_name);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let page_name = input.trim();
                    if page_name.is_empty() {
                        self.create_page(suggested_page_name, &content, None)?;
                        println!(
                            "Imported Markdown to page: '{}'",
                            suggested_page_name
                        );
                    } else {
                        self.create_page(page_name, &content, None)?;
                        println!("Imported Markdown to page: {}", page_name);
                    }
                }
            }
            "wikia" => {
                // Import from a wiki service (e.g., Wikia)
                let wiki_url = Url::parse(&content)?;
                let response = get(wiki_url)?;

                if response.status().is_success() {
                    let html_content = response.text()?;

                    // Extract content from HTML (This is a simplified approach)
                    let content_start = html_content.find("<div id=\"mw-content-text\">");
                    let content_end = html_content.find("</div>");
                    if let (Some(start), Some(end)) = (content_start, content_end) {
                        let extracted_content =
                            &html_content[start + 24..end].replace("<br>", "\n"); // Replace <br> with newlines

                        if let Some(page_name) = page_name {
                            self.create_page(page_name, &extracted_content, None)?;
                            println!("Imported from Wikia to page: {}", page_name);
                        } else {
                            // If no page name is provided, suggest one from the URL
                            let page_name = wiki_url
                                .path_segments()
                                .unwrap()
                                .last()
                                .unwrap();
                            self.create_page(page_name, &extracted_content, None)?;
                            println!("Imported from Wikia to page: {}", page_name);
                        }
                    } else {
                        println!("Error: Could not extract content from Wikia page.");
                    }
                } else {
                    println!("Error: Could not retrieve Wikia page.");
                }
            }
            _ => {
                println!("Invalid format. Supported formats: text, markdown, wikia");
            }
        }

        Ok(())
    }
}

// ... (Helper functions: create_or_load_index, get_schema_fields, get_snippet, sanitize_tag, etc.)
