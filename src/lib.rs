//! project hosted on [github](https://github.com/jzck/kernel)

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
extern crate multiboot2;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate alloc;
extern crate x86;

/// 80x25 screen and simplistic terminal driver
#[macro_use] pub mod vga;
/// PS/2 detection and processing
pub mod keyboard;
/// simplisitc kernel commands
pub mod console;
/// rust wrappers around cpu I/O instructions.
pub mod cpuio;
/// ACPI self-content module
pub mod acpi;
/// physical frame allocator + paging module + heap allocator
pub mod memory;
/// x86 interruptions
pub mod interrupts;

fn init_kernel(multiboot_info_addr: usize) -> Result <(), &'static str> {

    let boot_info = unsafe { multiboot2::load(multiboot_info_addr)};

    if let Some(rsdp) = boot_info.rsdp_v2_tag() {
        acpi::load(rsdp)?;
    } else if let Some(rsdp) = boot_info.rsdp_tag() {
        acpi::load(rsdp)?;
    } else {
        acpi::init()?;
    }

    enable_paging();
    enable_write_protect_bit();

    memory::init(&boot_info);
    interrupts::init();
    vga::init();

    Ok(())
}

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    if let Err(msg) = init_kernel(multiboot_info_addr) {
        println!("Kernel initialization has failed: {}", msg);
        cpuio::halt();
    }

    x86::instructions::interrupts::int3();

    loop { keyboard::kbd_callback(); }
}

#[lang = "eh_personality"] #[no_mangle]
pub extern fn eh_personality() {

}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32)
-> ! {
    println!("PANIC: {}", fmt);
    println!("FILE: {}", file);
    println!("LINE: {}", line);
    flush!();
    loop {}
}

use memory::BumpAllocator;

pub const HEAP_START: usize = (1 << 22); //first entry of p2
pub const HEAP_SIZE: usize = 100 * 1024; //100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START,
                                                          HEAP_START + HEAP_SIZE);
