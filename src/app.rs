use crate::editor::Editor;

use crossterm::{event, style, ExecutableCommand};
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

pub fn write_to_screen<T: std::fmt::Display>(position: Vector2D<i16>, object: T, fg: style::Color) {
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

pub fn preview_write_to_screen<T: std::fmt::Display>(position: Vector2D<i16>, object: T) {
    write_to_screen(position, object, style::Color::Magenta);
}

impl App {
    pub fn new() -> App {
        let terminal_size = crossterm::terminal::size().unwrap();
        let terminal_width = terminal_size.0 as i16;
        let terminal_height = (terminal_size.1 - 1) as i16; // leave space for footer
        App {
            editor: Editor::new(Vector2D::new(terminal_width / 7,
                                              terminal_height / 2)),
            editor_mode: EditorMode::Explore,
            should_quit: false,
            selected: false,
            buffer: vec![vec![' '; terminal_width as usize]; terminal_height as usize],
        }
    }

    fn borders(&self) -> Vector2D<i16> {
        Vector2D::new(self.buffer[0].len() as i16, self.buffer.len() as i16)
    }

    fn select(&mut self) {
        let last_position = self.editor.get_last_position();
        let curr_position = self.editor.get_position();

        if self.selected {
            let mut write_to_buff = |position: Vector2D<i16>, object: char| {
                let buff = &mut self.buffer;
                buff[position.y as usize][position.x as usize] = object;
            };
            match self.editor_mode {
                EditorMode::Rectangle => App::write_rectangle(
                    App::get_rect_vertices(last_position, curr_position),
                    &mut write_to_buff,
                ),
                _ => {}
            }
        } else {
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

    fn preview(&mut self) {
        let last_position = self.editor.get_last_position();
        let curr_position = self.editor.get_position();

        if self.selected {
            match self.editor_mode {
                EditorMode::Rectangle => App::write_rectangle(
                    App::get_rect_vertices(last_position, curr_position),
                    &mut preview_write_to_screen,
                ),
                EditorMode::Explore => preview_write_to_screen(curr_position, "*"),
                _ => {}
            }
        } else {
            preview_write_to_screen(curr_position, "*");
        }
    }

    fn write_rectangle(
        vertices: [Vector2D<i16>; 4],
        write_func: &mut dyn FnMut(Vector2D<i16>, char),
    ) {
        let [top_left, top_right, bottom_right, bottom_left] = vertices;
        for col in top_left.x..bottom_right.x {
            write_func(Vector2D::new(col, top_left.y), '─');
            write_func(Vector2D::new(col, bottom_right.y), '─');
        }
        for row in top_left.y..bottom_right.y {
            write_func(Vector2D::new(top_left.x, row), '│');
            write_func(Vector2D::new(bottom_right.x, row), '│');
        }
        write_func(top_left, '┌');
        write_func(top_right, '┐');
        write_func(bottom_right, '┘');
        write_func(bottom_left, '└');
    }

    fn get_rect_vertices(vertex1: Vector2D<i16>, vertex2: Vector2D<i16>) -> [Vector2D<i16>; 4] {
        let dimensions =
            Vector2D::new((vertex1.x - vertex2.x).abs(), (vertex1.y - vertex2.y).abs());
        let anchor = Vector2D::new(min(vertex1.x, vertex2.x), min(vertex1.y, vertex2.y));
        let top_left = anchor;
        let top_right = anchor + Vector2D::new(dimensions.x, 0);
        let bottom_right = anchor + Vector2D::new(dimensions.x, dimensions.y);
        let bottom_left = anchor + Vector2D::new(0, dimensions.y);

        [top_left, top_right, bottom_right, bottom_left]
    }

    fn draw(&mut self) {
        io::stdout()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::All,
            ))
            .unwrap();

        self.draw_footer();
        self.draw_buffer();
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
        let spacing: String =
            " ".repeat(terminal_size.0 as usize - tabs.len() - &position_string.len());
        tabs += spacing.as_str();
        tabs += &position_string;

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
    fn draw_buffer(&self) {
        for (i, col) in self.buffer.iter().enumerate() {
            let buff_line: String = col.into_iter().collect();
            io::stdout()
                .execute(crossterm::cursor::MoveTo(0, i as u16))
                .unwrap()
                .execute(style::Print(buff_line))
                .unwrap();
        }
    }
}
