use x86::structures::gdt;
use x86::structures::tss;
use x86::instructions::segmentation::*;
use x86::instructions::tables::load_tss;
use x86::*;
use spin::Once;

pub static mut GDT: gdt::Gdt = gdt::Gdt::new();
pub static mut TSS: tss::TaskStateSegment = tss::TaskStateSegment::new();

pub static GDT_KERNEL_CODE: u16 = 1;
pub static GDT_KERNEL_DATA: u16 = 1;
pub static GDT_USER_CODE: u16 = 2;
pub static GDT_USER_DATA: u16 = 3;

pub unsafe fn init() {
    TSS.ss0 = gdt::SegmentSelector::new(GDT_KERNEL_CODE, PrivilegeLevel::Ring0).0;
    asm!("mov %ebp, $0" : "=r" (TSS.esp0));

    // the following order is important
    let kcode_selector = GDT.add_entry(gdt::Descriptor::kernel_code_segment());
    let kdata_selector = GDT.add_entry(gdt::Descriptor::kernel_data_segment());
    let tss_selector = GDT.add_entry(gdt::Descriptor::tss_segment(&TSS));
    //I read that the tss should be twice as long
    //fuck knows why...
    //also this doesnt work if the tss is after 3rd spot
    //once again: fuck knows why...
    GDT.add_entry(gdt::Descriptor(0));
    let ucode_selector = GDT.add_entry(gdt::Descriptor::user_code_segment());
    let udata_selector = GDT.add_entry(gdt::Descriptor::user_data_segment());

    GDT.load();
    set_cs(kcode_selector);
    load_ds(kdata_selector);
    load_es(kdata_selector);
    load_ss(kdata_selector);
    load_tss(tss_selector);
}
