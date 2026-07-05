#![allow(dead_code)]

pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct Window {
    pub id: u32,
    pub rect: Rect,
    pub z_index: u32,
    pub title: &'static str,
}

pub fn create_window(id: u32, x: u32, y: u32, w: u32, h: u32, title: &'static str) -> Window {
    Window {
        id,
        rect: Rect { x, y, w, h },
        z_index: 0,
        title,
    }
}
