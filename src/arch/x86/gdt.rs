use x86::structures::gdt;
use x86::structures::tss;
use x86::structures::gdt::SegmentSelector;
use x86::instructions::segmentation::*;
use x86::instructions::tables::load_tss;
use x86::PrivilegeLevel::{Ring0, Ring3};

pub static mut GDT: gdt::Gdt = gdt::Gdt::new();
pub static mut TSS: tss::TaskStateSegment = tss::TaskStateSegment::new();
pub static mut TASK_TSS: tss::TaskStateSegment = tss::TaskStateSegment::new();

pub static GDT_KERNEL_CODE: SegmentSelector = SegmentSelector::new(1, Ring0);
pub static GDT_KERNEL_DATA: SegmentSelector = SegmentSelector::new(2, Ring0);
pub static GDT_USER_CODE: SegmentSelector = SegmentSelector::new(3, Ring3);
pub static GDT_USER_DATA: SegmentSelector = SegmentSelector::new(4, Ring3);
pub static GDT_TSS: SegmentSelector = SegmentSelector::new(5, Ring3);
pub static GDT_TASK_TSS: SegmentSelector = SegmentSelector::new(7, Ring3);

pub unsafe fn init() {
    // the following *order* is important
    let kcode_selector = GDT.add_entry(gdt::Descriptor::kernel_code_segment());
    let kdata_selector = GDT.add_entry(gdt::Descriptor::kernel_data_segment());
    let ucode_selector = GDT.add_entry(gdt::Descriptor::user_code_segment());
    let udata_selector = GDT.add_entry(gdt::Descriptor::user_data_segment());

    //I read that the tss should be twice as long
    //fuck knows why...
    TSS.ss0 = GDT_KERNEL_CODE.0;
    asm!("mov %esp, $0" : "=r" (TSS.esp0));
    let tss_selector = GDT.add_entry(gdt::Descriptor::tss_segment(&TSS));
    GDT.add_entry(gdt::Descriptor(0));

    TASK_TSS.eip = self::test_task as *const () as u32;
    let task_tss_selector = GDT.add_entry(gdt::Descriptor::tss_segment(&TASK_TSS));
    GDT.add_entry(gdt::Descriptor(0));

    assert_eq!(kcode_selector, GDT_KERNEL_CODE);
    assert_eq!(kdata_selector, GDT_KERNEL_DATA);
    assert_eq!(ucode_selector, GDT_USER_CODE);
    assert_eq!(udata_selector, GDT_USER_DATA);
    assert_eq!(tss_selector, GDT_TSS);
    assert_eq!(task_tss_selector, GDT_TASK_TSS);

    // use x86::structures::gdt::Descriptor;
    // println!(
    //     "tr({:#x}):\n {:#?}",
    //     tss_selector.0,
    //     gdt::Descriptor(GDT.table[tss_selector.index() as usize])
    // );
    // flush!();

    GDT.load();
    set_cs(kcode_selector);
    load_ds(kdata_selector);
    load_es(kdata_selector);
    load_ss(kdata_selector);
    load_tss(tss_selector);
    // unreachable!();
}

pub fn test_task() {
    println!("inside test task omg we did it !!!");
    flush!();
}
