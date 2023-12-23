use std::cmp::{max, min};
use vector2d::Vector2D;

#[derive(Debug)]
pub struct Editor {
    current_position: Vector2D<i16>,
    last_position: Vector2D<i16>,
}

impl Editor {
    pub fn move_position(&mut self, offset: Vector2D<i16>, borders: Vector2D<i16>) {
        self.current_position.x += offset.x;
        self.current_position.y += offset.y;

        self.current_position.x = min(max(self.current_position.x, 0), borders.x);
        self.current_position.y = min(max(self.current_position.y, 0), borders.y);
    }

    pub fn set_last_position(&mut self) {
        self.last_position = self.current_position;
    }

    pub fn get_position(&self) -> Vector2D<i16> {
        self.current_position
    }
    pub fn get_last_position(&self) -> Vector2D<i16> {
        self.last_position
    }

    pub fn new(start_position: Vector2D<i16>) -> Editor {
        Editor {
            current_position: start_position,
            last_position: Vector2D::new(0, 0),
        }
    }
}
