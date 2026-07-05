#![allow(dead_code)]

pub fn load_kernel() -> Option<usize> {
    // v0.2026.00004: nog steeds stub
    // later vervangen door ELF loader

    let fake_kernel_entry: usize = 0x100000;
    Some(fake_kernel_entry)
}
