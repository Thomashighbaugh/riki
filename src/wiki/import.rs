use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

pub struct Import {
    pub root_dir: PathBuf,
}

impl Import {
    pub fn new(wiki: &Wiki) -> Import {
        Import {
            root_dir: wiki.root_dir.clone(),
        }
    }

    pub fn import_from_files(
        &self,
        import_dir: &PathBuf,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in walkdir::WalkDir::new(import_dir) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let file_path = entry.path();
                    let file_name = file_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    let mut file = File::open(file_path)?;
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;

                    // Determine page name based on file name
                    let page_name = if file_name.ends_with(".md") {
                        file_name.replace(".md", "")
                    } else {
                        file_name
                    };

                    // Create a new page in the wiki
                    let wiki = Wiki::new(self.root_dir.clone(), config.templates_dir.clone(), config);
                    let mut wiki = wiki;
                    wiki.create_page(&page_name, &content, None)?;
                }
            }
        }

        Ok(())
    }

    pub fn import_from_wiki(&self, url: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        // Simple import, just extract the page content
        let response = reqwest::blocking::get(url)?;

        if response.status().is_success() {
            let content = response.text()?;

            // Extract page title and content
            let re = Regex::new(r"<title>(.*?)</title>").unwrap();
            let page_name = if let Some(capture) = re.captures(&content) {
                capture[1].to_string()
            } else {
                // Default page name if title not found
                "ImportedPage".to_string()
            };

            // Extract page content from the wiki page
            let re = Regex::new(r"<div class=\"mw-parser-output\">(.*?)</div>").unwrap();
            let extracted_content = if let Some(capture) = re.captures(&content) {
                capture[1].to_string()
            } else {
                // Fallback to the whole response content
                content
            };

            // Create a new page in the wiki
            let wiki = Wiki::new(self.root_dir.clone(), config.templates_dir.clone(), config);
            let mut wiki = wiki;
            wiki.create_page(&page_name, &extracted_content, None)?;

            Ok(())
        } else {
            Err(format!("Failed to import from Wiki: {}", url).into())
        }
    }

    pub fn import_from_url(&self, url: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(url)?;

        if response.status().is_success() {
            let content = response.text()?;

            // Determine the page name based on the URL
            let page_name = url
                .split('/')
                .last()
                .unwrap_or_default()
                .replace(".md", "")
                .replace(".txt", "");

            // Create a new page in the wiki
            let wiki = Wiki::new(self.root_dir.clone(), config.templates_dir.clone(), config);
            let mut wiki = wiki;
            wiki.create_page(&page_name, &content, None)?;

            Ok(())
        } else {
            Err(format!("Failed to import from URL: {}", url).into())
        }
    }
}
