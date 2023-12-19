use std::cmp::{max, min};
use strum_macros::{Display, EnumIter};


#[repr(usize)]
#[derive(EnumIter, Display, Copy, Clone)]
pub enum EditorMode {
    #[strum(serialize = "Explore (Escape)")]
    Explore,
    #[strum(serialize = "Rectangle (ctrl+R)")]
    Rectangle,
    #[strum(serialize = "Text (ctrl+T)")]
    Text,
    #[strum(serialize = "Line (ctrl+L)")]
    Line,
    #[strum(serialize = "Arrow (ctrl+A)")]
    Arrow,
    #[strum(serialize = "Help (ctrl+H)")]
    Help,
    #[strum(serialize = "Quit (ctrl+Q)")]
    Quit,
}


pub struct Editor {
    pub editor_mode: EditorMode,
    pub selected: bool,
    buffer: Vec<Vec<char>>,
    current_position: (i16, i16),
    last_position: (i16, i16),
}

impl Editor {
    pub fn move_position(&mut self, offset: (i16, i16)) {
        self.current_position.0 += offset.0;
        self.current_position.1 += offset.1;

        self.current_position.0 = min(max(self.current_position.0, 0), self.width() as i16);
        self.current_position.1 = min(max(self.current_position.1, 0), self.height() as i16);
    }

    pub fn width(&self) -> usize {
        self.buffer.len()
    }

    pub fn height(&self) -> usize {
        self.buffer[0].len()
    }


    pub fn set_last_position(&mut self) {
        self.last_position.0 = self.current_position.0;
        self.last_position.1 = self.current_position.1;
    }

    pub fn get_position(&self) -> (i16, i16) {
        self.current_position
    }

    pub fn new(buffer: Vec<Vec<char>>,
               current_position: (i16, i16),
               last_position: (i16, i16)) -> Editor {
        Editor {
            editor_mode: EditorMode::Explore,
            selected: false,
            buffer,
            current_position,
            last_position,
        }
    }
}
