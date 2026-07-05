#![allow(dead_code)]

// Minimal ELF skeleton (geen echte parsing nog)
pub struct ElfHeader {
    pub entry: usize,
}

pub struct ElfBinary {
    pub header: ElfHeader,
}

pub fn parse_elf(_data: &[u8]) -> Option<ElfBinary> {
    // v0.2026.00005: stub parser
    // later: echte ELF64 parsing

    Some(ElfBinary {
        header: ElfHeader {
            entry: 0x100000, // fake kernel entry
        },
    })
}
