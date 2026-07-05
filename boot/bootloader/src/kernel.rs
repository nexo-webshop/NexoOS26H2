#![allow(dead_code)]

use crate::elf::{parse_elf, ElfBinary};

pub fn load_kernel() -> Option<usize> {
    let fake_kernel_image: [u8; 1] = [0];

    let elf: ElfBinary = parse_elf(&fake_kernel_image)?;

    Some(elf.header.entry)
}
