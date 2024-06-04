// src/wiki/tags.rs

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use memmap::{Mmap, Protection};
use tantivy::query::{Query, TermQuery, BooleanQuery, Term}, schema::Field, collector::TopDocs, DocAddress;
use tantivy::Term as TantivyTerm;
use tantivy::IndexRecordOption;
use crate::config::Config;
use crossterm::{
    style::{self, Color, PrintStyledContent},
};

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub index: Index,
    pub backlinks: HashMap<String, Vec<String>>,
    pub tag_cache: HashMap<PathBuf, (Vec<String>, std::time::SystemTime)>,
}

impl Wiki {
    // ... (Other methods)

    pub fn list_all_tags(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut all_tags = HashSet::new();

        for entry in walkdir::WalkDir::new(&self.root_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let path = entry.path();
            let metadata = fs::metadata(path)?;
            let last_modified = metadata.modified()?;

            if let Some((cached_tags, cached_modified)) = self.tag_cache.get(path) {
                if last_modified <= *cached_modified {
                    all_tags.extend(cached_tags.iter().cloned());
                    continue; // Skip reading the file
                }
            }

            let content = fs::read_to_string(path)?;
            let tags = self.extract_tags(&content);
            all_tags.extend(tags.clone());
            self.tag_cache.insert(path.to_path_buf(), (tags, last_modified)); // Update cache
        }

        println!("Tags in use:");
        for tag in all_tags {
            println!("- {}", tag);
        }

        Ok(())
    }

    pub fn modify_page_tags(
        &self,
        page_name: &str,
        add_tags: &[String],
        remove_tags: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if !page_path.exists() {
            return Err(format!("Error: Page '{}' not found.", page_name).into());
        }

        let mut updated_content = self.update_page_tags(
            &page_path,
            add_tags,
            remove_tags,
        )?;

        fs::write(&page_path, updated_content)?;

        println!("Tags updated for page '{}'.", page_name);

        // Re-index the page to update the search index with the new tags
        if let Err(err) = self.index_page(page_name) {
            eprintln!("Warning: Failed to re-index page '{}'. Error: {}", page_name, err);
        }

        Ok(())
    }

    fn extract_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let mut in_frontmatter = false;
        for line in content.lines() {
            if line == "---" {
                in_frontmatter = !in_frontmatter;
                if !in_frontmatter {
                    break; // Exit loop after frontmatter ends
                }
            } else if in_frontmatter && line.starts_with("  - ") {
                let tag = line.strip_prefix("  - ").unwrap().to_string();
                tags.push(tag);
            }
        }
        tags
    }

    fn update_page_tags(
        &self,
        page_path: &Path,
        add_tags: &[String],
        remove_tags: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 1. Read the file content
        let mut file = File::open(&page_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // 2. Find the headmatter boundaries
        let frontmatter_start = content.find("---\n").ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "Error: Page does not have valid YAML frontmatter.",
            )
        })?;
        let frontmatter_end = content
            .find("---\n", frontmatter_start + 4)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Error: Page does not have a closing YAML frontmatter.",
                )
            })?;

        // 3. Extract existing tags
        let mut existing_tags = Vec::new();
        for line in &content[frontmatter_start + 4..frontmatter_end].lines() {
            if line.starts_with("  - ") {
                existing_tags.push(line.strip_prefix("  - ").unwrap().to_string());
            }
        }

        // 4. Update tags
        existing_tags.extend(add_tags.iter().cloned());
        existing_tags.retain(|tag| !remove_tags.contains(tag));

        // 5. Create a memory mapping of the file
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&page_path)
            .map_err(|err| {
                Error::new(
                    ErrorKind::Other,
                    format!(
                        "Error: Could not open page file '{}' for writing: {}",
                        page_path.display(),
                        err
                    )
                )
            })?;
        let mmap = unsafe { Mmap::map(&file, Protection::ReadWrite) }
            .map_err(|err| {
                Error::new(
                    ErrorKind::Other,
                    format!("Error: Could not memory map page file '{}': {}", page_path.display(), err)
                )
            })?;

        // 6. Modify the headmatter in the memory mapping
        let mut headmatter_offset = frontmatter_start as usize;
        let mut updated_headmatter = format!(
            "---\n{}",
            existing_tags
                .iter()
                .map(|tag| format!("  - {}", tag))
                .collect::<Vec<String>>()
                .join("\n")
        );

        // Write the updated headmatter to the memory mapping
        for byte in updated_headmatter.bytes() {
            mmap[headmatter_offset] = byte;
            headmatter_offset += 1;
        }

        // 7. Return the updated content
        Ok(content) // content is the string with the updated headmatter
    }

    fn sanitize_tag(tag: &str) -> String {
        tag.chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c, // Allow these characters
                _ => '_', // Replace invalid characters with '_'
            })
            .collect::<String>()
    }

    // ... (Other methods) 
}

fn get_schema_fields(schema: &Schema) -> (Field, Field, Field, Field) {
    let name = schema.get_field("name").expect("Field 'name' not found in schema");
    let content = schema
        .get_field("content")
        .expect("Field 'content' not found in schema");
    let tags = schema.get_field("tags").expect("Field 'tags' not found in schema");
    let modified_date = schema.get_field("content").expect("Field 'content' not found in schema");
    (name, content, tags, modified_date)
}

// ... (Other functions:  index_page, get_snippet, etc.)
