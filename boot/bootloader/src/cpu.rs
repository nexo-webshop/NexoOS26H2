#![allow(dead_code)]

// Simulatie van CPU state voor kernel handover
pub struct CpuState {
    pub instruction_pointer: usize,
    pub stack_pointer: usize,
    pub flags: usize,
}

impl CpuState {
    pub fn new(kernel_entry: usize) -> Self {
        Self {
            instruction_pointer: kernel_entry,
            stack_pointer: 0x800000, // tijdelijke stack
            flags: 0x202,            // default CPU flags
        }
    }
}
