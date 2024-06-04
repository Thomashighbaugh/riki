// src/wiki/search.rs

use tantivy::{query::{Query, FuzzyTermQuery, BooleanQuery, Term, BooleanClause, TermRange}, schema::Field, collector::TopDocs, DocAddress};
use tantivy::{query::{QueryParser, TermQuery}, schema::{Schema, TEXT}, collector::TopDocs};
use tantivy::Term as TantivyTerm;
use std::fs::{self, File};
use std::io::{Read, Write};
use chrono::{DateTime, Utc, NaiveDate, Datelike};
use std::str::FromStr;
use std::time::SystemTime;
use std::time::Duration;

use crossterm::{
    style::{self, Color, PrintStyledContent},
};

use crate::config::Config;

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub index: Index,
    pub backlinks: HashMap<String, Vec<String>>,
    pub tag_cache: HashMap<PathBuf, (Vec<String>, SystemTime)>,
}

impl Wiki {
    // ... (Other methods)

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
                modified_date,
                TantivyTerm::from_field_text(modified_date, ×tamp_from.to_string()),
                TantivyTerm::from_field_text(modified_date, ×tamp_to.to_string()),
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

fn get_snippet(
    schema: &Schema,
    doc: &Document,
    field: Field,
    query: &Box<dyn Query>,
    searcher: &Searcher,
    snippet_length: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let highlighter = Highlighter::new(schema, field, SummaryOptions {
        fragment_size: snippet_length, // Adjust the desired snippet length
        ..Default::default()
    });
    let mut result = String::new();
    let snippet_result = highlighter.highlight(query, doc, &searcher)?;
    for fragment in snippet_result.iter() {
        let fragment_string: String = fragment?.iter().collect();
        result.push_str(&format!("...{}...", fragment_string));
    }
    Ok(result)
}
