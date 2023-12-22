use crate::editor::Editor;
use crossterm::ExecutableCommand;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub struct App {
    editor: Editor,
    editor_mode: EditorMode,
    should_quit: bool,
    selected: bool,
    buffer: Vec<Vec<char>>,
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
    #[strum(serialize = "Arrow (ctrl+S)")]
    Arrow,
    #[strum(serialize = "Quit (ctrl+C)")]
    Quit,
}

pub fn write_to_screen(
    position: (u16, u16),
    char: char,
    bg: crossterm::style::Color,
    fg: crossterm::style::Color,
) {
    io::stdout()
        .execute(crossterm::cursor::MoveTo(position.0, position.1))
        .unwrap()
        .execute(crossterm::style::SetForegroundColor(fg))
        .unwrap()
        .execute(crossterm::style::SetBackgroundColor(bg))
        .unwrap()
        .execute(crossterm::style::Print(char))
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

    fn borders(&self) -> (i16, i16) {
        (self.buffer.len() as i16, self.buffer[0].len() as i16)
    }

    fn width(&self) -> usize {
        self.buffer.len()
    }
    fn height(&self) -> usize {
        self.buffer[0].len()
    }

    fn select(&mut self) {
        if self.selected {
        } else {
            self.editor.set_last_position();
        }
        self.selected = !self.selected;
    }

    pub fn run(&mut self) {
        loop {
            use crossterm::event;
            use event::Event::Key;
            use event::{KeyCode, KeyModifiers};
            self.draw_footer();
            if event::poll(std::time::Duration::from_millis(250)).unwrap() {
                if let Key(key) = event::read().unwrap() {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => self.editor_mode = EditorMode::Explore,
                            KeyCode::Right => self.editor.move_position((1, 0), self.borders()),
                            KeyCode::Left => self.editor.move_position((-1, 0), self.borders()),
                            KeyCode::Down => self.editor.move_position((0, 1), self.borders()),
                            KeyCode::Up => self.editor.move_position((0, -1), self.borders()),
                            KeyCode::Char(' ') => self.select(),
                            _ => {
                                if key.modifiers & KeyModifiers::CONTROL == KeyModifiers::CONTROL {
                                    match key.code {
                                        KeyCode::Char('q') => {
                                            self.editor_mode = EditorMode::Explore
                                        }
                                        KeyCode::Char('w') => {
                                            self.editor_mode = EditorMode::Rectangle
                                        }
                                        KeyCode::Char('e') => self.editor_mode = EditorMode::Text,
                                        KeyCode::Char('a') => self.editor_mode = EditorMode::Line,
                                        KeyCode::Char('s') => self.editor_mode = EditorMode::Arrow,
                                        KeyCode::Char('c') => break,
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn preview(&self) {}

    fn draw_footer(&self) {
        let mut tabs = EditorMode::iter()
            .map(|variant| {
                if variant == self.editor_mode {
                    format!("* {}", variant.to_string())
                } else {
                    variant.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" | ");
        let position = self.editor.get_position();
        let position_string = format!("pos: ({}, {})", position.0, position.1);
        let terminal_size = crossterm::terminal::size().unwrap();
        let spacing: String = std::iter::repeat(' ')
            .take(terminal_size.0 as usize - tabs.len() - &position_string.len())
            .collect();
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
