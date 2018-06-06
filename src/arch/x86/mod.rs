extern crate x86;

#[macro_use]
pub mod paging;
pub mod interrupt;
pub mod devices;
pub mod consts;

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

    // set up user stack
    // use x86::structures::paging::*;
    // for page in ::USER_STACK_RANGE {
    //     active_table.map(page, PageTableFlags::WRITABLE);
    // }

    // set up pic, pit
    devices::init();

    // primary CPU entry point
    ::kmain();
}

pub unsafe fn switch(ip: u32) -> ! {
    asm!("push $0; ret" :: "r"(ip) :: "volatile", "intel");
    unreachable!();
}

pub unsafe fn usermode(ip: u32, sp: u32) -> ! {
    // use x86::structures::gdt::{Descriptor, SegmentSelector};
    // use x86::instructions::segmentation::*;
    // use x86::PrivilegeLevel::{Ring0, Ring3};

    x86::instructions::interrupts::disable();

    // println!("sp: {:#x}", sp);
    // println!("ip: {:#x}", ip);

    // load_ds(udata_selector);
    // asm!("mov $0, %ds" :: "r"(gdt::GDT_USER_DATA << 3 | 3));

    // loop {}

    // load_es(udata_selector);
    // load_fs(udata_selector);
    // load_gs(udata_selector);

    use x86::registers::flags;
    flags::set_flags(flags::Flags::NT);

    // asm!("mov %esp, $0" : "=r" (sp));

    println!("{:#x}", gdt::GDT_KERNEL_DATA.0);
    println!("{:#x}", sp);
    println!("{:#x}", 1 << 9);
    println!("{:#x}", gdt::GDT_KERNEL_CODE.0);
    println!("{:#x}", ip);
    flush!();

    asm!("
         push $0; \
         push $1; \
         push $2; \
         push $3; \
         push $4"
         : //no output
         : "r"(gdt::GDT_USER_DATA.0),
         "r"(sp),
         "r"(1 << 9) // interrupt enable flag
         "r"(gdt::GDT_USER_CODE.0),
         "r"(ip)
         : //no clobbers
         : "intel", "volatile"
         );

    // loop {}

    asm!("iret");

    unreachable!();
}
