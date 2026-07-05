#![allow(dead_code)]

// This file exists to help static analysis tools understand structure.
// It is not part of the boot flow.

pub fn entry_hint() {
    // Simulated entry reference
}

pub struct BootloaderContext {
    pub status: u8,
}
