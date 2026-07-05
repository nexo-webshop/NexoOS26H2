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

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00006");
    println!("Stage: memory map acquisition");

    let kernel_entry = match load_kernel() {
        Some(addr) => addr,
        None => {
            println!("Kernel load failed");
            return Status::LOAD_ERROR;
        }
    };

    // 🔒 MEMORY MAP STAGE (conceptueel in deze versie)
    let memory_map_ptr: usize = 0x0; // placeholder
    let memory_map_entries: usize = 0;

    let bootinfo = BootInfo {
        memory_map_ptr,
        memory_map_entries,
        framebuffer_addr: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
        kernel_entry,
    };

    println!("Kernel entry: {:#x}", kernel_entry);
    println!("Memory map acquired (stub)");
    println!("BootInfo updated");

    println!("READY FOR REAL UEFI MEMORY MAP (next upgrade)");

    Status::SUCCESS
}
