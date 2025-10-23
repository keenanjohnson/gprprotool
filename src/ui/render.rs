use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::app::{App, AppState, MainMenuItem};
use super::file_browser;

pub fn render(f: &mut Frame, app: &App) {
    match app.state {
        AppState::MainMenu => render_main_menu(f, app),
        AppState::FileBrowser => file_browser::render_file_browser(f, app, f.area()),
        AppState::FileInfo => render_file_info(f, app),
        AppState::ConversionConfig => render_conversion_config(f, app),
        AppState::Converting => render_converting(f, app),
        AppState::Complete => render_complete(f, app),
        AppState::Error => render_error(f, app),
    }
}

fn render_main_menu(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = vec![
        Line::from(""),
        Line::from(Span::styled(
            "GprProTool",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "GoPro GPR File Converter",
            Style::default().fg(Color::Gray),
        ))
        .alignment(Alignment::Center),
    ];

    let title_widget = Paragraph::new(title).block(Block::default().borders(Borders::ALL));
    f.render_widget(title_widget, chunks[0]);

    // Menu items
    let menu_items: Vec<ListItem> = MainMenuItem::all()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.main_menu_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if i == app.main_menu_index {
                "> "
            } else {
                "  "
            };

            ListItem::new(format!("{}{}", prefix, item.as_str())).style(style)
        })
        .collect();

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"));
    f.render_widget(menu, chunks[1]);

    // Help
    let help_text = vec![Line::from(vec![
        Span::styled("↑/↓ or j/k: ", Style::default().fg(Color::Gray)),
        Span::raw("Navigate | "),
        Span::styled("Enter: ", Style::default().fg(Color::Gray)),
        Span::raw("Select | "),
        Span::styled("q: ", Style::default().fg(Color::Gray)),
        Span::raw("Quit"),
    ])];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}

fn render_file_info(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // File info
    let mut lines = vec![Line::from("")];

    if let Some(ref file) = app.selected_file {
        lines.push(Line::from(vec![
            Span::styled("File: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&file.filename),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Path: ", Style::default().fg(Color::Gray)),
            Span::raw(file.path.display().to_string()),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Size: ", Style::default().fg(Color::Gray)),
            Span::raw(file.format_size()),
        ]));
        lines.push(Line::from(""));

        if let Some(ref metadata) = file.metadata {
            lines.push(Line::from(Span::styled(
                "Metadata:",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            lines.push(Line::from(vec![
                Span::styled("Camera: ", Style::default().fg(Color::Gray)),
                Span::raw(&metadata.camera_model),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Dimensions: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{}x{}", metadata.width, metadata.height)),
            ]));

            if let Some(iso) = metadata.iso {
                lines.push(Line::from(vec![
                    Span::styled("ISO: ", Style::default().fg(Color::Gray)),
                    Span::raw(iso.to_string()),
                ]));
            }

            if let Some(ref exp) = metadata.exposure_time {
                lines.push(Line::from(vec![
                    Span::styled("Exposure: ", Style::default().fg(Color::Gray)),
                    Span::raw(exp),
                ]));
            }

            if let Some(ref f_num) = metadata.f_number {
                lines.push(Line::from(vec![
                    Span::styled("F-Number: ", Style::default().fg(Color::Gray)),
                    Span::raw(f_num),
                ]));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "No metadata available",
                Style::default().fg(Color::Yellow),
            )));
        }
    }

    let info = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("File Information"))
        .wrap(Wrap { trim: true });
    f.render_widget(info, chunks[0]);

    // Help
    let help_text = vec![Line::from(vec![
        Span::styled("c: ", Style::default().fg(Color::Gray)),
        Span::raw("Convert | "),
        Span::styled("Esc/q: ", Style::default().fg(Color::Gray)),
        Span::raw("Back"),
    ])];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[1]);
}

fn render_conversion_config(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Configure conversion settings")
        .block(Block::default().borders(Borders::ALL).title("Conversion Settings"));
    f.render_widget(title, chunks[0]);

    // Options
    let config = &app.conversion_config;
    let options = vec![
        format!("Output Format: {}", config.output_format.as_str()),
        format!("Quality: {}", config.quality_display()),
        format!("Preserve Metadata: {}", if config.preserve_metadata { "Yes" } else { "No" }),
        format!("Output Directory: {}", config.output_directory.as_ref().unwrap_or(&"Same as source".to_string())),
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == app.config_option_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if i == app.config_option_index {
                "> "
            } else {
                "  "
            };

            ListItem::new(format!("{}{}", prefix, opt)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Options"));
    f.render_widget(list, chunks[1]);

    // Help
    let help_text = vec![Line::from(vec![
        Span::styled("↑/↓: ", Style::default().fg(Color::Gray)),
        Span::raw("Navigate | "),
        Span::styled("←/→: ", Style::default().fg(Color::Gray)),
        Span::raw("Adjust | "),
        Span::styled("Enter: ", Style::default().fg(Color::Gray)),
        Span::raw("Convert | "),
        Span::styled("Esc: ", Style::default().fg(Color::Gray)),
        Span::raw("Back"),
    ])];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}

fn render_converting(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let filename = app
        .selected_file
        .as_ref()
        .map(|f| f.filename.as_str())
        .unwrap_or("Unknown");

    let info = Paragraph::new(format!("Converting: {}", filename))
        .block(Block::default().borders(Borders::ALL).title("Converting"))
        .alignment(Alignment::Center);
    f.render_widget(info, chunks[0]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(app.conversion_progress as u16);
    f.render_widget(gauge, chunks[1]);
}

fn render_complete(f: &mut Frame, app: &App) {
    let area = f.area();

    let message = app
        .success_message
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Operation completed successfully!");

    let paragraph = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter or Esc to continue",
            Style::default().fg(Color::Gray),
        ))
        .alignment(Alignment::Center),
    ])
    .block(Block::default().borders(Borders::ALL).title("Success"));

    f.render_widget(paragraph, area);
}

fn render_error(f: &mut Frame, app: &App) {
    let area = f.area();

    let message = app
        .error_message
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("An error occurred");

    let paragraph = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter or Esc to continue",
            Style::default().fg(Color::Gray),
        ))
        .alignment(Alignment::Center),
    ])
    .block(Block::default().borders(Borders::ALL).title("Error"))
    .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
