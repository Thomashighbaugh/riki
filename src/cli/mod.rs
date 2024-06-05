// src/cli/mod.rs

pub mod main_menu;
pub mod add_wiki;
pub mod new_page;
pub mod search;
pub mod edit_page;
pub mod delete_page;
pub mod list_pages;
pub mod templates;
pub mod tags;
pub mod backlinks;
pub mod graph;
pub mod history;
pub mod import;

pub use main_menu::*;
pub use add_wiki::*;
pub use new_page::*;
pub use search::*;
pub use edit_page::*;
pub use delete_page::*;
pub use list_pages::*;
pub use templates::*;
pub use tags::*;
pub use backlinks::*;
pub use graph::*;
pub use history::*;
pub use import::*;
