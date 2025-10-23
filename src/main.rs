mod gpr;
mod models;
mod ui;
mod utils;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

use ui::app::{App, AppState};

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render::render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.state {
                    AppState::MainMenu => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Up | KeyCode::Char('k') => app.previous_menu_item(),
                            KeyCode::Down | KeyCode::Char('j') => app.next_menu_item(),
                            KeyCode::Enter => app.select_menu_item(),
                            _ => {}
                        }
                    }
                    AppState::FileBrowser => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.back_to_main_menu(),
                            KeyCode::Up | KeyCode::Char('k') => app.previous_file(),
                            KeyCode::Down | KeyCode::Char('j') => app.next_file(),
                            KeyCode::Enter => app.select_file(),
                            KeyCode::Backspace => app.go_to_parent_directory(),
                            _ => {}
                        }
                    }
                    AppState::FileInfo => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.back_to_file_browser(),
                            KeyCode::Char('c') => app.go_to_conversion_config(),
                            _ => {}
                        }
                    }
                    AppState::ConversionConfig => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.back_to_file_info(),
                            KeyCode::Up | KeyCode::Char('k') => app.previous_config_option(),
                            KeyCode::Down | KeyCode::Char('j') => app.next_config_option(),
                            KeyCode::Left | KeyCode::Char('h') => app.adjust_config_option(-1),
                            KeyCode::Right | KeyCode::Char('l') => app.adjust_config_option(1),
                            KeyCode::Enter => app.start_conversion(),
                            _ => {}
                        }
                    }
                    AppState::Converting => {
                        match key.code {
                            KeyCode::Char('q') => app.cancel_conversion(),
                            _ => {}
                        }
                    }
                    AppState::Complete => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                                app.back_to_main_menu()
                            }
                            _ => {}
                        }
                    }
                    AppState::Error => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                                app.back_to_main_menu()
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
