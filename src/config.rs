// THIS IS THE FILE: src/config.rs

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub wiki_paths: HashMap<String, PathBuf>,
    pub templates_dir: PathBuf,
    pub index_dir: PathBuf,
    pub editor: Option<String>,
    pub snippet_length: usize,
}

impl Config {
    pub fn default() -> Config {
        let wiki_paths = HashMap::from([
            (
                "main".to_string(),
                dirs::data_local_dir().unwrap().join("riki/wiki"),
            ),
        ]);

        Config {
            wiki_paths,
            templates_dir: dirs::config_dir().unwrap().join("riki/templates"),
            index_dir: dirs::data_local_dir().unwrap().join("riki/index"),
            editor: env::var("EDITOR").ok(),
            snippet_length: 200,
        }
    }
}

pub fn load_config(config_path: &Path) -> Config {
    let config_str = fs::read_to_string(config_path).map_err(|err| {
        println!(
            "Error reading config file: {} - {}",
            config_path.display(),
            err
        );
        std::process::exit(1);
    })
    .unwrap();

    serde_yaml::from_str(&config_str).map_err(|err| {
        println!(
            "Error parsing config file: {} - {}",
            config_path.display(),
            err
        );
        std::process::exit(1);
    })
    .unwrap()
}

pub fn save_config(config: &Config) {
    let config_path = dirs::config_dir().unwrap().join("riki/config.yaml");
    let config_str = serde_yaml::to_string(config).map_err(|err| {
        println!(
            "Error serializing config file: {} - {}",
            config_path.display(),
            err
        );
        std::process::exit(1);
    })
    .unwrap();

    fs::write(config_path, config_str).map_err(|err| {
        println!(
            "Error writing config file: {} - {}",
            config_path.display(),
            err
        );
        std::process::exit(1);
    })
    .unwrap();
}

pub fn install_default_templates(templates_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(templates_dir)?;

    let default_templates = vec![
        ("concept.md", include_str!("templates/concept.md")),
        ("tech-comparison.md", include_str!("templates/tech-comparison.md")),
        ("book-notes.md", include_str!("templates/book-notes.md")),
        ("problem-solution.md", include_str!("templates/problem-solution.md")),
        ("troubleshooting.md", include_str!("templates/troubleshooting.md")),
        ("vulnerability.md", include_str!("templates/vulnerability.md")),
        ("api-doc.md", include_str!("templates/api-doc.md")),
        ("shell-command.md", include_str!("templates/shell-command.md")),
        ("project-idea.md", include_str!("templates/project-idea.md")),
        ("code-snippet.md", include_str!("templates/code-snippet.md")),
        ("architecture-diagram.md", include_str!("templates/architecture-diagram.md")),
        ("interview-prep.md", include_str!("templates/interview-prep.md")),
        ("tool-doc.md", include_str!("templates/tool-doc.md")),
    ];

    for (file_name, content) in default_templates {
        let file_path = templates_dir.join(file_name);
        fs::write(file_path, content)?;
    }

    Ok(())
}
