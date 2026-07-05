#![allow(dead_code)]

pub struct BootInfo {
    pub memory_map_addr: usize,
    pub memory_map_size: usize,
    pub framebuffer_addr: usize,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub kernel_entry: usize,
}
