extern crate x86;

#[macro_use]
pub mod paging;
pub mod interrupt;
pub mod device;
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

    // set up pic & apic
    device::init(&mut active_table);

    // primary CPU entry point
    ::kmain();
}

pub unsafe fn usermode(ip: u32, sp: u32, arg: u32) -> ! {
    asm!("push r10
         push r11
         push r12
         push r13
         push r14
         push r15
         "
         : //no output
         : "{r10}"(gdt::GDT_USER_DATA << 3 | 3),
         "{r11}"(sp),
         "{r12}"(1 << 9) // interrupt enable flag
         "{r13}"(gdt::GDT_USER_CODE << 3 | 3),
         "{r14}"(ip),
         "{r15}"(arg)
         : //no clobbers
         : "intel", "volatile"
         );

    asm!("mov ds, r14d
         mov es, r14d
         mov fs, r15d
         mov gs, r14d
         fninit
         iret"
         : //no output (never returns)
         : "{r14}"(gdt::GDT_USER_DATA << 3 | 3),
         "{r15}"(gdt::GDT_USER_CODE << 3 | 3)
         : //no clobbers (never returns)
         : "intel", "volatile"
         );
    unreachable!();
}
