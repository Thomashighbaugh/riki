// THIS IS THE FILE: src/wiki/search.rs

use std::fs;
use std::path::{Path, PathBuf};

use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, BooleanClause, FuzzyTermQuery, Query, TermQuery, PhraseQuery},
    schema::{Schema, IndexRecordOption, Field, TextOptions, Index, Term},
    IndexWriter,
    Score,
    Document,
    IndexReader,
    Searcher,
    ReloadPolicy,
};
use chrono::{NaiveDate, NaiveDateTime};
use tantivy_query_grammar::parse_query;

use crate::config::Config;
use crate::wiki::page::Wiki;

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub index: Index,
    pub schema: Schema,
}

impl Wiki {
    pub fn new(root_dir: PathBuf, templates_dir: PathBuf, config: &Config) -> Wiki {
        // Load or create the search index
        let index_path = config.index_dir.clone();
        let index = if index_path.exists() {
            Index::open(index_path).unwrap()
        } else {
            // Create a new index if it doesn't exist
            let mut schema_builder = Schema::builder();
            schema_builder.add_text_field(
                "content",
                TextOptions::default()
                    .set_indexing_options(IndexRecordOption::WithFreqsAndPositions)
                    .set_tokenizer("en"),
            );
            schema_builder.add_text_field(
                "tags",
                TextOptions::default()
                    .set_indexing_options(IndexRecordOption::WithFreqsAndPositions)
                    .set_tokenizer("en"),
            );
            schema_builder.add_text_field(
                "directory",
                TextOptions::default()
                    .set_indexing_options(IndexRecordOption::WithFreqsAndPositions)
                    .set_tokenizer("en"),
            );
            schema_builder.add_u64_field("date", Index);
            let schema = schema_builder.build();
            Index::create(index_path, schema.clone()).unwrap()
        };

        let index = index;
        let schema = schema;

        Wiki {
            root_dir,
            templates_dir,
            index,
            schema,
        }
    }

    pub fn search(
        &self,
        query: &str,
        tags: &[&str],
        directories: &[&str],
        date_range: &[&str],
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        // Parse the query using tantivy-query-grammar
        let query = parse_query(&self.schema, query)?;

        let mut search_query = BooleanQuery::new(true);

        // Add the main search query
        search_query.add(Box::new(query) as Box<dyn Query>, BooleanClause::Must);

        // Add tag filters
        if !tags.is_empty() {
            for tag in tags {
                let tag_query = TermQuery::new(
                    Term::from_field_text(self.schema.get_field("tags").unwrap(), tag),
                    IndexRecordOption::Basic,
                );
                search_query.add(Box::new(tag_query) as Box<dyn Query>, BooleanClause::Must);
            }
        }

        // Add directory filters
        if !directories.is_empty() {
            for directory in directories {
                let directory_query = TermQuery::new(
                    Term::from_field_text(self.schema.get_field("directory").unwrap(), directory),
                    IndexRecordOption::Basic,
                );
                search_query.add(Box::new(directory_query) as Box<dyn Query>, BooleanClause::Must);
            }
        }

        // Add date filters
        if date_range.len() == 2 {
            if let (Ok(start_date), Ok(end_date)) = (
                date_range[0].parse::<NaiveDate>(),
                date_range[1].parse::<NaiveDate>(),
            ) {
                let start_date_filter =
                    self.schema.get_field("date").unwrap();

                let end_date_filter =
                    self.schema.get_field("date").unwrap();

                search_query.add(Box::new(start_date_filter) as Box<dyn Query>, BooleanClause::Must);
                search_query.add(Box::new(end_date_filter) as Box<dyn Query>, BooleanClause::Must);
            }
        }

        let top_docs = searcher.search(&search_query, &TopDocs::with_limit(10))?;

        let mut search_results = Vec::new();
        for (doc_address, score) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            let page_name = retrieved_doc
                .get_first(self.schema.get_field("content").unwrap())
                .unwrap()
                .text()
                .unwrap()
                .to_string();
            let content = self.read_page(&page_name, &Config::default())?;
            search_results.push(SearchResult {
                page_name,
                content,
                score,
            });
        }

        Ok(search_results)
    }

    pub fn index_wiki(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        // Open the index writer
        let mut index_writer = self.index.writer(50_000_000)?;
        index_writer.set_reload_policy(ReloadPolicy::OnCommit);

        // Iterate through all wiki pages and index them
        for entry in walkdir::WalkDir::new(&self.root_dir) {
            if let Ok(entry) = entry {
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

                    // Read the page content
                    let contents = fs::read_to_string(entry.path()).unwrap();

                    // Extract tags
                    let tags = self.extract_tags(&contents);

                    // Extract directory
                    let directory = entry
                        .path()
                        .parent()
                        .unwrap()
                        .strip_prefix(&self.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    // Get last modified date
                    let last_modified_date = fs::metadata(entry.path())
                        .unwrap()
                        .modified()
                        .unwrap();
                    let last_modified_date = NaiveDateTime::from_timestamp(
                        last_modified_date.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                        0,
                    );

                    // Create a new document
                    let mut document = Document::new();
                    document.add_text(
                        self.schema.get_field("content").unwrap(),
                        &contents,
                    );
                    document.add_text(
                        self.schema.get_field("tags").unwrap(),
                        tags.join(" ").as_str(),
                    );
                    document.add_text(
                        self.schema.get_field("directory").unwrap(),
                        &directory,
                    );
                    document.add_u64(
                        self.schema.get_field("date").unwrap(),
                        last_modified_date.timestamp() as u64,
                    );

                    // Add the document to the index
                    index_writer.add_document(document)?;
                }
            }
        }

        // Commit the changes
        index_writer.commit()?;
        Ok(())
    }

    fn extract_tags(&self, content: &str) -> Vec<String> {
        let re = Regex::new(r"tags:\s*-\s*(.*?)\s*").unwrap();
        let mut tags = Vec::new();

        for capture in re.captures_iter(content) {
            let tag = capture[1].trim().to_string();
            tags.push(tag);
        }

        tags
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
}

#[derive(Debug)]
pub struct SearchResult {
    pub page_name: String,
    pub content: String,
    pub score: Score,
}
