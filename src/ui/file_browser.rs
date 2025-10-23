use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::app::App;

pub fn render_file_browser(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let current_path = app.current_directory.display().to_string();
    let header = Paragraph::new(format!("Current Directory: {}", current_path))
        .block(Block::default().borders(Borders::ALL).title("File Browser"));
    f.render_widget(header, chunks[0]);

    // File list
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let is_dir = path.is_dir();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("???");

            let display_name = if is_dir {
                format!("📁 {}/", name)
            } else {
                format!("📄 {}", name)
            };

            let style = if i == app.file_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if is_dir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(display_name).style(style)
        })
        .collect();

    let files_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"));
    f.render_widget(files_list, chunks[1]);

    // Footer with help
    let help_text = vec![
        Line::from(vec![
            Span::styled("↑/↓ or j/k: ", Style::default().fg(Color::Gray)),
            Span::raw("Navigate | "),
            Span::styled("Enter: ", Style::default().fg(Color::Gray)),
            Span::raw("Select | "),
            Span::styled("Backspace: ", Style::default().fg(Color::Gray)),
            Span::raw("Parent Dir | "),
            Span::styled("Esc/q: ", Style::default().fg(Color::Gray)),
            Span::raw("Back"),
        ]),
    ];
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}
