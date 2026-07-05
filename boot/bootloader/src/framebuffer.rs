#![allow(dead_code)]

pub struct FramebufferInfo {
    pub addr: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

pub fn get_framebuffer() -> FramebufferInfo {
    // v0.2026.00008: stub (UEFI Graphics Output Protocol komt later)

    FramebufferInfo {
        addr: 0x0,
        width: 1024,
        height: 768,
        pitch: 0,
    }
}
