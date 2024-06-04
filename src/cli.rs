// src/cli.rs

use clap::Subcommand;
use crate::config::{Config, save_config, install_default_templates};
use std::path::PathBuf;
use crate::wiki::Wiki;
use std::{
    io::{self, Write},
    process::Command,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RikiCLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new wiki directory to the configuration
    Add { path: PathBuf },
    /// Create a new page in a wiki
    New {
        wiki_path: PathBuf,
        page_name: String,
        #[arg(short, long, value_name = "TEMPLATE_NAME")]
        template: Option<String>,
    },
    /// Search for content within your wikis
    Search {
        query: String,
        #[arg(short, long, value_name = "TAG", help = "Search for pages with this tag")]
        tag: Option<String>,
        #[arg(short, long, value_name = "DIRECTORY", help = "Search within this directory")]
        directory: Option<PathBuf>,
        #[arg(short, long, value_name = "DATE", help = "Search for pages modified on or after this date (YYYY-MM-DD)")]
        date_from: Option<String>,
        #[arg(short, long, value_name = "DATE", help = "Search for pages modified on or before this date (YYYY-MM-DD)")]
        date_to: Option<String>,
    },
    /// Edit a page using your default editor
    Edit {
        wiki_path: PathBuf,
        page_name: String,
    },
    /// Delete a page
    Delete {
        wiki_path: PathBuf,
        page_name: String,
    },
    /// List all pages in a wiki
    List { wiki_path: PathBuf },
    /// Manage templates
    Templates {
        #[command(subcommand)]
        command: TemplateCommands,
    },
    /// Manage tags
    Tags {
        #[command(subcommand)]
        command: TagCommands,
    },
    /// Display backlinks for a page
    Backlinks {
        wiki_path: PathBuf,
        page_name: String,
    },
    /// Generate a graph of wiki links
    Graph {
        wiki_path: PathBuf,
        #[arg(short, long, value_name = "OUTPUT_FILE", help = "Output file path (optional)")]
        output_file: Option<PathBuf>,
    },
    /// Export a page or all pages to a different format
    Export {
        wiki_path: PathBuf,
        page_name: Option<String>,
        #[arg(short, long, value_name = "FORMAT", help = "Output format (html, pdf, text)", default_value_t = String::from("html"))]
        format: String,
        #[arg(short, long, value_name = "OUTPUT_FILE", help = "Output file path (optional)")]
        output_file: Option<PathBuf>,
    },
    /// Import notes from other formats into a wiki
    Import {
        wiki_path: PathBuf,
        #[arg(short, long, value_name = "FILE", help = "Path to the file to import")]
        file: PathBuf,
        #[arg(short, long, value_name = "FORMAT", help = "Format of the file to import (text, markdown, wikia)", default_value_t = String::from("markdown"))]
        format: String,
        #[arg(short, long, value_name = "PAGE_NAME", help = "Name of the page to import into (optional)")]
        page_name: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// List available templates
    List,
}

#[derive(Subcommand, Debug)]
pub enum TagCommands {
    /// List all tags in use
    List,
    /// Add or remove tags from a page
    Modify {
        wiki_path: PathBuf,
        page_name: String,
        #[arg(short, long, value_name = "TAG")]
        add: Vec<String>,
        #[arg(short, long, value_name = "TAG")]
        remove: Vec<String>,
    },
}

impl RikiCLI {
    pub fn run(&self, mut config: Config) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Tags { command } => {
                match command {
                    TagCommands::List => {
                        let wiki = Wiki::new(PathBuf::new(), config.templates_dir.clone(), &config)?;
                        wiki.list_all_tags()?;
                    }
                    TagCommands::Modify {
                        wiki_path,
                        page_name,
                        add,
                        remove,
                    } => {
                        let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                        wiki.modify_page_tags(page_name, add, remove)?;
                    }
                }
            }
            Commands::Export {
                wiki_path,
                page_name,
                format,
                output_file,
            } => {
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.export_page(page_name.as_deref(), format, output_file.as_deref())?;
            }
            Commands::Import {
                wiki_path,
                file,
                format,
                page_name,
            } => {
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.import_page(file, format, page_name.as_deref())?;
            }
            Commands::Graph { wiki_path, output_file } => {
                let mut wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.generate_graph(output_file.clone())?;
            }
            Commands::Backlinks { wiki_path, page_name } => {
                let mut wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                // Call read_page to process wikilinks and populate backlinks:
                wiki.read_page(page_name, &config)?;

                let backlinks = wiki.get_backlinks(page_name);
                if backlinks.is_empty() {
                    println!("No backlinks found for '{}'.", page_name);
                } else {
                    println!("Backlinks for '{}':", page_name);
                    for backlink in backlinks {
                        println!("- {}", backlink);
                    }
                }
            }
            Commands::Search {
                query,
                tag,
                directory,
                date_from,
                date_to,
            } => {
                for wiki_path in config.wiki_paths.values() {
                    let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                    wiki.search_pages(
                        query,
                        &config,
                        tag.as_deref(),
                        directory.as_deref(),
                        date_from.as_deref(),
                        date_to.as_deref(),
                    )?;
                }
            }
            Commands::Add { path } => {
                if !path.exists() {
                    return Err(format!("Error: The path '{}' does not exist.", path.display()).into());
                }
                // Prompt for a wiki name
                println!("Enter a name for this wiki:");
                let mut wiki_name = String::new();
                io::stdin().read_line(&mut wiki_name)?;
                let wiki_name = wiki_name.trim();

                if config.wiki_paths.contains_key(wiki_name) {
                    println!("Info: A wiki with the name '{}' already exists.", wiki_name);
                } else {
                    config.wiki_paths.insert(wiki_name.to_string(), path.clone());
                    save_config(&config)?;
                    println!("Wiki path '{}' added as '{}' successfully!", path.display(), wiki_name);
                }
            }
            Commands::New {
                wiki_path,
                page_name,
                template,
            } => {
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;

                match template {
                    Some(template_name) => {
                        wiki.create_page_from_template(page_name, template_name)?;
                        println!(
                            "Page '{}' created successfully in '{}' using template '{}'!",
                            page_name,
                            wiki_path.display(),
                            template_name
                        );
                    }
                    None => {
                        wiki.create_page_interactive(page_name)?;
                        println!(
                            "Page '{}' created successfully in '{}'!",
                            page_name,
                            wiki_path.display()
                        );
                    }
                }
                // Now index the page
                if let Err(err) = wiki.index_page(page_name) {
                    eprintln!("Warning: Failed to index page '{}'. Error: {}", page_name, err);
                }
            }
            Commands::Edit { wiki_path, page_name } => {
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.edit_page(page_name, &config)?;
            }
            Commands::Delete { wiki_path, page_name } => {
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;

                // Prompt for confirmation
                println!("Are you sure you want to delete page '{}'? (yes/no)", page_name);
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "yes" {
                    if wiki.delete_page(page_name)? {
                        println!(
                            "Page '{}' deleted successfully from '{}'!",
                            page_name,
                            wiki_path.display()
                        );
                    }
                } else {
                    println!("Deletion cancelled.");
                }
            }
            Commands::List { wiki_path } => {
                let wiki = Wiki::new(wiki_path.clone(), config.templates_dir.clone(), &config)?;
                wiki.list_pages()?;
            }
            Commands::Templates { command } => {
                match command {
                    TemplateCommands::List => {
                        let wiki = Wiki::new(PathBuf::new(), config.templates_dir.clone(), &config)?;
                        wiki.list_templates()?;
                    }
                }
            }
        }
        Ok(())
    }
}
