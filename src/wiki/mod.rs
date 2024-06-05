// src/wiki/mod.rs

pub mod page;
pub mod search;
pub mod tags;
pub mod backlinks;
pub mod history;
pub mod templates;
pub mod export;
pub mod import;
pub mod graph;
pub mod utils;

pub use page::*;
pub use search::*;
pub use tags::*;
pub use backlinks::*;
pub use history::*;
pub use templates::*;
pub use export::*;
pub use import::*;
pub use graph::*;
pub use utils::*;

pub struct Wiki {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub index: Index,
    pub backlinks: HashMap<String, Vec<String>>,
    pub tag_cache: HashMap<PathBuf, (Vec<String>, std::time::SystemTime)>,
}
