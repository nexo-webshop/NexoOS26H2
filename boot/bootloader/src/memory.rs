#![allow(dead_code)]

use uefi::table::boot::{MemoryDescriptor, MemoryType};

pub struct MemoryMap {
    pub descriptors: &'static [MemoryDescriptor],
}

pub fn get_memory_map() -> MemoryMap {
    // v0.2026.00006: stub for now, real UEFI integration comes next step

    MemoryMap {
        descriptors: &[],
    }
}
