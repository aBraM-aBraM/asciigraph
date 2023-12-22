use crate::editor::Editor;


use crossterm::{event, ExecutableCommand};
use std::cmp::min;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use vector2d::Vector2D;

#[derive(Debug)]
pub struct App {
    editor: Editor,
    editor_mode: EditorMode,
    should_quit: bool,
    selected: bool,
    buffer: Vec<Vec<char>>,
}

#[repr(usize)]
#[derive(EnumIter, Display, Copy, Clone, PartialEq, Debug)]
pub enum EditorMode {
    #[strum(serialize = "Explore (Ctrl+Q)")]
    Explore,
    #[strum(serialize = "Rectangle (Ctrl+W)")]
    Rectangle,
    #[strum(serialize = "Text (ctrl+E)")]
    Text,
    #[strum(serialize = "Line (ctrl+A)")]
    Line,
    #[strum(serialize = "Arrow (ctrl+S)")]
    Arrow,
    #[strum(serialize = "Quit (ctrl+C)")]
    Quit,
}

pub fn write_to_screen<T: std::fmt::Display>(
    position: Vector2D<i16>,
    object: T,
    fg: crossterm::style::Color,
) {
    io::stdout()
        .execute(crossterm::cursor::MoveTo(
            position.x as u16,
            position.y as u16,
        ))
        .unwrap()
        .execute(crossterm::style::SetForegroundColor(fg))
        .unwrap()
        .execute(crossterm::style::Print(object))
        .unwrap()
        .execute(crossterm::style::ResetColor)
        .unwrap();
}

impl App {
    pub fn new() -> App {
        let terminal_size = crossterm::terminal::size().unwrap();
        App {
            editor: Editor::new(),
            editor_mode: EditorMode::Explore,
            should_quit: false,
            selected: false,
            buffer: vec![vec![' '; terminal_size.1 as usize]; terminal_size.0 as usize],
        }
    }

    fn borders(&self) -> Vector2D<i16> {
        Vector2D::new(self.buffer.len() as i16, self.buffer[0].len() as i16)
    }

    fn select(&mut self) {
        if self.selected {} else {
            self.editor.set_last_position();
        }
        self.selected = !self.selected;
    }

    pub fn run(&mut self) {
        while !self.should_quit {
            self.draw();
            self.handle_input()
        }
    }

    fn handle_input(&mut self) {
        use crossterm::event;
        use event::Event::Key;
        use event::{KeyCode, KeyModifiers};
        if event::poll(std::time::Duration::from_millis(250)).unwrap() {
            if let Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => self.editor_mode = EditorMode::Explore,
                        KeyCode::Right => self
                            .editor
                            .move_position(Vector2D::new(1, 0), self.borders()),
                        KeyCode::Left => self
                            .editor
                            .move_position(Vector2D::new(-1, 0), self.borders()),
                        KeyCode::Down => self
                            .editor
                            .move_position(Vector2D::new(0, 1), self.borders()),
                        KeyCode::Up => self
                            .editor
                            .move_position(Vector2D::new(0, -1), self.borders()),
                        KeyCode::Char(' ') => self.select(),
                        _ => {
                            if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                                match key.code {
                                    KeyCode::Char('q') => self.editor_mode = EditorMode::Explore,
                                    KeyCode::Char('w') => self.editor_mode = EditorMode::Rectangle,
                                    KeyCode::Char('e') => self.editor_mode = EditorMode::Text,
                                    KeyCode::Char('a') => self.editor_mode = EditorMode::Line,
                                    KeyCode::Char('s') => self.editor_mode = EditorMode::Arrow,
                                    KeyCode::Char('c') => self.should_quit = true,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn preview(&self) {
        if self.selected {
            match self.editor_mode {
                EditorMode::Rectangle => App::preview_rectangle(self.get_current_rectangle()),
                _ => {}
            }
        }
    }

    fn get_current_rectangle(
        &self,
    ) -> [Vector2D<i16>; 4] {
        let dimensions = Vector2D::new(
            (self.editor.get_last_position().x - self.editor.get_position().x).abs(),
            (self.editor.get_last_position().y - self.editor.get_position().y).abs(),
        );
        let anchor = Vector2D::new(
            min(
                self.editor.get_last_position().x,
                self.editor.get_position().x,
            ),
            min(
                self.editor.get_last_position().y,
                self.editor.get_position().y,
            ),
        );
        let top_left = anchor;
        let top_right = anchor + Vector2D::new(dimensions.x, 0);
        let bottom_right = anchor + Vector2D::new(dimensions.x, dimensions.y);
        let bottom_left = anchor + Vector2D::new(0, dimensions.y);

        [top_left, top_right, bottom_right, bottom_left]
    }

    fn preview_rectangle(vertices: [Vector2D<i16>; 4]) {
        let [top_left, top_right, bottom_right, bottom_left] = vertices;

        use crossterm::style;

        for col in top_left.x..bottom_right.x {
            write_to_screen(
                Vector2D::new(col, top_left.y),
                '─',
                style::Color::DarkMagenta,
            );
            write_to_screen(
                Vector2D::new(col, bottom_right.y),
                '─',
                style::Color::DarkMagenta,
            );
        }
        for row in top_left.y..bottom_right.y {
            write_to_screen(
                Vector2D::new(top_left.x, row),
                '│',
                style::Color::DarkMagenta,
            );
            write_to_screen(
                Vector2D::new(bottom_right.x, row),
                '│',
                style::Color::DarkMagenta,
            );
        }
        write_to_screen(top_left, '┌', style::Color::DarkMagenta);
        write_to_screen(top_right, '┐', style::Color::DarkMagenta);
        write_to_screen(bottom_right, '┘', style::Color::DarkMagenta);
        write_to_screen(bottom_left, '└', style::Color::DarkMagenta);
    }

    fn draw(&self) {
        io::stdout()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::All,
            ))
            .unwrap();

        self.draw_footer();
        self.preview();
    }

    fn draw_footer(&self) {
        let mut tabs = EditorMode::iter()
            .map(|variant| {
                if variant == self.editor_mode {
                    format!("* {}", variant)
                } else {
                    variant.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" | ");
        let position = self.editor.get_position();
        let position_string = format!(
            "pos: ({}, {}), last_pos: ({}, {})",
            position.x,
            position.y,
            self.editor.get_last_position().x,
            self.editor.get_last_position().y
        );
        let terminal_size = crossterm::terminal::size().unwrap();
        let spacing: String = " ".repeat(terminal_size.0 as usize - tabs.len() - &position_string.len());
        tabs += spacing.as_str();
        tabs += &position_string;

        use crossterm::style;
        io::stdout()
            .execute(crossterm::cursor::MoveTo(0, terminal_size.1 - 1))
            .unwrap()
            .execute(style::SetForegroundColor(style::Color::White))
            .unwrap()
            .execute(style::SetBackgroundColor(style::Color::DarkMagenta))
            .unwrap()
            .execute(style::Print(tabs))
            .unwrap()
            .execute(style::ResetColor)
            .unwrap();
    }
}
