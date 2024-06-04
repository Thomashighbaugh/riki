// wiki.rs

use std::path::{PathBuf, Path};
use std::fs::{File, create_dir_all, remove_file, OpenOptions};
use std::io::{Write, Read, Error, ErrorKind};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, TEXT};
use tantivy::{doc, Directory, Index, ReloadPolicy, Term as TantivyTerm};
use tantivy::{query::{Query, FuzzyTermQuery, BooleanQuery, Term, BooleanClause, TermRange}, schema::Field, collector::TopDocs, DocAddress};
use std::collections::{HashSet, HashMap};
use std::process::Command;
use chrono::{DateTime, Utc, NaiveDate, Datelike};
use std::str::FromStr;
use std::time::Duration;
use memmap::{Mmap, Protection};
use std::convert::TryInto;
use pulldown_cmark::{html, Options, Parser};
use url::Url;
use reqwest::blocking::get;
use git2::{Repository, Status, Commit, ObjectType, IndexAddOption, Tree, Signature};
use crossterm::{
    style::{self, Color, PrintStyledContent},
};

use crate::config::Config;

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub index: Index,
    pub backlinks: HashMap<String, Vec<String>>,
    pub tag_cache: HashMap<PathBuf, (Vec<String>, std::time::SystemTime)>,
}

impl Wiki {
    pub fn new(root_dir: PathBuf, templates_dir: PathBuf, config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let index = create_or_load_index(&config.index_dir)?;

        Ok(Wiki {
            root_dir,
            templates_dir,
            index,
            backlinks: HashMap::new(),
            tag_cache: HashMap::new(),
        })
    }

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

    pub fn search_pages(
        &self,
        query_str: &str,
        config: &Config,
        tag: Option<&str>,
        directory: Option<&Path>,
        date_from: Option<&str>,
        date_to: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let reader = self.index
            .reader()
            .map_err(|err| {
                Error::new(
                    ErrorKind::Other,
                    format!("Error: Could not open the search index reader: {}", err)
                )
            })?
            .reload(ReloadPolicy::OnCommit)
            .map_err(|err| {
                Error::new(
                    ErrorKind::Other,
                    format!("Error: Could not reload the search index: {}", err)
                )
            })?;
        let searcher = reader.searcher();

        let schema = self.index.schema();
        let (name, content, tags, modified_date) = get_schema_fields(&schema);

        let mut query = BooleanQuery::new(true); // Start with an empty BooleanQuery

        // 1. Add fuzzy term queries for content search
        for term in query_str.split_whitespace() {
            let fuzzy_term = FuzzyTermQuery::new(
                Term::from_field_text(content, term),
                2,
                true,
            );
            query.add(Box::new(fuzzy_term) as Box<dyn Query>, BooleanClause::Should);
        }

        // 2. Add tag queries (if provided)
        if let Some(tag) = tag {
            let tag_query = TermQuery::new(
                Term::from_field_text(tags, tag),
                IndexRecordOption::Basic,
            );
            query.add(Box::new(tag_query) as Box<dyn Query>, BooleanClause::Must);
        }

        // 3. Add directory filter (if provided)
        if let Some(directory) = directory {
            // Assume directory is relative to the wiki root
            let directory_path = self.root_dir.join(directory);
            let directory_query = TermQuery::new(
                Term::from_field_text(name, directory_path.to_str().unwrap()),
                IndexRecordOption::Basic,
            );
            query.add(Box::new(directory_query) as Box<dyn Query>, BooleanClause::Must);
        }

        // 4. Add date range filters (if provided)
        if let (Some(date_from), Some(date_to)) = (date_from, date_to) {
            let date_from_parsed = NaiveDate::from_str(date_from)?;
            let date_to_parsed = NaiveDate::from_str(date_to)?;

            // Get timestamp ranges for filtering by modified date
            let timestamp_from = date_from_parsed
                .and_hms(0, 0, 0)
                .and_utc()
                .timestamp_millis() as u64;
            let timestamp_to = date_to_parsed
                .and_hms(23, 59, 59)
                .and_utc()
                .timestamp_millis() as u64;

            let date_filter = TermRange::new(
                content,
                TantivyTerm::from_field_text(content, ×tamp_from.to_string()),
                TantivyTerm::from_field_text(content, ×tamp_to.to_string()),
                false,
                false,
            );

            query.add(Box::new(date_filter) as Box<dyn Query>, BooleanClause::Must);
        }

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        if top_docs.is_empty() {
            println!(
                "{} {}",
                style("No matching pages found.").with(Color::Yellow),
                style("(Try a different query or adjust the fuzziness.)")
                    .with(Color::DarkGrey)
            );
        } else {
            println!("Search Results (Ranked):");
            for (_score, doc_address) in top_docs {
                let retrieved_doc = searcher.doc(doc_address)?;
                let page_name = retrieved_doc.get_first(name).unwrap().text().unwrap();
                let content_snippet = get_snippet(
                    &schema,
                    &retrieved_doc,
                    content,
                    &query,
                    &searcher,
                    config.snippet_length,
                )?;

                let styled_page_name = if page_name.contains(':') {
                    let parts: Vec<&str> = page_name.split(':').collect();
                    format!(
                        "{}:{}",
                        style(parts[0]).with(Color::Blue),
                        style(parts[1]).bold()
                    )
                } else {
                    style(page_name).bold()
                };

                println!(
                    "{} {} {} {}",
                    style("-").with(Color::Green),
                    styled_page_name,
                    style(":").with(Color::DarkGrey),
                    content_snippet
                );
            }
        }
        Ok(())
    }

