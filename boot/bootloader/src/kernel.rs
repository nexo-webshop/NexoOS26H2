#![allow(dead_code)]

use crate::elf::{parse_elf, ElfBinary};

pub fn load_kernel() -> Option<usize> {
    // FIX: Zorg voor een minimale mock ELF-header (64 bytes voor x86_64 ELF)
    // om te voorkomen dat parse_elf direct crasht/None teruggeeft.
    let mut fake_kernel_image: [u8; 64] = [0; 64];
    
    // ELF Magic Bytes: 0x7F 'E' 'L' 'F'
    fake_kernel_image[0] = 0x7F;
    fake_kernel_image[1] = b'E';
    fake_kernel_image[2] = b'L';
    fake_kernel_image[3] = b'F';
    // ELF Class (2 = 64-bit)
    fake_kernel_image[4] = 2; 
    
    // Vul hier eventueel een nep entrypoint in op de offset waar je parse_elf de e_entry verwacht (meestal byte 24-31)
    // Bijvoorbeeld een entry point op 0x100000:
    fake_kernel_image[24..32].copy_from_slice(&0x00100000u64.to_le_bytes());

    let elf: ElfBinary = parse_elf(&fake_kernel_image)?;

    Some(elf.header.entry)
}
