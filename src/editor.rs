use std::cmp::{max, min};

pub struct Editor {
    current_position: (i16, i16),
    last_position: (i16, i16),
}

impl Editor {
    pub fn move_position(&mut self, offset: (i16, i16), borders: (i16, i16)) {
        self.current_position.0 += offset.0;
        self.current_position.1 += offset.1;

        self.current_position.0 = min(max(self.current_position.0, 0), borders.0);
        self.current_position.1 = min(max(self.current_position.1, 0), borders.1);
    }

    pub fn set_last_position(&mut self) {
        self.last_position.0 = self.current_position.0;
        self.last_position.1 = self.current_position.1;
    }

    pub fn get_position(&self) -> (i16, i16) {
        self.current_position
    }

    pub fn new() -> Editor {
        Editor {
            current_position: (0, 0),
            last_position: (0, 0),
        }
    }
}
