//! project hosted on [github](https://github.com/jzck/kernel)

// nightly stuff we need
#![no_std]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(asm)]
#![feature(thread_local)]
// home made heap
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
// x86 specific
#![feature(abi_x86_interrupt)]

extern crate alloc;
#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
extern crate raw_cpuid;
extern crate rlibc;
extern crate slab_allocator;
extern crate spin;

// used by arch/x86, need conditional compilation here
extern crate x86;

/// 80x25 terminal driver
#[macro_use]
pub mod vga;
/// PS/2 detection and processing
pub mod keyboard;
/// simplisitc kernel commands
pub mod console;
/// ACPI self contained module
pub mod acpi;
/// Heap allocators
pub mod allocator;
/// Memory management
pub mod memory;
/// arch specific entry points
pub mod arch;
pub use arch::x86::consts::*;
/// concurrency management
pub mod scheduling;
/// uptime counting
pub mod time;

/// kernel entry point. arch module is responsible for
/// calling this once the core has loaded
pub fn kmain() -> ! {
    // memory init after heap is available
    memory::init_noncore();

    // vga is *not* cpu specific I think
    vga::init();

    scheduling::schedule();
    unreachable!();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("PANIC: {}", fmt);
    println!("FILE: {}", file);
    println!("LINE: {}", line);
    flush!();
    loop {}
}

#[global_allocator]
static HEAP_ALLOCATOR: allocator::Allocator = allocator::Allocator;
