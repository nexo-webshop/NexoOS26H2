#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;
mod cpu;
mod elf;
mod memory;
mod framebuffer; // 🆕 NEW

use uefi::prelude::*;
use uefi::println;

use bootinfo::BootInfo;
use kernel::load_kernel;
use memory::get_memory_map;
use framebuffer::get_framebuffer; // 🆕 NEW

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00008");
    println!("Stage: framebuffer initialization");

    let kernel_entry = match load_kernel() {
        Some(addr) => addr,
        None => return Status::LOAD_ERROR,
    };

    let memory_map = match get_memory_map() {
        Ok(map) => map,
        Err(_) => return Status::LOAD_ERROR,
    };

    let mut usable_memory = 0;
    let mut reserved_memory = 0;
    let mut entries = 0;

    for _region in memory_map.regions {
        entries += 1;
        // (logica blijft conceptueel uit vorige versie)
    }

    let fb = get_framebuffer(); // 🆕 NEW

    let bootinfo = BootInfo {
        memory_map_ptr: memory_map.regions.as_ptr() as usize,
        memory_map_entries: entries,
        usable_memory,
        reserved_memory,

        framebuffer_addr: fb.addr,
        framebuffer_width: fb.width,
        framebuffer_height: fb.height,
        framebuffer_pitch: fb.pitch,

        kernel_entry,
    };

    println!("Framebuffer initialized");
    println!("Resolution: {}x{}", fb.width, fb.height);
    println!("Kernel ready at: {:#x}", kernel_entry);

    println!("READY FOR FIRST GRAPHICS OUTPUT (next version)");

    Status::SUCCESS
}
