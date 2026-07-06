#![allow(dead_code)]

use uefi::boot;
use uefi::table::boot::{MemoryDescriptor, MemoryType};

pub struct MemoryRegion {
    pub ty: MemoryType,
    pub physical_start: u64,
    pub pages: u64,
}

// FIX: Koppel de lifetime aan de onderliggende memory map entries
pub struct MemoryMap<'a> {
    pub regions: &'a [MemoryDescriptor],
}

// FIX: Gebruik boot::memory_map_allocated of accepteer een lifetime.
// Let op: boot::memory_map() alloceert intern uit de UEFI pool als de "alloc" feature aan staat.
pub fn get_memory_map() -> Result<MemoryMap<'static>, uefi::Status> {
    // In uefi 0.35 met "alloc" feature geeft memory_map() een MemoryMapOwned terug,
    // die een eigenaar is van de allocatie. Om de entries static te maken,
    // moeten we de deallocatie voorkomen (leken) of de struct zelf returnen.
    
    let map = boot::memory_map()?;
    
    // We "leken" de MemoryMapOwned container zodat de UEFI pool allocatie 
    // niet gedropt wordt, waardoor de innerlijke slice veilig 'static wordt.
    let leaked_map = core::mem::ManuallyDrop::new(map);
    
    Ok(MemoryMap {
        regions: unsafe { core::mem::transmute(leaked_map.entries()) },
    })
}
