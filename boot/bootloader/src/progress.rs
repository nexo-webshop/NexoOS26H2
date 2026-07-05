#![allow(dead_code)]

use crate::graphics::{draw_pixel, Color};

pub fn draw_progress_bar(
    framebuffer: usize,
    x: u32,
    y: u32,
    width: u32,
    progress: u32,
    max: u32,
) {
    let bar_width = 200;
    let filled = (progress * bar_width) / max;

    let bg = Color { r: 50, g: 50, b: 50 };
    let fg = Color { r: 0, g: 200, b: 255 };

    // background
    for i in 0..bar_width {
        draw_pixel(framebuffer, x + i, y, width, bg);
    }

    // foreground
    for i in 0..filled {
        draw_pixel(framebuffer, x + i, y, width, fg);
    }
}
