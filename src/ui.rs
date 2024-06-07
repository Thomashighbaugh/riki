use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::error::Error;
use std::io;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap}, Terminal,
};

pub fn display_help() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(f.size());

        let help_text = vec![
            "Riki is a simple command-line interface for managing wikis.",
            "",
            "Commands:",
            "  config <wiki_url>   - Configure a wiki",
            "  search <term>       - Search for a page",
            "  add <page_name>      - Add a new page",
            "  edit <page_name>     - Edit a page",
            "  view <page_name>     - View a page",
            "  delete <page_name>   - Delete a page",
        ];
        let paragraph = Paragraph::new(help_text.join("\n"))
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Help").borders(Borders::ALL));

        f.render_widget(paragraph, chunks[0]);
    })?;

    // Enter raw mode
    enable_raw_mode()?;

    // Leave raw mode
    disable_raw_mode()?;

    terminal.show_cursor()?;
    Ok(())
}
