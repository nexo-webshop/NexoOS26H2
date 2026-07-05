#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;
mod cpu;
mod elf;
mod memory;
mod framebuffer;
mod graphics; // 🆕 NEW

use uefi::prelude::*;
use uefi::println;

use bootinfo::BootInfo;
use kernel::load_kernel;
use memory::get_memory_map;
use framebuffer::get_framebuffer;
use graphics::{draw_pixel, Color}; // 🆕 NEW

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00009");
    println!("Stage: first pixel rendering");

    let kernel_entry = load_kernel().ok_or(Status::LOAD_ERROR)?;

    let memory_map = get_memory_map().map_err(|_| Status::LOAD_ERROR)?;

    let mut entries = 0;
    for _ in memory_map.regions {
        entries += 1;
    }

    let fb = get_framebuffer();

    let bootinfo = BootInfo {
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

    println!("Drawing test pattern...");

    // 🎨 FIRST PIXELS EVER
    let color = Color { r: 0, g: 255, b: 0 };

    for x in 0..200 {
        draw_pixel(
            fb.addr,
            x,
            100,
            fb.width,
            color,
        );
    }

    println!("Pixel line drawn");
    println!("Framebuffer write successful");

    Status::SUCCESS
}
