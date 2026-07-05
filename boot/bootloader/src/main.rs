#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::println;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    println!("NexoOS Bootloader v0.2026.00002");
    println!("Status: OK");
    println!("UEFI initialized");

    Status::SUCCESS
}
