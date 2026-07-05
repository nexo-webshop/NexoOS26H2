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

use graphics::primitives::Color;
use graphics::framebuffer::draw_pixel;

use input::keyboard::KeyboardState;
use input::mouse::MouseState;
use input::events::InputEvent;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init();

    let _ = core::boot_core();
    let _ = system::init_system();

    let kernel_entry = load_kernel().ok()?;
    let memory_map = get_memory_map().ok()?;

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
