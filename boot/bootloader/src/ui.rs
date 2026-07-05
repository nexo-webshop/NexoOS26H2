#![allow(dead_code)]

use crate::graphics::{draw_pixel, Color};
use crate::font::get_char_bitmap;

pub fn draw_char(
    framebuffer: usize,
    x: u32,
    y: u32,
    width: u32,
    c: char,
    color: Color,
) {
    let bitmap = get_char_bitmap(c);

    for row in 0..8 {
        let line = bitmap[row];

        for col in 0..8 {
            if (line >> (7 - col)) & 1 == 1 {
                draw_pixel(
                    framebuffer,
                    x + col,
                    y + row,
                    width,
                    color,
                );
            }
        }
    }
}

pub fn draw_text(
    framebuffer: usize,
    mut x: u32,
    y: u32,
    width: u32,
    text: &str,
    color: Color,
) {
    for c in text.chars() {
        draw_char(framebuffer, x, y, width, c, color);
        x += 8;
    }
}
