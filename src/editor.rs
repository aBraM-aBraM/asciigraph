use std::cmp::{max, min};
use strum_macros::{Display, EnumIter};


#[repr(usize)]
#[derive(EnumIter, Display, Copy, Clone)]
pub enum EditorMode {
    #[strum(serialize = "Explore (Escape)")]
    Explore,
    #[strum(serialize = "Choose (Space)")]
    Choose,
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
    buffer: Vec<Vec<char>>,
    current_position: (i16, i16),
    last_position: (i16, i16),
}

impl Editor {
    pub fn move_position(&mut self, offset: (i16, i16)) {
        self.current_position.0 += offset.0;
        self.current_position.1 += offset.1;

        let width = self.buffer.len();
        let height = self.buffer[0].len();

        self.current_position.0 = min(max(self.current_position.0, 0), width as i16);
        self.current_position.1 = min(max(self.current_position.1, 0), height as i16);
    }
    pub fn get_position(&self) -> (i16, i16) {
        self.current_position.clone()
    }

    pub fn new(
        buffer: Vec<Vec<char>>,
        current_position: (i16, i16),
        last_position: (i16, i16),
    ) -> Editor {
        return Editor {
            editor_mode: EditorMode::Explore,
            buffer,
            current_position,
            last_position,
        };
    }
}
