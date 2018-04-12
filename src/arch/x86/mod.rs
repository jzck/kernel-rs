extern crate x86;

#[macro_use]
pub mod paging;
pub mod interrupt;
pub mod device;
pub mod pti;

pub mod gdt;
pub mod idt;

use multiboot2;
use acpi;

#[no_mangle]
pub unsafe extern "C" fn x86_rust_start(multiboot_info_addr: usize) {
    // parse multiboot2 info
    let boot_info = multiboot2::load(multiboot_info_addr);

    // println!("{:?}", boot_info);
    // flush!();
    // asm!("hlt");

    // ACPI must be intialized BEFORE paging is active
    if let Some(rsdp) = boot_info.rsdp_v2_tag() {
        acpi::load(rsdp).expect("ACPI failed");
    } else if let Some(rsdp) = boot_info.rsdp_tag() {
        acpi::load(rsdp).expect("ACPI failed");
    } else {
        acpi::init().expect("ACPI failed");
    }

    // set up physical allocator
    ::memory::init(&boot_info);

    // set up virtual addressing (paging)
    let mut active_table = paging::init(&boot_info);

    // load idt (exceptions + irqs)
    idt::init();

    // fill and load gdt
    gdt::init();

    // set up heap
    ::allocator::init(&mut active_table);

    // set up pic & apic
    device::init(&mut active_table);

    // primary CPU entry point
    ::kmain();
}
