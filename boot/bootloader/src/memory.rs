#![allow(dead_code)]

use uefi::boot;
use uefi::table::boot::{MemoryDescriptor, MemoryType};

pub struct MemoryRegion {
    pub ty: MemoryType,
    pub physical_start: u64,
    pub pages: u64,
}

pub struct MemoryMap {
    pub regions: &'static [MemoryDescriptor],
}

pub fn get_memory_map() -> Result<MemoryMap, uefi::Status> {
    let map = boot::memory_map();

    match map {
        Ok(map) => {
            Ok(MemoryMap {
                regions: map.entries(),
            })
        }
        Err(e) => Err(e),
    }
}
