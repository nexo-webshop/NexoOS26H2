#![allow(dead_code)]

pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub visible: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            visible: true,
        }
    }

    pub fn move_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}