    pub fn edit_page(&self, page_name: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        if !page_path.exists() {
            return Err(format!("Error: Page '{}' not found.", page_name).into());
        }

        let editor_command = config
            .editor
            .clone()
            .unwrap_or_else(|| {
                // Try to get the default system editor if not configured
                std::env::var("EDITOR").unwrap_or_else(|_| {
                    // If $EDITOR is not set, provide a suggestion
                    println!("Warning: No editor configured. Consider setting the 'editor' option in '.riki.yaml' or the $EDITOR environment variable.");
                    "nano".to_string() // You can change the default suggestion here
                })
            });

        // Open the page with the determined editor
        match Command::new(&editor_command).arg(&page_path).status() {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error opening editor '{}': {:?}", editor_command, err);
                println!("Consider editing the file manually: {}", page_path.display());
                Err(format!("Failed to open editor for {}", page_name).into())
            }
        }
    }

    pub fn list_pages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let styled_path = style(self.root_dir.display()).with(Color::Cyan);
        println!("Pages in {}:", styled_path);

        for entry in walkdir::WalkDir::new(&self.root_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .map_or(false, |ext| ext == "md")
            })
        {
            let path = entry.path();
            let page_name = path.file_stem().unwrap().to_str().unwrap();

            // Style the page name (e.g., green)
            let styled_name = style(format!("- {}", page_name)).with(Color::Green);
            println!("{}", styled_name);
        }

        Ok(())
    }

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
                let output_path = output_file
                    .unwrap_or_else(|| self.root_dir.join(format!("{}.html", page_name.unwrap_or("all"))));
                let mut file = File::create(output_path)?;
                file.write_all(output_content.as_bytes())?;
                println!("Exported to HTML: {}", output_path.display());
            }
            "pdf" => {
                // Export to PDF (requires external tools like Pandoc)
                let output_path = output_file
                    .unwrap_or_else(|| self.root_dir.join(format!("{}.pdf", page_name.unwrap_or("all"))));
                let command = Command::new("pandoc")
                    .arg("--from=markdown")
                    .arg("--to=pdf")
                    .arg(format!(\"--output={}\", output_path.display()))
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
                let output_path = output_file
                    .unwrap_or_else(|| self.root_dir.join(format!("{}.txt", page_name.unwrap_or("all"))));
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

    pub fn import_page(
        &self,
        file: &Path,
        format: &str,
        page_name: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();

        let mut file = File::open(file)?;
        file.read_to_string(&mut content)?;

        match format {
            "text" => {
                // Import plain text
                if let Some(page_name) = page_name {
                    self.create_page(page_name, &content, None)?;
                    println!("Imported plain text to page: {}", page_name);
                } else {
                    // If no page name is specified, suggest one based on the file name
                    let suggested_page_name = file
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .trim_end_matches(".txt");
                    println!("Enter a page name (leave blank to use '{}'):", suggested_page_name);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let page_name = input.trim();
                    if page_name.is_empty() {
                        self.create_page(suggested_page_name, &content, None)?;
                        println!(
                            "Imported plain text to page: '{}'",
                            suggested_page_name
                        );
                    } else {
                        self.create_page(page_name, &content, None)?;
                        println!("Imported plain text to page: {}", page_name);
                    }
                }
            }
            "markdown" => {
                // Import Markdown
                if let Some(page_name) = page_name {
                    self.create_page(page_name, &content, None)?;
                    println!("Imported Markdown to page: {}", page_name);
                } else {
                    let suggested_page_name = file
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .trim_end_matches(".md");
                    println!("Enter a page name (leave blank to use '{}'):", suggested_page_name);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let page_name = input.trim();
                    if page_name.is_empty() {
                        self.create_page(suggested_page_name, &content, None)?;
                        println!(
                            "Imported Markdown to page: '{}'",
                            suggested_page_name
                        );
                    } else {
                        self.create_page(page_name, &content, None)?;
                        println!("Imported Markdown to page: {}", page_name);
                    }
                }
            }
            "wikia" => {
                // Import from a wiki service (e.g., Wikia)
                let wiki_url = Url::parse(&content)?;
                let response = get(wiki_url)?;

                if response.status().is_success() {
                    let html_content = response.text()?;

                    // Extract content from HTML (This is a simplified approach)
                    let content_start = html_content.find("<div id=\"mw-content-text\">");
                    let content_end = html_content.find("</div>");
                    if let (Some(start), Some(end)) = (content_start, content_end) {
                        let extracted_content =
                            &html_content[start + 24..end].replace("<br>", "\n"); // Replace <br> with newlines

                        if let Some(page_name) = page_name {
                            self.create_page(page_name, &extracted_content, None)?;
                            println!("Imported from Wikia to page: {}", page_name);
                        } else {
                            // If no page name is provided, suggest one from the URL
                            let page_name = wiki_url
                                .path_segments()
                                .unwrap()
                                .last()
                                .unwrap();
                            self.create_page(page_name, &extracted_content, None)?;
                            println!("Imported from Wikia to page: {}", page_name);
                        }
                    } else {
                        println!("Error: Could not extract content from Wikia page.");
                    }
                } else {
                    println!("Error: Could not retrieve Wikia page.");
                }
            }
            _ => {
                println!("Invalid format. Supported formats: text, markdown, wikia");
            }
        }

        Ok(())
    }

    pub fn generate_graph(&mut self, output_file: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Build the backlinks map
        for entry in walkdir::WalkDir::new(&self.root_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let page_name = entry.path().file_stem().unwrap().to_str().unwrap();
            let _ = self.read_page(page_name, &config); // This will populate backlinks
        }

        // 2. Generate the DOT graph definition
        let mut dot_graph = String::from("digraph wiki {\\n");
        for (page, backlinks) in &self.backlinks {
            for backlink in backlinks {
                dot_graph.push_str(&format!("\t\\\"{}\\\" -> \\\"{}\\\";\\n", backlink, page));
            }
        }
        dot_graph.push_str("}\n");

        // 3. Determine output file
        let output_path = output_file.unwrap_or_else(|| self.root_dir.join("wiki_graph.png"));

        // 4. Execute the 'dot' command to generate the graph image
        let output = Command::new("dot")
            .arg("-Tpng")
            .arg(format!("-o{}", output_path.display()))
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        // Write the DOT graph definition to the 'dot' command's stdin
        if let Some(mut stdin) = output.stdin.take() {
            stdin.write_all(dot_graph.as_bytes())?;
        }

        output.wait_with_output()?; // Wait for the 'dot' command to finish

        println!("Wiki graph generated at: {}", output_path.display());

        Ok(())
    }

    pub fn get_backlinks(&self, page_name: &str) -> Vec<String> {
        self.backlinks.get(page_name).cloned().unwrap_or_default()
    }

    pub fn index_page(&self, page_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let page_path = self.get_page_path(page_name);

        // Read the content of the page
        let contents = std::fs::read_to_string(&page_path)?;
        let tags = self.extract_tags(&contents); // Extract tags
        let metadata = fs::metadata(&page_path)?; // Get metadata
        let last_modified = metadata.modified()?; // Get last modified time

        // Get the schema fields
        let schema = self.index.schema();
        let (name, content, tags, modified_date) = get_schema_fields(&schema);

        // Add the page to the index
        let mut index_writer = self.index.writer(50_000_000)?;
        index_writer.add_document(doc!(
            name => page_name,
            content => contents,
            tags => tags.join(" "), // Assuming you want to search tags as text
            content => last_modified.duration_since(SystemTime::UNIX_EPOCH)?.as_millis().to_string()
        ))?;
        index_writer.commit()?;

        Ok(())
    }

    fn process_wikilinks(&mut self, content: &str, config: &Config, current_page: &str) -> String {
        let re = Regex::new(r#"\\[\\[(?:([^:]+):)?([^\\[\\]]+)\\]\\]"#).unwrap();

        re.replace_all(content, |caps: &Captures| {
            let wiki_name = caps.get(1).map(|m| m.as_str());
            let linked_page = caps.get(2).unwrap().as_str();

            // Track backlinks
            if let Some(wiki_name) = wiki_name {
                // Inter-wiki link (currently no backlinks for inter-wiki) 
            } else {
                // Normal wikilink (within the current wiki)
                self.backlinks
                    .entry(linked_page.to_string())
                    .or_insert_with(Vec::new)
                    .push(current_page.to_string());
            }

            // Construct the link based on whether it's an inter-wiki link
            if let Some(wiki_name) = wiki_name {
                // Inter-wiki link
                if let Some(wiki_path) = config.wiki_paths.get(wiki_name) {
                    let linked_page_path = wiki_path.join(format!("{}.md", linked_page));
                    let link_text = if linked_page_path.exists() {
                        format!("[{}]({})", linked_page, linked_page_path.display())
                    } else {
                        format!("[{} (not created yet)]({})", linked_page, linked_page_path.display())
                    };
                    link_text
                } else {
                    format!("[[{}:{} (wiki not found)]]", wiki_name, linked_page)
                }
            } else {
                // Normal wikilink (within the current wiki)
                let linked_page_path = self.get_page_path(linked_page);
                let link_text = if linked_page_path.exists() {
                    format!("[{}]({})", linked_page, linked_page_path.display())
                } else {
                    format!("[{} (not created yet)]({})", linked_page, linked_page_path.display())
                };
                link_text
            }
        }).to_string()
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
