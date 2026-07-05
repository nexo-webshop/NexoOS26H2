#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;
mod cpu;
mod elf;
mod memory;

use uefi::prelude::*;
use uefi::println;

use bootinfo::BootInfo;
use kernel::load_kernel;
use memory::get_memory_map;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00007");
    println!("Stage: REAL memory map acquisition");

    let kernel_entry = match load_kernel() {
        Some(addr) => addr,
        None => {
            println!("Kernel load failed");
            return Status::LOAD_ERROR;
        }
    };

    // 🧠 REAL MEMORY MAP
    let memory_map = match get_memory_map() {
        Ok(map) => map,
        Err(_) => {
            println!("Failed to get memory map");
            return Status::LOAD_ERROR;
        }
    };

    let mut usable_memory = 0;
    let mut reserved_memory = 0;
    let mut entries = 0;

    for region in memory_map.regions {
        entries += 1;

        match region.ty {
            uefi::table::boot::MemoryType::CONVENTIONAL => {
                usable_memory += (region.page_count() * 4096) as usize;
            }
            _ => {
                reserved_memory += (region.page_count() * 4096) as usize;
            }
        }
    }

    let bootinfo = BootInfo {
        memory_map_addr: memory_map.regions.as_ptr() as usize,
        memory_map_entries: entries,
        usable_memory,
        reserved_memory,
        framebuffer_addr: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
        kernel_entry,
    };

    println!("Kernel entry: {:#x}", kernel_entry);
    println!("Memory regions: {}", entries);
    println!("Usable memory: {} bytes", usable_memory);
    println!("Reserved memory: {} bytes", reserved_memory);

    println!("BootInfo fully populated");

    Status::SUCCESS
}
