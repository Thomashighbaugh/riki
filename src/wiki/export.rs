// src/wiki/export.rs

use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::config::Config;

pub struct Wiki {
    // ... other fields
}

impl Wiki {
    // ... other methods

    pub fn export_page(
        &self,
        page_name: Option<&str>,
        format: &str,
        output_file: Option<&Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_content = String::new();

        if let Some(page_name) = page_name {
            // Export a single page
            let content = self.read_page(page_name, &config)?;
            output_content = content;
        } else {
            // Export all pages (currently just concatenates them)
            for entry in walkdir::WalkDir::new(&self.root_dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            {
                let page_content = fs::read_to_string(entry.path())?;
                output_content.push_str(&page_content);
                output_content.push_str("\n");
            }
        }

        match format {
            "html" => {
                // Convert Markdown to HTML
                let parser = Parser::new_ext(&output_content, Options::all());
                html::push_html(&mut output_content, parser);

                // Write HTML to file
                let output_path = output_file.unwrap_or_else(|| {
                    self.root_dir
                        .join(format!("{}.html", page_name.unwrap_or("all")))
                });
                let mut file = File::create(output_path)?;
                file.write_all(output_content.as_bytes())?;
                println!("Exported to HTML: {}", output_path.display());
            }
            "pdf" => {
                // Export to PDF (requires external tools like Pandoc)
                let output_path = output_file.unwrap_or_else(|| {
                    self.root_dir
                        .join(format!("{}.pdf", page_name.unwrap_or("all")))
                });
                let command = Command::new("pandoc")
                    .arg("--from=markdown")
                    .arg("--to=pdf")
                    .arg(format!("--output={}", output_path.display()))
                    .stdin(std::process::Stdio::piped())
                    .spawn()?;

                if let Some(mut stdin) = command.stdin.take() {
                    stdin.write_all(output_content.as_bytes())?;
                }

                command.wait_with_output()?;
                println!("Exported to PDF: {}", output_path.display());
            }
            "text" => {
                // Write plain text to file
                let output_path = output_file.unwrap_or_else(|| {
                    self.root_dir
                        .join(format!("{}.txt", page_name.unwrap_or("all")))
                });
                let mut file = File::create(output_path)?;
                file.write_all(output_content.as_bytes())?;
                println!("Exported to plain text: {}", output_path.display());
            }
            _ => {
                println!("Invalid format. Supported formats: html, pdf, text");
            }
        }

        Ok(())
    }
}

// ... (Helper functions: create_or_load_index, get_schema_fields, get_snippet, sanitize_tag, etc.)
