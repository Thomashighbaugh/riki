// src/cli/mod.rs

pub mod add_wiki;
pub mod backlinks;
pub mod delete_page;
pub mod edit_page;
pub mod graph;
pub mod history;
pub mod list_pages;
pub mod main_menu;
pub mod new_page;
pub mod search;
pub mod tags;
pub mod templates;

pub use add_wiki::*;
pub use backlinks::*;
pub use delete_page::*;
pub use edit_page::*;
pub use graph::*;
pub use history::*;
pub use list_pages::*;
pub use main_menu::*;
pub use new_page::*;
pub use search::*;
pub use tags::*;
pub use templates::*;
