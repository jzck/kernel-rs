//! project hosted on [github](https://github.com/jzck/kernel)

// nightly stuff we need
#![no_std]
#![feature(lang_items)]
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
/// rust wrappers around cpu I/O instructions.
pub mod io;
/// ACPI self contained module
pub mod acpi;
/// Heap allocators
pub mod allocator;
/// Memory management
pub mod memory;
/// arch specific entry points
pub mod arch;

// use core::mem;
// use x86::structures::idt::*;
/// kernel entry point. arch module is responsible for
/// calling this once the core has loaded
pub fn kmain() -> ! {
    // heap avalaible for tracking freed frames
    memory::init_noncore();

    // x86::instructions::interrupts::int3();
        // println!("size of idt entry: {}", mem::size_of::<IdtEntry<HandlerFunc>>());
        // println!("size of i32 {}", mem::size_of::<i32>());
        // flush!();
        // unsafe {asm!("hlt");}
    // let x = 0;
    // let y = 5 /x;
    // println!("x {} y {}", x, y);

    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    unsafe {
        *(0xdead as *mut u32) = 42;
    };

    // vga is *not* cpu specific
    vga::init();

    loop {}
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

pub const HEAP_START: usize = (1 << 22 + 2); //third entry of p2
pub const HEAP_SIZE: usize = 10 * 4096 * 8; //~ 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: allocator::Allocator = allocator::Allocator;
