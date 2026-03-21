mod app;
mod editor;
mod notes;

use app::App;
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io::stdout;

fn main() -> std::io::Result<()> {
    // Setup
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    notes::ensure_notes_dir()?;

    // Event loop
    loop {
        terminal.draw(|frame| {
            ui(frame, &app);
        })?;
        if handle_events(&mut app)? {
            break;
        }
    }

    // Teardown
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    use app::Mode;
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    match app.mode {
        Mode::FilePicker => {
            let items: Vec<ListItem> = app
                .notes
                .iter()
                .enumerate()
                .map(|(i, path)| {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    let style = if i == app.selected_note {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    };
                    ListItem::new(name).style(style)
                })
                .collect();

            let list =
                List::new(items).block(Block::default().borders(Borders::ALL).title(" Notes "));
            frame.render_widget(list, chunks[0]);
            frame.render_widget(
                Paragraph::new(" ↑↓ navigate   Enter open   Esc cancel"),
                chunks[1],
            );
        }
        _ => {
            // Editor area
            let text: Vec<ratatui::text::Line> = app
                .buffer
                .lines
                .iter()
                .map(|l| ratatui::text::Line::from(l.as_str()))
                .collect();
            frame.render_widget(Paragraph::new(text), chunks[0]);

            // Status bar — changes based on mode
            let status = match app.mode {
                Mode::Saving => format!("Save as: {}_", app.save_input),
                _ => {
                    let mode_str = match app.mode {
                        Mode::Normal => "NORMAL",
                        Mode::Insert => "INSERT",
                        _ => unreachable!(),
                    };
                    let dirty_marker = if app.buffer.dirty { " [+]" } else { "" };
                    let file_name = app.buffer.file_path.as_deref().unwrap_or("untitled");
                    format!("{} | {}{}", file_name, mode_str, dirty_marker)
                }
            };
            frame.render_widget(Paragraph::new(status), chunks[1]);

            frame.set_cursor_position((app.buffer.cursor_col as u16, app.buffer.cursor_row as u16));
        }
    }
}

fn handle_events(app: &mut App) -> std::io::Result<bool> {
    use app::Mode;
    use crossterm::event::{self, Event, KeyCode, KeyModifiers};

    if event::poll(std::time::Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Char('i') => app.mode = Mode::Insert,
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if app.buffer.file_path.is_some() {
                            app.buffer.save()?;
                            app.buffer.dirty = false;
                        } else {
                            app.save_input.clear();
                            app.mode = Mode::Saving;
                        }
                    }
                    KeyCode::Char(' ') => {
                        app.refresh_notes()?;
                        app.mode = Mode::FilePicker;
                    }
                    KeyCode::Left => app.buffer.move_left(),
                    KeyCode::Right => app.buffer.move_right(),
                    KeyCode::Up => app.buffer.move_up(),
                    KeyCode::Down => app.buffer.move_down(),
                    _ => {}
                },
                Mode::Insert => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if app.buffer.file_path.is_some() {
                            app.buffer.save()?;
                            app.buffer.dirty = false;
                        } else {
                            app.save_input.clear();
                            app.mode = Mode::Saving;
                        }
                    }
                    KeyCode::Char(c) => app.buffer.insert_char(c),
                    KeyCode::Enter => app.buffer.insert_newline(),
                    KeyCode::Backspace => app.buffer.delete_char(),
                    KeyCode::Left => app.buffer.move_left(),
                    KeyCode::Right => app.buffer.move_right(),
                    KeyCode::Up => app.buffer.move_up(),
                    KeyCode::Down => app.buffer.move_down(),
                    _ => {}
                },
                Mode::FilePicker => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Up => {
                        if app.selected_note > 0 {
                            app.selected_note -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.selected_note < app.notes.len().saturating_sub(1) {
                            app.selected_note += 1;
                        }
                    }
                    KeyCode::Enter => app.open_selected_note()?,
                    _ => {}
                },
                Mode::Saving => match key.code {
                    KeyCode::Esc => {
                        app.save_input.clear();
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Enter => app.confirm_save()?,
                    KeyCode::Backspace => {
                        app.save_input.pop();
                    }
                    KeyCode::Char(c) => app.save_input.push(c),
                    _ => {}
                },
            }
        }
    }

    Ok(app.should_quit)
}
