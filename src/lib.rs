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
pub use arch::x86::consts::*;

// use core::mem;
// use x86::structures::idt::*;
/// kernel entry point. arch module is responsible for
/// calling this once the core has loaded
pub fn kmain() -> ! {
    // heap avalaible for tracking freed frames
    memory::init_noncore();

    // vga is *not* cpu specific
    vga::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    // unsafe {
    //     *(0xdead as *mut u32) = 42;
    // };

    // x86::instructions::interrupts::int3();

    // println!("flags: {:?}", x86::registers::flags::flags());
    // flush!();

    let sp = (::USER_STACK_OFFSET + ::USER_STACK_SIZE).as_u32();
    // let sp: u32;
    // unsafe {
    //     asm!("mov %ebp, $0" : "=r" (sp));
    // }
    let ip = self::init as *const () as u32;

    unsafe {
        arch::x86::usermode(ip, sp, 0);
    }
    unreachable!()
    // loop {}
}

pub fn init() {
    println!("inside init function!!!!!");
    flush!();
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

#[global_allocator]
static HEAP_ALLOCATOR: allocator::Allocator = allocator::Allocator;
