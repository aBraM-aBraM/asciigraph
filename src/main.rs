use anyhow::Result;
use crossterm::event;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode;
use crossterm::event::KeyCode::Char;
use ratatui::layout::Rect;
use ratatui::prelude::{CrosstermBackend, Terminal};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Tabs};
use std::cmp::min;
use std::io::Stderr;

struct App {
    selected: SelectedTab,
    editor_mode: EditorMode,
    should_quit: bool,
    size: Rect,
    x: u16,
    y: u16,
}

#[repr(usize)]
#[derive(Copy, Clone)]
enum SelectedTab {
    Editor,
    Help = 6,
}

#[derive(Copy, Clone)]
enum EditorMode {
    Explore,
    Choose,
    Rectangle,
    Text,
    Line,
    Arrow,
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        app.selected = SelectedTab::Editor;
                        app.editor_mode = EditorMode::Explore;
                    }
                    Char('r') => app.editor_mode = EditorMode::Rectangle,
                    Char('t') => app.editor_mode = EditorMode::Text,
                    Char('a') => app.editor_mode = EditorMode::Arrow,
                    Char('l') => app.editor_mode = EditorMode::Line,
                    Char(' ') => app.editor_mode = EditorMode::Choose,
                    KeyCode::Right => app.x = min(app.x + 1, app.size.width),
                    KeyCode::Left => {
                        if app.x > 0 {
                            app.x -= 1;
                        }
                    }
                    KeyCode::Down => app.y = min(app.y + 1, app.size.height),
                    KeyCode::Up => {
                        if app.y > 0 {
                            app.y -= 1;
                        }
                    }
                    Char('?') => app.selected = SelectedTab::Help,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn draw(app: &mut App, terminal: &mut Terminal<CrosstermBackend<Stderr>>) -> Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        let bottom = size.height - 1;
        let style = Style::default().bg(Color::Magenta);
        f.render_widget(
            Tabs::new(vec![
                "editor (esc)",
                "choose (space)",
                "rectangle (r)",
                "text (t)",
                "line (l)",
                "arrow (a)",
                "help (?)",
                "quit (q)",
            ])
                .select(app.selected.clone() as usize)
                .style(style)
                .highlight_style(Style::default().bg(Color::Black)),
            Rect::new(0, bottom, f.size().width - 1, 1),
        );
        let pos_str = format!("pos ({}, {})", app.x, app.y);
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut app = App {
        selected: SelectedTab::Editor,
        editor_mode: EditorMode::Explore,
        should_quit: false,
        size: terminal.size()?,
        x: 0,
        y: 0,
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
