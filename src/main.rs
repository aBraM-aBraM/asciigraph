mod editor;

use anyhow::Result;
use crossterm::event;
use crossterm::event::Event::Key;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::prelude::{CrosstermBackend, Terminal};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Tabs};
use std::io::Stderr;
use strum::IntoEnumIterator;
use editor::{Editor, EditorMode};

struct App {
    should_quit: bool,
    size: Rect,
    editor: Editor,
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => app.editor.editor_mode = EditorMode::Explore,
                    KeyCode::Right => app.editor.move_position((1, 0)),
                    KeyCode::Left => app.editor.move_position((-1, 0)),
                    KeyCode::Down => app.editor.move_position((0, 1)),
                    KeyCode::Up => app.editor.move_position((0, -1)),
                    KeyCode::Char(' ') => app.editor.editor_mode = EditorMode::Choose,
                    _ => {
                        if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                            match key.code {
                                KeyCode::Char('r') => app.editor.editor_mode = EditorMode::Rectangle,
                                KeyCode::Char('t') => app.editor.editor_mode = EditorMode::Text,
                                KeyCode::Char('a') => app.editor.editor_mode = EditorMode::Arrow,
                                KeyCode::Char('l') => app.editor.editor_mode = EditorMode::Line,
                                KeyCode::Char('h') => app.editor.editor_mode = EditorMode::Help,
                                KeyCode::Char('q') => app.should_quit = true,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn draw_footer(app: &mut App, terminal: &mut Terminal<CrosstermBackend<Stderr>>) -> Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        let bottom = size.height - 1;
        let style = Style::default().bg(Color::Magenta);
        f.render_widget(
            Tabs::new(
                EditorMode::iter()
                    .map(|variant| variant.to_string())
                    .collect::<Vec<String>>(),
            )
                .select(app.editor.editor_mode as usize)
                .style(style)
                .highlight_style(Style::default().bg(Color::Black)),
            Rect::new(0, bottom, f.size().width - 1, 1),
        );
        let pos = app.editor.get_position();
        let pos_str = format!("pos ({}, {})", pos.0, pos.1);
        let pos_str_length = pos_str.len();
        f.render_widget(
            Paragraph::new(pos_str).style(style),
            Rect::new(
                f.size().width - pos_str_length as u16 - 1,
                bottom,
                pos_str_length as u16,
                1,
            ),
        );
    })?;
    Ok(())
}

fn draw(app: &mut App, terminal: &mut Terminal<CrosstermBackend<Stderr>>) -> Result<()> {
    draw_footer(app, terminal)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let terminal_size = terminal.size()?;
    let mut app = App {
        should_quit: false,
        size: terminal_size,
        editor: editor::Editor::new(
            vec![vec![' '; terminal_size.width as usize]; terminal_size.height as usize],
            (0, 0),
            (0, 0),
        ),
    };

    loop {
        draw(&mut app, &mut terminal)?;
        update(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
