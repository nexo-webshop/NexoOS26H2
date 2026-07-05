#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;
mod cpu;
mod elf;
mod memory;
mod framebuffer;
mod graphics;
mod font;       // 🆕
mod ui;         // 🆕
mod progress;   // 🆕

use uefi::prelude::*;

use bootinfo::BootInfo;
use kernel::load_kernel;
use memory::get_memory_map;
use framebuffer::get_framebuffer;
use graphics::Color;
use ui::draw_text;
use progress::draw_progress_bar;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    let kernel_entry = load_kernel().ok()?;
    let memory_map = get_memory_map().ok()?;

    let mut entries = 0;
    for _ in memory_map.regions {
        entries += 1;
    }

    let fb = get_framebuffer();

    let _bootinfo = BootInfo {
        memory_map_ptr: memory_map.regions.as_ptr() as usize,
        memory_map_entries: entries,
        usable_memory: 0,
        reserved_memory: 0,
        framebuffer_addr: fb.addr,
        framebuffer_width: fb.width,
        framebuffer_height: fb.height,
        framebuffer_pitch: fb.pitch,
        kernel_entry,
    };

    let white = Color { r: 255, g: 255, b: 255 };

    // 🧠 BOOT UI START
    draw_text(
        fb.addr,
        20,
        20,
        fb.width,
        "NexoOS Bootloader v0.2026.00010",
        white,
    );

    draw_text(
        fb.addr,
        20,
        40,
        fb.width,
        "Initializing system...",
        white,
    );

    // 📊 progress bar simulation
    for i in 0..100 {
        draw_progress_bar(fb.addr, 20, 70, fb.width, i, 100);
    }

    Status::SUCCESS
}
