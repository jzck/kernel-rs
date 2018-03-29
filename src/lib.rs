//! project hosted on [github](https://github.com/jzck/kernel)
//! exclusively x86
//!
#![no_std]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]

#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]

#![feature(abi_x86_interrupt)]

extern crate rlibc;
#[macro_use] extern crate alloc;
#[macro_use] extern crate lazy_static;
extern crate spin;
extern crate multiboot2;
extern crate slab_allocator;

// used by arch/x86, need conditional compilation here
extern crate x86;

/// 80x25 terminal driver
#[macro_use] pub mod vga;
/// PS/2 detection and processing
pub mod keyboard;
/// simplisitc kernel commands
pub mod console;
/// rust wrappers around cpu I/O instructions.
pub mod cpuio;
pub mod io;
/// ACPI self contained module
pub mod acpi;
/// Heap allocators
pub mod allocator;
/// Memory management
pub mod memory;
/// arch specific entry points
pub mod arch;

/// kernel entry point. arch module is responsible for calling this
pub fn kmain() -> ! {

    // vga is specific to chipset not cpu
    vga::init();

    // x86::instructions::interrupts::disable();
    // x86::instructions::interrupts::int3();
    // x86::instructions::interrupts::enable();

    // fn stack_overflow() { stack_overflow(); }
    // stack_overflow();

    // unsafe {
    //     *(0xdead as *mut u32) = 42;
    // };

    loop {}
}

#[lang = "eh_personality"] #[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32)
-> ! {
    println!("PANIC: {}", fmt);
    println!("FILE: {}", file);
    println!("LINE: {}", line);
    flush!();
    loop {}
}

pub const HEAP_START: usize = (1 << 22); //first entry of p2
pub const HEAP_SIZE: usize = 10 * 4096 * 8; //~ 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: allocator::Allocator = allocator::Allocator;
