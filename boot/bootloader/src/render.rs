#![allow(dead_code)]

use crate::graphics::{draw_pixel, Color};
use crate::window::Rect;

pub fn draw_rect(
    framebuffer: usize,
    rect: Rect,
    width: u32,
    color: Color,
) {
    // simpele outline renderer

    for x in rect.x..rect.x + rect.w {
        draw_pixel(framebuffer, x, rect.y, width, color);
        draw_pixel(framebuffer, x, rect.y + rect.h, width, color);
    }

    for y in rect.y..rect.y + rect.h {
        draw_pixel(framebuffer, rect.x, y, width, color);
        draw_pixel(framebuffer, rect.x + rect.w, y, width, color);
    }
}
