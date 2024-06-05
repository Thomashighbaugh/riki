// THIS IS THE FILE: src/cli/main_menu.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn print_main_menu(
    stdout: &mut io::Stdout,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let (_width, height) = terminal::size()?;

    // Clear the terminal
    terminal::Clear(ClearType::All)?;

    // Print the main menu
    writeln!(
        stdout,
        "{}",
        style::style("Riki: Your Personal Command-Line Wiki").bold().green()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style(format!("Current Wiki: {}", config.wiki_paths.keys().next().unwrap_or("None"))).bold()
    )?;
    writeln!(stdout, "{}", style::style("-----------------------").bold());
    writeln!(
        stdout,
        "{}",
        style::style("1. Add a Wiki").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("2. New Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("3. Search").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("4. Edit Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("5. Delete Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("6. List Pages").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("7. Manage Templates").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("8. Manage Tags").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("9. Backlinks").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("10. Generate Wiki Graph").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("11. View Page History").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("12. Revert Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("13. Export Pages").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("14. Import Pages").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("q. Quit").bold().dim()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn handle_main_menu_input(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // Wait for user input
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    add_wiki(stdout, config)?;
                    break;
                }
                KeyCode::Char('2') => {
                    new_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('3') => {
                    search(stdout, config)?;
                    break;
                }
                KeyCode::Char('4') => {
                    edit_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('5') => {
                    delete_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('6') => {
                    list_pages(stdout, config)?;
                    break;
                }
                KeyCode::Char('7') => {
                    templates(stdout, config)?;
                    break;
                }
                KeyCode::Char('8') => {
                    tags(stdout, config)?;
                    break;
                }
                KeyCode::Char('9') => {
                    backlinks(stdout, config)?;
                    break;
                }
                KeyCode::Char('1')
                    if key.modifiers == KeyModifiers::SHIFT =>
                {
                    graph(stdout, config)?;
                    break;
                }
                KeyCode::Char('1')
                    if key.modifiers == KeyModifiers::ALT =>
                {
                    history(stdout, config)?;
                    break;
                }
                KeyCode::Char('1')
                    if key.modifiers == KeyModifiers::CONTROL =>
                {
                    revert_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('2')
                    if key.modifiers == KeyModifiers::SHIFT =>
                {
                    export_pages(stdout, config)?;
                    break;
                }
                KeyCode::Char('2')
                    if key.modifiers == KeyModifiers::ALT =>
                {
                    import_pages(stdout, config)?;
                    break;
                }
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            }
            print_main_menu(stdout, config)?;
        }
    }
    Ok(())
}

pub fn add_wiki(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for wiki name
    writeln!(
        stdout,
        "{}",
        style::style("Enter a name for your new wiki: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut wiki_name = String::new();
    io::stdin().read_line(&mut wiki_name)?;
    wiki_name = wiki_name.trim().to_string();

    // Ask for wiki path
    writeln!(
        stdout,
        "{}",
        style::style("Enter the path to your new wiki directory: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut wiki_path_str = String::new();
    io::stdin().read_line(&mut wiki_path_str)?;
    let wiki_path_str = wiki_path_str.trim();
    let wiki_path = PathBuf::from(wiki_path_str);

    // Add the wiki to the configuration
    config.wiki_paths.insert(wiki_name, wiki_path);
    save_config(config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Wiki '{}' added successfully.", wiki_name)).bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn new_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter a name for your new page: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for template name
    let templates = Templates::new(config).list_templates()?;
    if templates.is_empty() {
        writeln!(
            stdout,
            "{}",
            style::style("No templates available. Creating a blank page.").bold()
        )?;
        stdout.flush()?;
    } else {
        writeln!(
            stdout,
            "{}",
            style::style("Choose a template (enter 'q' to create a blank page): ").bold().dim()
        )?;
        stdout.flush()?;

        for (i, template) in templates.iter().enumerate() {
            writeln!(
                stdout,
                "{}",
                style::style(format!("{}: {}", i + 1, template)).dim()
            )?;
            stdout.flush()?;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let template = if input == "q" {
            None
        } else if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= templates.len() {
                Some(templates[index - 1].as_str())
            } else {
                writeln!(
                    stdout,
                    "{}",
                    style::style("Invalid template choice.").bold().red()
                )?;
                stdout.flush()?;
                return Ok(());
            }
        } else {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid input.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        };

        // Create the new page
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut wiki = wiki;
        wiki.create_page(&page_name, "", template)?;

        // Open the page in the default editor
        if let Some(editor) = &config.editor {
            let mut cmd = std::process::Command::new(editor);
            cmd.arg(wiki.get_page_path(&page_name));
            let _ = cmd.spawn();
        }
    }

    Ok(())
}

pub fn search(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for search query
    writeln!(
        stdout,
        "{}",
        style::style("Enter your search query: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut query = String::new();
    io::stdin().read_line(&mut query)?;
    query = query.trim().to_string();

    // Ask for tags
    writeln!(
        stdout,
        "{}",
        style::style("Enter any tags (comma-separated, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut tags = String::new();
    io::stdin().read_line(&mut tags)?;
    let tags: Vec<&str> = tags.trim().split(',').collect();

    // Ask for directories
    writeln!(
        stdout,
        "{}",
        style::style("Enter any directories (comma-separated, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut directories = String::new();
    io::stdin().read_line(&mut directories)?;
    let directories: Vec<&str> = directories.trim().split(',').collect();

    // Ask for date range (optional)
    writeln!(
        stdout,
        "{}",
        style::style("Enter a date range (YYYY-MM-DD, YYYY-MM-DD, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut date_range = String::new();
    io::stdin().read_line(&mut date_range)?;
    let date_range: Vec<&str> = date_range.trim().split(',').collect();

    // Perform the search
    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let search_results = wiki.search(
        &query,
        &tags,
        &directories,
        &date_range,
        config.snippet_length,
    )?;

    // Display the search results
    terminal::Clear(ClearType::All)?;
    writeln!(
        stdout,
        "{}",
        style::style(format!("Search results for '{}':", query)).bold().green()
    )?;
    stdout.flush()?;

    for result in search_results {
        writeln!(
            stdout,
            "{}",
            style::style(format!("{} - {}", result.page_name, result.snippet))
                .dim()
        )?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn edit_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to edit: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Open the page in the default editor
    if let Some(editor) = &config.editor {
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut cmd = std::process::Command::new(editor);
        cmd.arg(wiki.get_page_path(&page_name));
        let _ = cmd.spawn();
    }

    Ok(())
}

pub fn delete_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to delete: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Confirm deletion
    writeln!(
        stdout,
        "{}",
        style::style(format!("Are you sure you want to delete '{}'? (y/n): ", page_name))
            .bold()
            .red()
    )?;
    stdout.flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    if input == "y" {
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut wiki = wiki;
        wiki.delete_page(&page_name)?;
        writeln!(
            stdout,
            "{}",
            style::style(format!("Page '{}' deleted successfully.", page_name))
                .bold()
                .green()
        )?;
        stdout.flush()?;
    } else {
        writeln!(
            stdout,
            "{}",
            style::style("Deletion cancelled.").bold()
        )?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn list_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let pages = wiki.list_pages(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Pages in the current wiki:").bold().green()
    )?;
    stdout.flush()?;

    for page in pages {
        writeln!(stdout, "{}", style::style(page).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn templates(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let templates = Templates::new(config).list_templates()?;

    writeln!(
        stdout,
        "{}",
        style::style("Available Templates:").bold().green()
    )?;
    stdout.flush()?;

    for template in templates {
        writeln!(stdout, "{}", style::style(template).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn tags(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let tags = wiki.list_tags(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Tags in the current wiki:").bold().green()
    )?;
    stdout.flush()?;

    for tag in tags {
        writeln!(stdout, "{}", style::style(tag).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn backlinks(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to view backlinks for: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let backlinks = wiki.get_backlinks(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Backlinks for '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for backlink in backlinks {
        writeln!(stdout, "{}", style::style(backlink).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn graph(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let mut graph = Graph::new(&wiki);
    graph.generate_graph(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Wiki graph generated successfully.").bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn history(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to view history for: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let history = wiki.get_page_history(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("History for '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for commit in history {
        writeln!(stdout, "{}", style::style(commit).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn revert_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to revert: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for commit hash
    writeln!(
        stdout,
        "{}",
        style::style("Enter the commit hash to revert to: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut commit_hash = String::new();
    io::stdin().read_line(&mut commit_hash)?;
    commit_hash = commit_hash.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    wiki.revert_page(&page_name, &commit_hash, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Page '{}' reverted successfully.", page_name))
            .bold()
            .green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn export_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let mut export = Export::new(&wiki);

    // Ask for export format
    writeln!(
        stdout,
        "{}",
        style::style("Choose an export format (html/pdf/txt): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut format = String::new();
    io::stdin().read_line(&mut format)?;
    format = format.trim().to_lowercase();

    match format.as_str() {
        "html" => export.export_html(config)?,
        "pdf" => export.export_pdf(config)?,
        "txt" => export.export_txt(config)?,
        _ => {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid export format.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        }
    }

    writeln!(
        stdout,
        "{}",
        style::style(format!("Pages exported successfully to '{}'.", config.wiki_paths.get("main").unwrap().join("export").display())).bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn import_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for import source
    writeln!(
        stdout,
        "{}",
        style::style("Choose an import source (files/wiki/url): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut source = String::new();
    io::stdin().read_line(&mut source)?;
    source = source.trim().to_lowercase();

    match source.as_str() {
        "files" => {
            // Ask for import directory
            writeln!(
                stdout,
                "{}",
                style::style("Enter the path to the directory to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut import_dir_str = String::new();
            io::stdin().read_line(&mut import_dir_str)?;
            let import_dir_str = import_dir_str.trim();
            let import_dir = PathBuf::from(import_dir_str);

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_files(&import_dir, config)?;
            writeln!(
                stdout,
                "{}",
                style::style("Pages imported successfully.").bold().green()
            )?;
            stdout.flush()?;
        }
        "wiki" => {
            // Ask for wiki URL
            writeln!(
                stdout,
                "{}",
                style::style("Enter the Wiki URL to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut wiki_url = String::new();
            io::stdin().read_line(&mut wiki_url)?;
            wiki_url = wiki_url.trim().to_string();

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_wiki(&wiki_url, config)?;
            writeln!(
                stdout,
                "{}",
                style::style(format!("Page imported successfully from '{}'.", wiki_url))
                    .bold()
                    .green()
            )?;
            stdout.flush()?;
        }
        "url" => {
            // Ask for URL
            writeln!(
                stdout,
                "{}",
                style::style("Enter the URL to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut url = String::new();
            io::stdin().read_line(&mut url)?;
            url = url.trim().to_string();

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_url(&url, config)?;
            writeln!(
                stdout,
                "{}",
                style::style(format!("Page imported successfully from '{}'.", url))
                    .bold()
                    .green()
            )?;
            stdout.flush()?;
        }
        _ => {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid import source.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        }
    }

    Ok(())
}// THIS IS THE FILE: src/cli/main_menu.rs

use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{self, Color, Print, PrintStyledContent},
    terminal::{self, size, Clear, ClearType},
};
use std::path::PathBuf;

use crate::config::{install_default_templates, load_config, save_config, Config};
use crate::wiki::Wiki;

pub fn print_main_menu(
    stdout: &mut io::Stdout,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let (_width, height) = terminal::size()?;

    // Clear the terminal
    terminal::Clear(ClearType::All)?;

    // Print the main menu
    writeln!(
        stdout,
        "{}",
        style::style("Riki: Your Personal Command-Line Wiki").bold().green()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style(format!("Current Wiki: {}", config.wiki_paths.keys().next().unwrap_or("None"))).bold()
    )?;
    writeln!(stdout, "{}", style::style("-----------------------").bold());
    writeln!(
        stdout,
        "{}",
        style::style("1. Add a Wiki").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("2. New Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("3. Search").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("4. Edit Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("5. Delete Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("6. List Pages").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("7. Manage Templates").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("8. Manage Tags").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("9. Backlinks").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("10. Generate Wiki Graph").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("11. View Page History").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("12. Revert Page").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("13. Export Pages").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("14. Import Pages").bold().dim()
    )?;
    writeln!(
        stdout,
        "{}",
        style::style("q. Quit").bold().dim()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn handle_main_menu_input(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // Wait for user input
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    add_wiki(stdout, config)?;
                    break;
                }
                KeyCode::Char('2') => {
                    new_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('3') => {
                    search(stdout, config)?;
                    break;
                }
                KeyCode::Char('4') => {
                    edit_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('5') => {
                    delete_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('6') => {
                    list_pages(stdout, config)?;
                    break;
                }
                KeyCode::Char('7') => {
                    templates(stdout, config)?;
                    break;
                }
                KeyCode::Char('8') => {
                    tags(stdout, config)?;
                    break;
                }
                KeyCode::Char('9') => {
                    backlinks(stdout, config)?;
                    break;
                }
                KeyCode::Char('1')
                    if key.modifiers == KeyModifiers::SHIFT =>
                {
                    graph(stdout, config)?;
                    break;
                }
                KeyCode::Char('1')
                    if key.modifiers == KeyModifiers::ALT =>
                {
                    history(stdout, config)?;
                    break;
                }
                KeyCode::Char('1')
                    if key.modifiers == KeyModifiers::CONTROL =>
                {
                    revert_page(stdout, config)?;
                    break;
                }
                KeyCode::Char('2')
                    if key.modifiers == KeyModifiers::SHIFT =>
                {
                    export_pages(stdout, config)?;
                    break;
                }
                KeyCode::Char('2')
                    if key.modifiers == KeyModifiers::ALT =>
                {
                    import_pages(stdout, config)?;
                    break;
                }
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            }
            print_main_menu(stdout, config)?;
        }
    }
    Ok(())
}

pub fn add_wiki(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for wiki name
    writeln!(
        stdout,
        "{}",
        style::style("Enter a name for your new wiki: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut wiki_name = String::new();
    io::stdin().read_line(&mut wiki_name)?;
    wiki_name = wiki_name.trim().to_string();

    // Ask for wiki path
    writeln!(
        stdout,
        "{}",
        style::style("Enter the path to your new wiki directory: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut wiki_path_str = String::new();
    io::stdin().read_line(&mut wiki_path_str)?;
    let wiki_path_str = wiki_path_str.trim();
    let wiki_path = PathBuf::from(wiki_path_str);

    // Add the wiki to the configuration
    config.wiki_paths.insert(wiki_name, wiki_path);
    save_config(config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Wiki '{}' added successfully.", wiki_name)).bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn new_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter a name for your new page: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for template name
    let templates = Templates::new(config).list_templates()?;
    if templates.is_empty() {
        writeln!(
            stdout,
            "{}",
            style::style("No templates available. Creating a blank page.").bold()
        )?;
        stdout.flush()?;
    } else {
        writeln!(
            stdout,
            "{}",
            style::style("Choose a template (enter 'q' to create a blank page): ").bold().dim()
        )?;
        stdout.flush()?;

        for (i, template) in templates.iter().enumerate() {
            writeln!(
                stdout,
                "{}",
                style::style(format!("{}: {}", i + 1, template)).dim()
            )?;
            stdout.flush()?;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let template = if input == "q" {
            None
        } else if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= templates.len() {
                Some(templates[index - 1].as_str())
            } else {
                writeln!(
                    stdout,
                    "{}",
                    style::style("Invalid template choice.").bold().red()
                )?;
                stdout.flush()?;
                return Ok(());
            }
        } else {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid input.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        };

        // Create the new page
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut wiki = wiki;
        wiki.create_page(&page_name, "", template)?;

        // Open the page in the default editor
        if let Some(editor) = &config.editor {
            let mut cmd = std::process::Command::new(editor);
            cmd.arg(wiki.get_page_path(&page_name));
            let _ = cmd.spawn();
        }
    }

    Ok(())
}

pub fn search(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for search query
    writeln!(
        stdout,
        "{}",
        style::style("Enter your search query: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut query = String::new();
    io::stdin().read_line(&mut query)?;
    query = query.trim().to_string();

    // Ask for tags
    writeln!(
        stdout,
        "{}",
        style::style("Enter any tags (comma-separated, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut tags = String::new();
    io::stdin().read_line(&mut tags)?;
    let tags: Vec<&str> = tags.trim().split(',').collect();

    // Ask for directories
    writeln!(
        stdout,
        "{}",
        style::style("Enter any directories (comma-separated, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut directories = String::new();
    io::stdin().read_line(&mut directories)?;
    let directories: Vec<&str> = directories.trim().split(',').collect();

    // Ask for date range (optional)
    writeln!(
        stdout,
        "{}",
        style::style("Enter a date range (YYYY-MM-DD, YYYY-MM-DD, optional): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut date_range = String::new();
    io::stdin().read_line(&mut date_range)?;
    let date_range: Vec<&str> = date_range.trim().split(',').collect();

    // Perform the search
    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let search_results = wiki.search(
        &query,
        &tags,
        &directories,
        &date_range,
        config.snippet_length,
    )?;

    // Display the search results
    terminal::Clear(ClearType::All)?;
    writeln!(
        stdout,
        "{}",
        style::style(format!("Search results for '{}':", query)).bold().green()
    )?;
    stdout.flush()?;

    for result in search_results {
        writeln!(
            stdout,
            "{}",
            style::style(format!("{} - {}", result.page_name, result.snippet))
                .dim()
        )?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn edit_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to edit: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Open the page in the default editor
    if let Some(editor) = &config.editor {
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut cmd = std::process::Command::new(editor);
        cmd.arg(wiki.get_page_path(&page_name));
        let _ = cmd.spawn();
    }

    Ok(())
}

pub fn delete_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to delete: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Confirm deletion
    writeln!(
        stdout,
        "{}",
        style::style(format!("Are you sure you want to delete '{}'? (y/n): ", page_name))
            .bold()
            .red()
    )?;
    stdout.flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    if input == "y" {
        let wiki_path = config
            .wiki_paths
            .get("main")
            .unwrap()
            .clone();
        let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
        let mut wiki = wiki;
        wiki.delete_page(&page_name)?;
        writeln!(
            stdout,
            "{}",
            style::style(format!("Page '{}' deleted successfully.", page_name))
                .bold()
                .green()
        )?;
        stdout.flush()?;
    } else {
        writeln!(
            stdout,
            "{}",
            style::style("Deletion cancelled.").bold()
        )?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn list_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let pages = wiki.list_pages(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Pages in the current wiki:").bold().green()
    )?;
    stdout.flush()?;

    for page in pages {
        writeln!(stdout, "{}", style::style(page).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn templates(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let templates = Templates::new(config).list_templates()?;

    writeln!(
        stdout,
        "{}",
        style::style("Available Templates:").bold().green()
    )?;
    stdout.flush()?;

    for template in templates {
        writeln!(stdout, "{}", style::style(template).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn tags(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let tags = wiki.list_tags(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Tags in the current wiki:").bold().green()
    )?;
    stdout.flush()?;

    for tag in tags {
        writeln!(stdout, "{}", style::style(tag).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn backlinks(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to view backlinks for: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let backlinks = wiki.get_backlinks(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Backlinks for '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for backlink in backlinks {
        writeln!(stdout, "{}", style::style(backlink).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn graph(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let mut graph = Graph::new(&wiki);
    graph.generate_graph(config)?;

    writeln!(
        stdout,
        "{}",
        style::style("Wiki graph generated successfully.").bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn history(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to view history for: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let history = wiki.get_page_history(&page_name, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("History for '{}':", page_name)).bold().green()
    )?;
    stdout.flush()?;

    for commit in history {
        writeln!(stdout, "{}", style::style(commit).dim())?;
        stdout.flush()?;
    }

    Ok(())
}

pub fn revert_page(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for page name
    writeln!(
        stdout,
        "{}",
        style::style("Enter the name of the page to revert: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut page_name = String::new();
    io::stdin().read_line(&mut page_name)?;
    page_name = page_name.trim().to_string();

    // Ask for commit hash
    writeln!(
        stdout,
        "{}",
        style::style("Enter the commit hash to revert to: ").bold().dim()
    )?;
    stdout.flush()?;

    let mut commit_hash = String::new();
    io::stdin().read_line(&mut commit_hash)?;
    commit_hash = commit_hash.trim().to_string();

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    wiki.revert_page(&page_name, &commit_hash, config)?;

    writeln!(
        stdout,
        "{}",
        style::style(format!("Page '{}' reverted successfully.", page_name))
            .bold()
            .green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn export_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    let wiki_path = config
        .wiki_paths
        .get("main")
        .unwrap()
        .clone();
    let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
    let mut export = Export::new(&wiki);

    // Ask for export format
    writeln!(
        stdout,
        "{}",
        style::style("Choose an export format (html/pdf/txt): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut format = String::new();
    io::stdin().read_line(&mut format)?;
    format = format.trim().to_lowercase();

    match format.as_str() {
        "html" => export.export_html(config)?,
        "pdf" => export.export_pdf(config)?,
        "txt" => export.export_txt(config)?,
        _ => {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid export format.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        }
    }

    writeln!(
        stdout,
        "{}",
        style::style(format!("Pages exported successfully to '{}'.", config.wiki_paths.get("main").unwrap().join("export").display())).bold().green()
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn import_pages(
    stdout: &mut io::Stdout,
    config: &mut Config,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal::Clear(ClearType::All)?;

    // Ask for import source
    writeln!(
        stdout,
        "{}",
        style::style("Choose an import source (files/wiki/url): ").bold().dim()
    )?;
    stdout.flush()?;

    let mut source = String::new();
    io::stdin().read_line(&mut source)?;
    source = source.trim().to_lowercase();

    match source.as_str() {
        "files" => {
            // Ask for import directory
            writeln!(
                stdout,
                "{}",
                style::style("Enter the path to the directory to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut import_dir_str = String::new();
            io::stdin().read_line(&mut import_dir_str)?;
            let import_dir_str = import_dir_str.trim();
            let import_dir = PathBuf::from(import_dir_str);

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_files(&import_dir, config)?;
            writeln!(
                stdout,
                "{}",
                style::style("Pages imported successfully.").bold().green()
            )?;
            stdout.flush()?;
        }
        "wiki" => {
            // Ask for wiki URL
            writeln!(
                stdout,
                "{}",
                style::style("Enter the Wiki URL to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut wiki_url = String::new();
            io::stdin().read_line(&mut wiki_url)?;
            wiki_url = wiki_url.trim().to_string();

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_wiki(&wiki_url, config)?;
            writeln!(
                stdout,
                "{}",
                style::style(format!("Page imported successfully from '{}'.", wiki_url))
                    .bold()
                    .green()
            )?;
            stdout.flush()?;
        }
        "url" => {
            // Ask for URL
            writeln!(
                stdout,
                "{}",
                style::style("Enter the URL to import from: ").bold().dim()
            )?;
            stdout.flush()?;

            let mut url = String::new();
            io::stdin().read_line(&mut url)?;
            url = url.trim().to_string();

            let wiki_path = config
                .wiki_paths
                .get("main")
                .unwrap()
                .clone();
            let wiki = Wiki::new(wiki_path, config.templates_dir.clone(), config);
            let mut import = Import::new(&wiki);
            import.import_from_url(&url, config)?;
            writeln!(
                stdout,
                "{}",
                style::style(format!("Page imported successfully from '{}'.", url))
                    .bold()
                    .green()
            )?;
            stdout.flush()?;
        }
        _ => {
            writeln!(
                stdout,
                "{}",
                style::style("Invalid import source.").bold().red()
            )?;
            stdout.flush()?;
            return Ok(());
        }
    }

    Ok(())
}
