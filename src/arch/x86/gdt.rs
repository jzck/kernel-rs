use x86::structures::gdt;
use x86::structures::tss;
use x86::instructions::segmentation::set_cs;
use x86::instructions::tables::load_tss;
use x86::registers::control;
use arch::x86::paging::ActivePageTable;
use spin::Once;

static GDT: Once<gdt::Gdt> = Once::new();
static TSS_MAIN: Once<tss::TaskStateSegment> = Once::new();
static TSS_INT: Once<tss::TaskStateSegment> = Once::new();

pub fn init(mut active_table: &mut ActivePageTable) {
    let tss_main = TSS_MAIN.call_once(|| {
        let mut tss = tss::TaskStateSegment::new();
        // tss.esp0 = stack.top;
        // tss.ss = 0x8;
        tss.cr3 = control::Cr3::read_u32();
        tss.reserved_iopb = 1; //T debug bit
        tss
    });

    let mut code_selector = gdt::SegmentSelector(0);
    let mut tss_main_selector = gdt::SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_main_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss_main));
        gdt
    });

    println!("gdt[0]={:#?}", gdt::Descriptor(gdt.table[0]));
    println!("gdt[1]={:#?}", gdt::Descriptor(gdt.table[1]));
    println!("gdt[2]={:#?}", gdt::Descriptor(gdt.table[2]));
    println!("gdt[3]={:#?}", gdt::Descriptor(gdt.table[3]));
    println!("gdt[4]={:#?}", gdt::Descriptor(gdt.table[4]));
    flush!();

    gdt.load();
    unsafe {
        // reload code segment register
        // println!("set_cs({:#x})", code_selector.0);
        set_cs(code_selector);
        // load TSS
        // println!("loading tss {:?}", tss_main_selector);
        load_tss(tss_main_selector);
    }
}
