// THIS IS THE FILE: src/wiki/mod.rs

pub mod page;
pub mod search;
pub mod tags;
pub mod backlinks;
pub mod history;
pub mod templates;
pub mod export;
pub mod import;
pub mod graph;
pub mod utils;

pub use page::*;
pub use search::*;
pub use tags::*;
pub use backlinks::*;
pub use history::*;
pub use templates::*;
pub use export::*;
pub use import::*;
pub use graph::*;
pub use utils::*;

use std::path::{Path, PathBuf};

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
}

impl Wiki {
    pub fn new(root_dir: PathBuf, templates_dir: PathBuf, _config: &Config) -> Wiki {
        Wiki {
            root_dir,
            templates_dir,
        }
    }

    pub fn get_page_path(&self, page_name: &str) -> PathBuf {
        self.root_dir.join(format!("{}.md", page_name))
    }

    pub fn read_page(
        &self,
        page_name: &str,
        _config: &Config,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if page_path.exists() {
            let contents = std::fs::read_to_string(page_path)?;
            Ok(contents)
        } else {
            Err(format!("Page not found: {}", page_name).into())
        }
    }

    pub fn create_page(
        &mut self,
        page_name: &str,
        contents: &str,
        template: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);
        let mut file = File::create(page_path)?;

        if let Some(template) = template {
            // Use a template to create the new page
            let template_content = self.read_page(template, &Config::default())?;
            file.write_all(template_content.as_bytes())?;
        } else {
            // Create a blank page
            file.write_all(contents.as_bytes())?;
        }

        Ok(())
    }

    pub fn list_pages(&self, _config: &Config) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut pages = Vec::new();

        // Iterate through all Markdown files in the wiki directory
        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();

                    pages.push(page_name);
                }
            }
        }

        Ok(pages)
    }
}
