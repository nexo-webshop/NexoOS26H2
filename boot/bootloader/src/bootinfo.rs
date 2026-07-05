#![allow(dead_code)]

pub struct BootInfo {
    pub memory_map_ptr: usize,
    pub memory_map_entries: usize,

    pub usable_memory: usize,
    pub reserved_memory: usize,

    // 🖥️ NEW: framebuffer support
    pub framebuffer_addr: usize,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_pitch: u32,

    pub kernel_entry: usize,
}
