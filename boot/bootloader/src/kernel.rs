#![allow(dead_code)]

pub fn load_kernel() -> Option<usize> {
    // v0.2026.00003: alleen simulatie
    let fake_kernel_entry: usize = 0x100000;
    Some(fake_kernel_entry)
}
