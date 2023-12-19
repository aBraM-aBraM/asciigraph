use anyhow::Result;
use crossterm::event;
use crossterm::event::Event::Key;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::prelude::{CrosstermBackend, Terminal};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use std::io::Stderr;
use ratatui::{
    style::Color,
    widgets::{canvas::*, *},
};
use ratatui::style::Style;
use strum::IntoEnumIterator;
use crate::editor::{Editor, EditorMode};

pub struct App {
    pub should_quit: bool,
    terminal: Terminal<CrosstermBackend<Stderr>>,
    editor: Editor,
}

impl App {
    pub fn new(should_quit: bool,
               terminal: Terminal<CrosstermBackend<Stderr>>,
               editor: Editor) -> App
    {
        App {
            should_quit,
            terminal,
            editor,
        }
    }

    fn select(&mut self)
    {
        if self.editor.selected {} else {
            self.editor.set_last_position();
        }
        self.editor.selected = !self.editor.selected;
    }


    pub fn update(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => self.editor.editor_mode = EditorMode::Explore,
                        KeyCode::Right => self.editor.move_position((1, 0)),
                        KeyCode::Left => self.editor.move_position((-1, 0)),
                        KeyCode::Down => self.editor.move_position((0, 1)),
                        KeyCode::Up => self.editor.move_position((0, -1)),
                        KeyCode::Char(' ') => self.select(),
                        _ => {
                            if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                                match key.code {
                                    KeyCode::Char('r') => self.editor.editor_mode = EditorMode::Rectangle,
                                    KeyCode::Char('t') => self.editor.editor_mode = EditorMode::Text,
                                    KeyCode::Char('a') => self.editor.editor_mode = EditorMode::Arrow,
                                    KeyCode::Char('l') => self.editor.editor_mode = EditorMode::Line,
                                    KeyCode::Char('h') => self.editor.editor_mode = EditorMode::Help,
                                    KeyCode::Char('q') => self.should_quit = true,
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

    fn draw_footer(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            let size = f.size();
            let bottom = size.height - 1;
            let style = Style::default().bg(Color::Magenta);

            f.render_widget(
                Tabs::new(
                    EditorMode::iter()
                        .map(|variant| variant.to_string())
                        .collect::<Vec<String>>(),
                )
                    .select(self.editor.editor_mode as usize)
                    .style(style)
                    .highlight_style(Style::default().bg(Color::Black)),
                Rect::new(0, bottom, f.size().width - 1, 1),
            );

            let pos = self.editor.get_position();
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

    fn draw_shape(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            f.render_widget(Canvas::default()
                                .block(Block::default().title("Canvas").borders(Borders::ALL))
                                .x_bounds([-180.0, 180.0])
                                .y_bounds([-90.0, 90.0])
                                .paint(|ctx| {
                                    ctx.draw(&Map {
                                        resolution: MapResolution::High,
                                        color: Color::White,
                                    });
                                    // ctx.layer();
                                    // ctx.draw(&Line {
                                    //     x1: 0.0,
                                    //     y1: 10.0,
                                    //     x2: 10.0,
                                    //     y2: 10.0,
                                    //     color: Color::White,
                                    // });
                                }), Rect::new(0, 0,
                                              (self.editor.width() - 1) as u16,
                                              (self.editor.height() - 1) as u16),
            )
        })?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.draw_footer()?;
        // self.draw_shape()?;
        Ok(())
    }
}