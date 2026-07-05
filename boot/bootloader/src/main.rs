#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;

use uefi::prelude::*;
use uefi::println;

use bootinfo::BootInfo;
use kernel::load_kernel;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00003");
    println!("Stage: kernel preparation");

    let kernel_entry = match load_kernel() {
        Some(addr) => addr,
        None => {
            println!("Kernel load failed");
            return Status::LOAD_ERROR;
        }
    };

    let bootinfo = BootInfo {
        memory_map_addr: 0,
        memory_map_size: 0,
        framebuffer_addr: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
        kernel_entry,
    };

    println!("Kernel prepared at: {:#x}", kernel_entry);
    println!("BootInfo created");

    Status::SUCCESS
}
