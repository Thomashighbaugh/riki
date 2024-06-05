// THIS IS THE FILE: src/wiki/templates.rs

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;

pub struct Templates {
    pub templates_dir: PathBuf,
}

impl Templates {
    pub fn new(config: &Config) -> Templates {
        Templates {
            templates_dir: config.templates_dir.clone(),
        }
    }

    pub fn create_page(
        &self,
        page_name: &str,
        template_name: &str,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create the new page file
        let page_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .join(format!("{}.md", page_name));
        let mut file = File::create(page_path)?;

        // Read the template content
        let template_path = self.templates_dir.join(format!("{}.md", template_name));
        let mut template_file = File::open(template_path)?;
        let mut template_content = String::new();
        template_file.read_to_string(&mut template_content)?;

        // Replace placeholders in the template
        let page_content = template_content
            .replace("{{page_name}}", page_name)
            .replace("{{technology}}", "Rust") // Default placeholder value
            .replace("{{technology1}}", "Rust")
            .replace("{{technology2}}", "Python")
            .replace("{{api_name}}", "GitHub API")
            .replace("{{api_description}}", "GitHub's REST API for managing repositories")
            .replace("{{api_url}}", "https://api.github.com")
            .replace("{{api_version}}", "v3")
            .replace("{{api_authentication}}", "OAuth")
            .replace("{{endpoint_name}}", "Repositories")
            .replace("{{method}}", "GET")
            .replace("{{endpoint_url}}", "/repos/{owner}/{repo}")
            .replace("{{endpoint_description}}", "Get information about a repository")
            .replace("{{param_name}}", "owner")
            .replace("{{param_type}}", "String")
            .replace("{{param_description}}", "The owner of the repository")
            .replace("{{response_code}}", "200")
            .replace("{{response_description}}", "OK")
            .replace("{{system_name}}", "Riki Wiki")
            .replace("{{system_description}}", "A command-line wiki system built with Rust")
            .replace("{{language}}", "Bash")
            .replace("{{tool_name}}", "Riki")
            .replace("{{tool_description}}", "A command-line wiki tool for managing notes")
            .replace("{{tool_installation}}", "cargo install riki")
            .replace("{{tool_usage}}", "riki -h")
            .replace("{{api_url}}", "https://api.github.com");

        file.write_all(page_content.as_bytes())?;

        // Create a Git commit for the new page
        let wiki = Wiki::new(config.wiki_paths.get("main").unwrap().clone(), config.templates_dir.clone(), config);
        let mut history = History::new(&wiki)?;
        history.add_page(&page_name, &page_content, config)?;

        Ok(())
    }

    pub fn list_templates(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut templates = Vec::new();

        for entry in std::fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();

            // Only include files ending with ".md"
            if file_name.ends_with(".md") {
                templates.push(file_name.replace(".md", "").to_string());
            }
        }

        Ok(templates)
    }
}
