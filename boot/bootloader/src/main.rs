#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;
mod cpu;
mod elf;

use uefi::prelude::*;
use uefi::println;

use bootinfo::BootInfo;
use kernel::load_kernel;
use cpu::CpuState;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00005");
    println!("Stage: ELF loader skeleton");

    let kernel_entry = match load_kernel() {
        Some(addr) => addr,
        None => {
            println!("Kernel load failed (ELF stage)");
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

    let cpu_state = CpuState::new(kernel_entry);

    println!("ELF parsed (stub)");
    println!("Kernel entry: {:#x}", kernel_entry);
    println!("CPU state ready");
    println!("BootInfo constructed");

    println!("READY FOR PHYSICAL MEMORY MAPPING (next version)");

    Status::SUCCESS
}
