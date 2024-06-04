// src/wiki/templates.rs

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct Wiki {
    // ... other fields
}

impl Wiki {
    // ... other methods

    pub fn create_page_from_template(&self, page_name: &str, template_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let template_path = self.templates_dir.join(format!("{}.md", template_name));

        if !template_path.exists() {
            return Err(format!("Error: Template '{}' not found.", template_name).into());
        }

        let template_content = std::fs::read_to_string(template_path)?;
        let page_content = template_content.replace("{{page_name}}", page_name);

        self.create_page(page_name, &page_content, None)
    }

    pub fn list_templates(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Available Templates:");
        for entry in std::fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let template_name = path.file_stem().unwrap().to_str().unwrap();
                println!("- {}", template_name);
            }
        }
        Ok(())
    }
}

// ... (Helper functions: create_or_load_index, get_schema_fields, get_snippet, sanitize_tag, etc.)
