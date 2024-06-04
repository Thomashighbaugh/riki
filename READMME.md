# Riki: Your Personalized Command-Line Wiki

Riki is a fast and flexible command-line tool for managing your personal wiki using plain Markdown files. It offers a user-friendly TUI menu for easy navigation and a robust set of features to help you organize and explore your notes.

### Why Did I Write Riki?

I have been on the hunt for a suitable solution to ease my note-taking in both technology related and spiritual related folders of Markdown notes I have written over the years, which I have come veryy close to in my neovim set but I wanted something independent of my text editor of choice just in case I could not use it or I decided in the future to switch to another text editor for whatever reason. 

I also wanted to write a project in rust, which I have been playing with for a while  (because I am convinced it is the future of web development) but wanted to work with it in a new context and in this case, working with it to solve a pressing personal need. 


### Thw Name? 

`riki` = `rust` + `wiki`

If I ever make this into a neovim plugin it will be called `liki` :wink:

## Key Features

- **Simple and Intuitive:** Write your notes in Markdown and organize them in folders.
- **Powerful Search:** Find what you need instantly with full-text search, fuzzy matching, tag-based searching, and date filtering.
- **Flexible Organization:** Use tags and directories to categorize and structure your notes.
- **Templates:** Quickly create new pages from pre-defined templates for meeting notes, blog posts, book notes, and more.
- **Customizable:** Set your preferred text editor, snippet length, and index location in the configuration file.
- **Wikilinks:** Easily link between pages within your wiki using the `[[Page Name]]` syntax, and even link between different wikis using `[[Wiki Name:Page Name]]`.
- **Built-in Tag Management:** List all tags in use, and easily add or remove tags from pages.
- **Backlinks:** Track and display backlinks (pages that link to the current page), which is incredibly helpful for understanding the relationships between your notes and exploring related content.
- **Version Control (Git):** Track changes to your pages using Git, allowing you to view past revisions and revert to previous versions.
- **Graph Visualization:** Generate a visual graph of the connections between pages in your wiki using Graphviz.
- **Export:** Export your pages to HTML, PDF, or plain text formats.
- **Import:** Import notes from plain text, Markdown files, and (in a simplified way) from popular wiki services.
- **TUI Menu:** A user-friendly TUI menu makes it easy to navigate and interact with `riki`.

## What Makes Riki Different?

Riki focuses on providing a streamlined and efficient wiki experience directly from your terminal. Here's what sets it apart:

- **Command-line Driven:** Designed for users who prefer the speed and flexibility of a terminal-based workflow.
- **Plain Text Markdown:** Uses standard Markdown files for notes, ensuring portability and longevity.
- **Lightweight and Fast:** Built in Rust for performance and a small footprint.
- **Extensible:** Potential for future extensibility through plugins and a robust API.
- **Open Source:** The code is available on GitHub for anyone to contribute and improve!

## Getting Started

### 1. Install

```bash
cargo install riki
```


##### Configure

Riki's configuration file is located at ~/.riki/config.yaml. You can customize the following settings:
    - wiki_paths: A HashMap where the key is the name of the wiki and the value is the path to the wiki directory.
    - templates_dir: The directory where your custom templates are stored
    - index_dir: The location of the search index.
    - editor: Your preferred command-line text editor
    - snippet_length: The desired length of search result snippets.

### 3. Usage

When you run riki, you will be presented with a TUI menu that allows you to easily navigate the various features. Here are some of the key actions you can perform:
    - Add a Wiki: Add new wiki directories to your configuration.
    - Create a New Page: Create new pages in your wikis, either from a template or with interactive content entry.
    - Search: Perform full-text searches, filter by tags, directories, or date ranges, and view snippets from the search results.
    - Edit a Page: Open pages for editing using your preferred text editor.
    - Delete a Page: Delete pages from your wiki.
    - List Pages: View a list of pages in your wiki.
    - Manage Templates: List available templates.
    - Manage Tags: View a list of all tags, add or remove tags from pages
    - View Backlinks: See a list of pages that link to the current page.
    - Generate a Wiki Graph: Create a visual graph of the relationships between pages in your wiki using Graphviz.
    - View Page History: View the history of changes made to a page.
    - Revert a Page: Revert a page to a specific commit in its history.
    - Export Pages: Export pages to HTML, PDF, or plain text formats.
    - Import Pages: Import notes from plain text, Markdown files, or (in a simplified way) from Wikia URLs.
