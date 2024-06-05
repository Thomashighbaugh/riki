// THIS IS THE FILE: src/wiki/graph.rs

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::wiki::page::Wiki;
use crate::wiki::backlinks::Backlinks;
use petgraph::graphmap::UnGraphMap;
use petgraph::dot::{Dot, Config};
use petgraph::prelude::*;

pub struct Graph {
    pub root_dir: PathBuf,
}

impl Graph {
    pub fn new(wiki: &Wiki) -> Graph {
        Graph {
            root_dir: wiki.root_dir.clone(),
        }
    }

    pub fn generate_graph(&mut self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let mut wiki = Wiki::new(self.root_dir.clone(), config.templates_dir.clone(), config);
        let mut backlinks = Backlinks::new();
        backlinks.update(&mut wiki, config);

        let mut graph = UnGraphMap::new();

        // Add nodes for each wiki page
        for page_name in wiki.list_pages(config)? {
            graph.add_node(page_name);
        }

        // Add edges for backlinks
        for (page, backlinks) in &backlinks.backlinks {
            for backlink in backlinks {
                graph.add_edge(page.to_string(), backlink.to_string(), ());
            }
        }

        // Generate DOT graph representation
        let dot_graph = Dot::with_config(&graph, &[Config::NodeNoLabel, Config::EdgeNoLabel]);
        let dot_graph = format!("{}", dot_graph);

        // Use Graphviz to convert DOT to an image
        let output_file = std::env::var("RIKI_GRAPH_OUTPUT");

        let output_path = output_file.unwrap_or_else(|| self.root_dir.join("wiki_graph.png"));

        let mut cmd = std::process::Command::new("dot");
        cmd.arg("-Tpng");
        cmd.arg("-o");
        cmd.arg(&output_path);
        cmd.stdin(std::process::Stdio::piped());
        let mut child = cmd.spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(dot_graph.as_bytes())?;
        stdin.flush()?;
        let _ = child.wait()?;

        Ok(())
    }
}
