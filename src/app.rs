use crossterm::{event, style, ExecutableCommand};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{stdin, Write};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use vector2d::Vector2D;

pub struct App {
    last_position: Vector2D<i16>,
    curr_position: Vector2D<i16>,
    editor_mode: EditorMode,
    selected: bool,
    line_alignment: bool,

    should_quit: bool,
    app_mode: AppMode,
    buffer: Vec<Vec<char>>,
}

enum AppMode {
    Editor,
    Command,
}

#[repr(usize)]
#[derive(EnumIter, Display, Copy, Clone, PartialEq)]
pub enum EditorMode {
    #[strum(serialize = "Explore (Ctrl+Q)")]
    Explore,
    #[strum(serialize = "Rectangle (Ctrl+W)")]
    Rectangle,
    #[strum(serialize = "Text (ctrl+E)")]
    Text,
    #[strum(serialize = "Line (ctrl+A)")]
    Line,
    #[strum(serialize = "Arrow (ctrl+D)")]
    Arrow,
    #[strum(serialize = "Rotate Line/Arrow (ctrl+R)")]
    Rotate,
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
        .execute(style::SetForegroundColor(fg))
        .unwrap()
        .execute(style::Print(object))
        .unwrap()
        .execute(style::ResetColor)
        .unwrap();
}

pub fn preview_write_to_screen<T: std::fmt::Display>(position: Vector2D<i16>, object: T) {
    write_to_screen(position, object, style::Color::Magenta);
}

impl App {
    pub fn new() -> App {
        let terminal_size = crossterm::terminal::size().unwrap();
        let terminal_width = terminal_size.0 as i16;
        let terminal_height = (terminal_size.1 - 2) as i16; // leave space for footer
        App {
            curr_position: Vector2D::new(terminal_width / 7, terminal_height / 2),
            last_position: Vector2D::new(0, 0),
            editor_mode: EditorMode::Explore,
            selected: false,
            line_alignment: true,

            should_quit: false,
            app_mode: AppMode::Editor,
            buffer: vec![vec![' '; terminal_width as usize]; terminal_height as usize],
        }
    }

    fn move_position(&mut self, offset: Vector2D<i16>) {
        self.curr_position.x += offset.x;
        self.curr_position.y += offset.y;
        let borders = self.borders();

        self.curr_position.x = min(max(self.curr_position.x, 0), borders.x);
        self.curr_position.y = min(max(self.curr_position.y, 0), borders.y);
    }

    fn borders(&self) -> Vector2D<i16> {
        Vector2D::new(self.buffer[0].len() as i16, self.buffer.len() as i16)
    }

    fn select(&mut self) {
        if self.selected {
            let mut write_to_buff = |position: Vector2D<i16>, object: char| {
                let buff = &mut self.buffer;
                buff[position.y as usize][position.x as usize] = object;
            };
            App::editor_write(self.editor_mode, [self.curr_position, self.last_position], &mut write_to_buff, self.line_alignment);
        } else {
            self.last_position = self.curr_position;
        }
        self.selected = !self.selected;
    }

    pub fn run(&mut self) {
        self.draw();
        while !self.should_quit {
            self.handle_input()
        }
    }

    fn handle_command_input(&mut self) {
        let filename_pos = Vector2D::new(0, self.borders().y);

        crossterm::terminal::disable_raw_mode().unwrap();
        io::stdout().
            execute(crossterm::cursor::Show)
            .unwrap()
            .execute(crossterm::cursor::MoveTo(filename_pos.x as u16, filename_pos.y as u16))
            .unwrap();

        let mut readline = String::new();
        stdin().read_line(&mut readline).unwrap();

        let mut file = File::create(readline.trim()).unwrap();
        for col in &self.buffer {
            let buff_line: String = col.iter().collect();
            assert_eq!(file.write(format!("{}\n", buff_line).as_bytes()).unwrap(), buff_line.len() + 1,
                       "failed to save");
        }

        crossterm::terminal::enable_raw_mode().unwrap();
        io::stdout().execute(crossterm::cursor::Hide).unwrap();
    }

