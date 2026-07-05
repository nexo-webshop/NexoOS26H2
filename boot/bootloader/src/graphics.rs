#![allow(dead_code)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn draw_pixel(
    framebuffer: usize,
    x: u32,
    y: u32,
    width: u32,
    color: Color,
) {
    // v0.2026.00009: naive 32-bit framebuffer assumption

    let offset = ((y * width + x) * 4) as usize;
    let ptr = (framebuffer + offset) as *mut u8;

    unsafe {
        *ptr.add(0) = color.b;
        *ptr.add(1) = color.g;
        *ptr.add(2) = color.r;
        *ptr.add(3) = 0;
    }
}
