#![no_std]
#![no_main]

mod analysis_hints;
mod bootinfo;
mod kernel;
mod cpu;
mod elf;
mod memory;
mod framebuffer;

mod core;
mod system;
mod graphics;
mod input;

use uefi::prelude::*;

use kernel::load_kernel;
use memory::get_memory_map;
use framebuffer::get_framebuffer;

// Zorg dat deze paden exact overeenkomen met hoe je graphics/input mods zijn opgebouwd
use graphics::Color; 
use graphics::draw_pixel;
use input::{KeyboardState, MouseState, InputEvent};

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    let _ = core::boot_core();
    let _ = system::init_system();

    // FIX: Netjes matchen in plaats van de ? operator te misbruiken op een Option
    let kernel_entry = match load_kernel() {
        Some(entry) => entry,
        None => return Status::ABORTED,
    };

    let memory_map = match get_memory_map() {
        Ok(map) => map,
        Err(status) => return status,
    };

    let fb = get_framebuffer();

    let mut keyboard = KeyboardState::new();
    let mut mouse = MouseState::new();

    let events = [
        InputEvent::KeyDown(65),
        InputEvent::MouseMove { x: 120, y: 80 },
        InputEvent::MouseClick { button: 1 },
    ];

    for event in events {
        match event {
            InputEvent::KeyDown(k) => keyboard.handle_key(k),
            InputEvent::KeyUp(_) => {},
            InputEvent::MouseMove { x, y } => mouse.move_to(x, y),
            InputEvent::MouseClick { .. } => mouse.click(),
        }
    }

    let white = Color { r: 255, g: 255, b: 255 };

    // minimal visual output
    for x in 0..200 {
        draw_pixel(fb.addr, x, 120, fb.width, white);
    }

    Status::SUCCESS
}