    fn handle_editor_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Right => self
                .move_position(Vector2D::new(1, 0)),
            KeyCode::Left => self
                .move_position(Vector2D::new(-1, 0)),
            KeyCode::Down => self
                .move_position(Vector2D::new(0, 1)),
            KeyCode::Up => self
                .move_position(Vector2D::new(0, -1)),
            _ => {
                if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('q') => self.editor_mode = EditorMode::Explore,
                        KeyCode::Char('w') => self.editor_mode = EditorMode::Rectangle,
                        KeyCode::Char('e') => self.editor_mode = EditorMode::Text,
                        KeyCode::Char('a') => self.editor_mode = EditorMode::Line,
                        KeyCode::Char('d') => self.editor_mode = EditorMode::Arrow,
                        KeyCode::Char('r') => self.line_alignment = !self.line_alignment,
                        _ => {}
                    }
                } else if self.editor_mode == EditorMode::Text {
                    if let KeyCode::Char(c) = key.code {
                        self.move_position(Vector2D::new(1, 0));
                        self.buffer[self.curr_position.y as usize][self.curr_position.x as usize] = c;
                    } else if key.code == KeyCode::Backspace {
                        self.buffer[self.curr_position.y as usize][self.curr_position.x as usize] = ' ';
                        self.move_position(Vector2D::new(-1, 0));
                    }
                } else if key.code == KeyCode::Char(' ') { self.select() }
            }
        }
    }

    fn handle_input(&mut self) {
        if event::poll(std::time::Duration::from_millis(250)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Press {
                    if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                        if key.code == KeyCode::Char('c') {
                            self.should_quit = true;
                        }
                        if key.code == KeyCode::Char('s') {
                            self.app_mode = AppMode::Command;
                        }
                    }

                    match self.app_mode {
                        AppMode::Editor => self.handle_editor_input(key),
                        AppMode::Command => self.handle_command_input(),
                    }
                    self.draw();
                }
            }
        }
    }

    fn editor_write(editor_mode: EditorMode, vertices: [Vector2D<i16>; 2], write_func: &mut dyn FnMut(Vector2D<i16>, char), line_alignment: bool) {
        let [current, last] = vertices;
        match editor_mode {
            EditorMode::Rectangle => App::write_rectangle(
                App::get_rect_vertices(last, current),
                write_func,
            ),
            EditorMode::Line => App::write_line(
                App::get_line_vertices(last, current, line_alignment),
                write_func,
            ),
            EditorMode::Arrow => App::write_arrow(
                App::get_line_vertices(last, current, line_alignment),
                write_func,
            ),
            EditorMode::Explore => write_func(current, '*'),
            _ => {}
        }
    }

    fn preview(&mut self) {
        if self.selected {
            App::editor_write(self.editor_mode, [self.curr_position, self.last_position], &mut preview_write_to_screen, self.line_alignment);
        } else if self.editor_mode != EditorMode::Text {
            preview_write_to_screen(self.curr_position, "*");
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


    fn hash_vec2d(vector2d: Vector2D<i16>) -> i16 {
        vector2d.x * 0x10 + vector2d.y // works only for normalized integer vectors
    }

    fn write_line(
        vertices: [Vector2D<i16>; 4],
        write_func: &mut dyn FnMut(Vector2D<i16>, char),
    ) {
        let [vertical, connector, horizontal, _] = vertices;


        let corner_char_map = HashMap::from([
            (App::hash_vec2d(Vector2D::new(1i16, 1i16)), '┌'),
            (App::hash_vec2d(Vector2D::new(-1i16, 1i16)), '┐'),
            (App::hash_vec2d(Vector2D::new(1i16, -1i16)), '└'),
            (App::hash_vec2d(Vector2D::new(-1i16, -1i16)), '┘'),
        ]);
        let corner_vec = Vector2D::new((horizontal.x - connector.x).signum(),
                                       (vertical.y - connector.y).signum());
        let col_vec = if corner_vec.x > 0 {
            connector.x..horizontal.x
        } else { horizontal.x..connector.x };
        let row_vec = if corner_vec.y > 0 {
            connector.y..vertical.y
        } else { vertical.y..connector.y };


        for col in col_vec {
            write_func(Vector2D::new(col, connector.y), '─');
        }
        for row in row_vec {
            write_func(Vector2D::new(connector.x, row), '│');
        }

        let corner_vec_hash = App::hash_vec2d(corner_vec);
        if corner_char_map.get(&corner_vec_hash).is_some() {
            write_func(connector, corner_char_map[&corner_vec_hash]);
        }
    }

    fn write_arrow(
        vertices: [Vector2D<i16>; 4],
        write_func: &mut dyn FnMut(Vector2D<i16>, char),
    ) {
        App::write_line(vertices, write_func);
        let pointer_char_map = HashMap::from([
            (App::hash_vec2d(Vector2D::new(1i16, 0i16)), '►'),
            (App::hash_vec2d(Vector2D::new(-1i16, 0i16)), '◄'),
            (App::hash_vec2d(Vector2D::new(0i16, 1i16)), '▼'),
            (App::hash_vec2d(Vector2D::new(0i16, -1i16)), '▲'),
        ]);
        let [_, connector, _, dir] = vertices;
        let dir_vec = dir - connector;
        let dir_vec = Vector2D::new(dir_vec.x.signum(), dir_vec.y.signum());
        let dir_vec_hash = App::hash_vec2d(dir_vec);

        if pointer_char_map.get(&dir_vec_hash).is_some() {
            write_func(dir, pointer_char_map[&dir_vec_hash]);
        }
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

    fn get_line_vertices(vertex1: Vector2D<i16>, vertex2: Vector2D<i16>, horizontal: bool) -> [Vector2D<i16>; 4]
    {
        // [vertical_vertex, connector, horizontal_vertex, direction_vertex]
        if horizontal {
            [vertex2, Vector2D::new(vertex2.x, vertex1.y), vertex1, vertex2]
        } else {
            [vertex1, Vector2D::new(vertex1.x, vertex2.y), vertex2, vertex2]
        }
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
        let position = self.curr_position;
        let position_string = format!(
            "pos: ({}, {}), last_pos: ({}, {})",
            position.x,
            position.y,
            self.last_position.x,
            self.last_position.y
        );
        let terminal_size = crossterm::terminal::size().unwrap();
        let spacing: String =
            " ".repeat(terminal_size.0 as usize - tabs.len() - position_string.len());
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
            let buff_line: String = col.iter().collect();
            io::stdout()
                .execute(crossterm::cursor::MoveTo(0, i as u16))
                .unwrap()
                .execute(style::Print(buff_line))
                .unwrap();
        }
    }
}
