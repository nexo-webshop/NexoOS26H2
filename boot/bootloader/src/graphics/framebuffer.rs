#![allow(dead_code)]

use crate::graphics::primitives::Color;

pub fn draw_pixel(
    framebuffer: usize,
    x: u32,
    y: u32,
    width: u32,
    color: Color,
) {
    let offset = ((y * width + x) * 4) as usize;
    let ptr = (framebuffer + offset) as *mut u8;

    unsafe {
        *ptr.add(0) = color.b;
        *ptr.add(1) = color.g;
        *ptr.add(2) = color.r;
        *ptr.add(3) = 0;
    }
}
