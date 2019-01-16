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
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
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

#[macro_use]
pub mod vga;
pub mod keyboard;
pub mod console;
pub mod acpi;
pub mod allocator;
pub mod memory;
pub mod arch;
pub use arch::x86::consts::*;
pub mod scheduling;
pub mod time;
pub mod pci;

/// kernel entry point. arch module is responsible for
/// calling this once the core has loaded
pub fn kmain() -> ! {
    // memory init after heap is available
    memory::init_noncore();

    // unsafe VGA
    unsafe { console::CONSOLE.init(); }

    pci::lspci();
    // scheduler WIP
    // scheduling::schedule();
    unreachable!();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    flush!();
    loop {}
}

#[global_allocator]
// pub static ALLOCATOR: slab_allocator::LockedHeap = allocator::ALLOCATOR;
pub static ALLOCATOR: slab_allocator::LockedHeap = slab_allocator::LockedHeap::empty();
