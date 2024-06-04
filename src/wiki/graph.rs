// src/wiki/graph.rs

use std::process::Command;
use std::path::PathBuf;

use crate::config::Config;

pub struct Wiki {
    // ... other fields
}

impl Wiki {
    // ... other methods 

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

    // ... (other methods)
}

// ... helper functions (create_or_load_index, get_schema_fields, get_snippet, sanitize_tag, etc.)
