extern crate x86;

#[macro_use]
pub mod macros;
pub mod paging;
pub mod interrupt;
pub mod device;

use multiboot2;
use acpi;

#[no_mangle]
pub unsafe extern fn x86_rust_start(multiboot_info_addr: usize) {
    // parse multiboot2 info
    let boot_info = multiboot2::load(multiboot_info_addr);

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

    // pic
    self::device::init();

    // set up interrupts
    self::interrupt::init();

    // set up virtual mapping
    let mut active_table = self::paging::init(&boot_info);

    // set up heap
    ::allocator::init(&mut active_table);

    // after core has loaded
    ::memory::init_noncore();


    // primary CPU entry point
    ::kmain();
}
