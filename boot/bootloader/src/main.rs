#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;

#[entry]
fn efi_main() -> Status {
    Status::SUCCESS
}
