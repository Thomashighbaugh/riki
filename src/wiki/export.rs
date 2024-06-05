// THIS IS THE FILE: src/wiki/export.rs

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;
use pulldown_cmark::{html, Options, Parser};

pub struct Export {
    pub root_dir: PathBuf,
}

impl Export {
    pub fn new(wiki: &Wiki) -> Export {
        Export {
            root_dir: wiki.root_dir.clone(),
        }
    }

    pub fn export_html(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = self.root_dir.join("export");
        std::fs::create_dir_all(&output_dir)?;

        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    // Construct the output path for the HTML file
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();
                    let output_path = output_dir.join(format!("{}.html", page_name));

                    // Read the page content
                    let content = self.read_page(&page_name, config)?;

                    // Parse the Markdown and generate HTML
                    let parser = Parser::new_ext(&content, Options::all());
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    // Write the HTML to the file
                    let mut file = File::create(output_path)?;
                    file.write_all(html_output.as_bytes())?;
                }
            }
        }

        Ok(())
    }

    pub fn export_pdf(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = self.root_dir.join("export");
        std::fs::create_dir_all(&output_dir)?;

        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    // Construct the output path for the PDF file
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();
                    let output_path = output_dir.join(format!("{}.pdf", page_name));

                    // Read the page content
                    let content = self.read_page(&page_name, config)?;

                    // Convert Markdown to HTML using pulldown-cmark
                    let parser = Parser::new_ext(&content, Options::all());
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    // Use a library like `wkhtmltopdf` to convert HTML to PDF
                    let mut cmd = std::process::Command::new("wkhtmltopdf");
                    cmd.arg("-").arg(output_path);
                    cmd.stdin(std::process::Stdio::piped());
                    let mut child = cmd.spawn()?;
                    let mut stdin = child.stdin.take().unwrap();
                    stdin.write_all(html_output.as_bytes())?;
                    stdin.flush()?;
                    let _ = child.wait()?;
                }
            }
        }

        Ok(())
    }

    pub fn export_txt(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = self.root_dir.join("export");
        std::fs::create_dir_all(&output_dir)?;

        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    // Construct the output path for the text file
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();
                    let output_path = output_dir.join(format!("{}.txt", page_name));

                    // Read the page content
                    let content = self.read_page(&page_name, config)?;

                    // Write the plain text content to the file
                    let mut file = File::create(output_path)?;
                    file.write_all(content.as_bytes())?;
                }
            }
        }

        Ok(())
    }

    fn read_page(&self, page_name: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let wiki = Wiki::new(self.root_dir.clone(), config.templates_dir.clone(), config);
        let content = wiki.read_page(page_name, config)?;

        Ok(content)
    }
}// THIS IS THE FILE: src/wiki/export.rs

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;
use pulldown_cmark::{html, Options, Parser};

pub struct Export {
    pub root_dir: PathBuf,
}

impl Export {
    pub fn new(wiki: &Wiki) -> Export {
        Export {
            root_dir: wiki.root_dir.clone(),
        }
    }

    pub fn export_html(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = self.root_dir.join("export");
        std::fs::create_dir_all(&output_dir)?;

        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    // Construct the output path for the HTML file
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();
                    let output_path = output_dir.join(format!("{}.html", page_name));

                    // Read the page content
                    let content = self.read_page(&page_name, config)?;

                    // Parse the Markdown and generate HTML
                    let parser = Parser::new_ext(&content, Options::all());
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    // Write the HTML to the file
                    let mut file = File::create(output_path)?;
                    file.write_all(html_output.as_bytes())?;
                }
            }
        }

        Ok(())
    }

    pub fn export_pdf(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = self.root_dir.join("export");
        std::fs::create_dir_all(&output_dir)?;

        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    // Construct the output path for the PDF file
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();
                    let output_path = output_dir.join(format!("{}.pdf", page_name));

                    // Read the page content
                    let content = self.read_page(&page_name, config)?;

                    // Convert Markdown to HTML using pulldown-cmark
                    let parser = Parser::new_ext(&content, Options::all());
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    // Use a library like `wkhtmltopdf` to convert HTML to PDF
                    let mut cmd = std::process::Command::new("wkhtmltopdf");
                    cmd.arg("-").arg(output_path);
                    cmd.stdin(std::process::Stdio::piped());
                    let mut child = cmd.spawn()?;
                    let mut stdin = child.stdin.take().unwrap();
                    stdin.write_all(html_output.as_bytes())?;
                    stdin.flush()?;
                    let _ = child.wait()?;
                }
            }
        }

        Ok(())
    }

    pub fn export_txt(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = self.root_dir.join("export");
        std::fs::create_dir_all(&output_dir)?;

        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
                // Only process Markdown files
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "md"
                {
                    // Construct the output path for the text file
                    let page_name = entry
                        .path()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".md", "")
                        .to_string();
                    let output_path = output_dir.join(format!("{}.txt", page_name));

                    // Read the page content
                    let content = self.read_page(&page_name, config)?;

                    // Write the plain text content to the file
                    let mut file = File::create(output_path)?;
                    file.write_all(content.as_bytes())?;
                }
            }
        }

        Ok(())
    }

    fn read_page(&self, page_name: &str, config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let wiki = Wiki::new(self.root_dir.clone(), config.templates_dir.clone(), config);
        let content = wiki.read_page(page_name, config)?;

        Ok(content)
    }
}
