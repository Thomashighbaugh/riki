// THIS IS THE FILE: src/wiki/page.rs

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;

use crate::config::Config;
use crate::wiki::{
    backlinks::Backlinks,
    history::History,
    search::Wiki as SearchWiki,
    tags::Tags,
};
use crate::wiki::utils::get_snippet;

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

    pub fn delete_page(&mut self, page_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);
        if page_path.exists() {
            fs::remove_file(page_path)?;
            Ok(())
        } else {
            Err(format!("Page not found: {}", page_name).into())
        }
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

    pub fn get_backlinks(
        &self,
        page_name: &str,
        config: &Config,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut backlinks = Backlinks::new();
        backlinks.update(self, config);
        Ok(backlinks.get_backlinks(page_name))
    }

    pub fn add_tag(
        &mut self,
        page_name: &str,
        tag_name: &str,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut tags = Tags::new();
        tags.update(self, config);
        tags.add_tag(page_name, tag_name);
        tags.save(self, config)?;

        Ok(())
    }

    pub fn remove_tag(
        &mut self,
        page_name: &str,
        tag_name: &str,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut tags = Tags::new();
        tags.update(self, config);
        tags.remove_tag(page_name, tag_name);
        tags.save(self, config)?;

        Ok(())
    }

    pub fn list_tags(&self, config: &Config) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut tags = Tags::new();
        tags.update(self, config);
        Ok(tags.list_tags())
    }

    pub fn list_tags_for_page(
        &self,
        page_name: &str,
        config: &Config,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut tags = Tags::new();
        tags.update(self, config);
        Ok(tags.list_tags_for_page(page_name))
    }

    pub fn list_pages_with_tag(
        &self,
        tag_name: &str,
        config: &Config,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut tags = Tags::new();
        tags.update(self, config);
        Ok(tags.list_pages_with_tag(tag_name))
    }

    pub fn get_page_history(
        &self,
        page_name: &str,
        config: &Config,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let history = History::new(self)?;
        Ok(history.get_page_history(page_name, config)?)
    }

    pub fn get_last_modified_date(
        &self,
        page_name: &str,
        config: &Config,
    ) -> Result<NaiveDateTime, Box<dyn std::error::Error>> {
        let history = History::new(self)?;
        Ok(history.get_last_modified_date(page_name, config)?)
    }

    pub fn revert_page(
        &self,
        page_name: &str,
        commit_hash: &str,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let history = History::new(self)?;
        history.revert_page(page_name, commit_hash, config)?;
        Ok(())
    }

    pub fn delete_page(&mut self, page_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);
        if page_path.exists() {
            fs::remove_file(page_path)?;
            Ok(())
        } else {
            Err(format!("Page not found: {}", page_name).into())
        }
    }

    pub fn search(
        &self,
        query: &str,
        tags: &[&str],
        directories: &[&str],
        date_range: &[&str],
        snippet_length: usize,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut search_wiki = SearchWiki::new(self.root_dir.clone(), self.templates_dir.clone(), &Config::default());
        let search_results = search_wiki.search(query, tags, directories, date_range)?;

        let mut results = Vec::new();
        for result in search_results {
            let snippet = get_snippet(&result.content, snippet_length)?;
            results.push(SearchResult {
                page_name: result.page_name,
                snippet,
            });
        }

        Ok(results)
    }
}

#[derive(Debug)]
pub struct SearchResult {
    pub page_name: String,
    pub snippet: String,
}
