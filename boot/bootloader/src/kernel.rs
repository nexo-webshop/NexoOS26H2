#![allow(dead_code)]

use crate::elf::{parse_elf, ElfBinary};

pub fn load_kernel() -> Option<usize> {
    // v0.2026.00005: introduce ELF pipeline concept

    let fake_kernel_image: [u8; 1] = [0]; // placeholder binary

    let elf: ElfBinary = parse_elf(&fake_kernel_image)?;

    Some(elf.header.entry)
}
