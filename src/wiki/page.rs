// src/wiki/page.rs

use std::path::{PathBuf, Path};
use std::fs::{File, create_dir_all, remove_file, OpenOptions};
use std::io::{Write, Read, Error, ErrorKind};

use crate::config::Config;

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub index: Index,
    pub backlinks: HashMap<String, Vec<String>>,
    pub tag_cache: HashMap<PathBuf, (Vec<String>, std::time::SystemTime)>,
}

impl Wiki {
    // ... (Other methods)

    pub fn create_page(&self, page_name: &str, content: &str, tags: Option<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if page_path.exists() {
            return Err(format!("Error: A page with the name '{}' already exists in this wiki. Choose a different name.", page_name).into());
        }

        if let Some(parent_dir) = page_path.parent() {
            if let Err(err) = create_dir_all(parent_dir) {
                return Err(format!("Error: Could not create directories for page '{}': {}", page_name, err).into());
            }
        }

        // Handle errors during file creation or writing
        let mut file = File::create(page_path).map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Error: Could not create page file '{}': {}",
                    page_path.display(),
                    err
                )
            )
        })?;

        // Write tags as YAML headmatter
        if let Some(tags) = tags {
            writeln!(file, "---\n")?;
            writeln!(file, "tags:\n")?;
            for tag in tags {
                writeln!(file, "  - {}", tag)?;
            }
            writeln!(file, "---\n")?;
        }

        writeln!(file, "{}", content)?;

        Ok(())
    }

    pub fn read_page(&mut self, page_name: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if !page_path.exists() {
            return Err(format!("Error: Page '{}' not found in this wiki.", page_name).into());
        }

        // Handle errors during file opening
        let mut file = File::open(page_path).map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Error: Could not open page file '{}': {}",
                    page_path.display(),
                    err
                )
            )
        })?;
        let mut contents = String::new();

        // Handle errors during file reading
        file.read_to_string(&mut contents).map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Error: Could not read content from page file '{}': {}",
                    page_path.display(),
                    err
                )
            )
        })?;

        // Reset backlinks for the current page
        self.backlinks.remove(page_name);

        // Process wikilinks and update backlinks
        let processed_content = self.process_wikilinks(&contents, config, page_name);

        Ok(processed_content)
    }

    pub fn update_page(&self, page_name: &str, new_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if !page_path.exists() {
            return Err(format!("Error: Page '{}' not found.", page_name).into());
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(page_path)?;

        write!(file, "{}", new_content)?;
        Ok(())
    }

    pub fn delete_page(&self, page_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if !page_path.exists() {
            println!("Info: Page '{}' does not exist.", page_name);
            return Ok(false); // Indicate that the page was not deleted because it didn't exist.
        }

        remove_file(page_path)?;
        Ok(true)
    }

    pub fn create_page_from_template(&self, page_name: &str, template_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let template_path = self.templates_dir.join(format!("{}.md", template_name));

        if !template_path.exists() {
            return Err(format!("Error: Template '{}' not found.", template_name).into());
        }

        let template_content = std::fs::read_to_string(template_path)?;
        let page_content = template_content.replace("{{page_name}}", page_name);

        self.create_page(page_name, &page_content, None)
    }

    pub fn create_page_interactive(&self, page_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Enter the initial content for '{}' (leave empty to create a blank page):", page_name);

        let mut content = String::new();
        std::io::stdin().read_line(&mut content)?;
        let mut tags = Vec::new();
        
        // Ask the user if they want to add tags
        println!("Do you want to add tags? (yes/no)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    
        if input.trim().to_lowercase() == "yes" {
            loop {
                println!("Enter a tag (or press Enter to finish):");
                let mut tag_input = String::new();
                std::io::stdin().read_line(&mut tag_input)?;
    
                let tag = tag_input.trim();
                if tag.is_empty() {
                    break;
                }
                tags.push(tag.to_string());
            }
        }
        self.create_page(page_name, &content.trim(), Some(tags))
    }

    fn get_page_path(&self, page_name: &str) -> PathBuf {
        let mut page_path = self.root_dir.clone();
        let sanitized_name = page_name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c,
                _ => '_',
            })
            .collect::<String>();

        page_path.push(format!("{}.md", sanitized_name));
        page_path
    }

    // ... (Other methods for search, backlinks, tags, history, export, import, etc.) 
}

// ... (Helper functions: create_or_load_index, get_schema_fields, get_snippet, sanitize_tag)
