mod app;
mod editor;

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
        widgets::Paragraph,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    // Editor area
    let text: Vec<ratatui::text::Line> = app
        .buffer
        .lines
        .iter()
        .map(|l| ratatui::text::Line::from(l.as_str()))
        .collect();
    frame.render_widget(Paragraph::new(text), chunks[0]);

    // Status bar
    let mode_str = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };
    frame.render_widget(Paragraph::new(mode_str), chunks[1]);

    // Cursor position
    frame.set_cursor_position((app.buffer.cursor_col as u16, app.buffer.cursor_row as u16));
}

fn handle_events(app: &mut App) -> std::io::Result<bool> {
    use app::Mode;
    use crossterm::event::{self, Event, KeyCode};

    if event::poll(std::time::Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Char('i') => app.mode = Mode::Insert,
                    KeyCode::Left => app.buffer.move_left(),
                    KeyCode::Right => app.buffer.move_right(),
                    KeyCode::Up => app.buffer.move_up(),
                    KeyCode::Down => app.buffer.move_down(),
                    _ => {}
                },
                Mode::Insert => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char(c) => app.buffer.insert_char(c),
                    KeyCode::Enter => app.buffer.insert_newline(),
                    KeyCode::Backspace => app.buffer.delete_char(),
                    KeyCode::Left => app.buffer.move_left(),
                    KeyCode::Right => app.buffer.move_right(),
                    KeyCode::Up => app.buffer.move_up(),
                    KeyCode::Down => app.buffer.move_down(),
                    _ => {}
                },
            }
        }
    }

    Ok(app.should_quit)
}
