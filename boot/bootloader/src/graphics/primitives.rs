#![allow(dead_code)]

use crate::graphics::framebuffer::draw_pixel;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn fill_screen(framebuffer: usize, width: u32, color: Color) {
    for i in 0..(width * 200) {
        draw_pixel(framebuffer, i, 100, width, color);
    }
}
