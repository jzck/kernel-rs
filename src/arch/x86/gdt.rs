use x86::structures::gdt;
use x86::structures::tss;
use x86::instructions::segmentation::set_cs;
use x86::instructions::tables::load_tss;
use x86::registers::control;
use arch::x86::paging::ActivePageTable;
use spin::Once;
// use io;

static GDT: Once<gdt::Gdt> = Once::new();
static TSS_MAIN: Once<tss::TaskStateSegment> = Once::new();
static TSS_INT: Once<tss::TaskStateSegment> = Once::new();

pub fn init(mut active_table: &mut ActivePageTable) {
    let tss_main = TSS_MAIN.call_once(|| {
        let mut tss = tss::TaskStateSegment::new();
        // tss.esp0 = stack.top;
        tss.ss0 = 0x8;
        tss.cr3 = control::Cr3::read_u32();
        tss.reserved_iopb = 1; //T debug bit
        tss
    });

    let tss_int = TSS_INT.call_once(|| {
        let mut tss = tss::TaskStateSegment::new();
        match ::memory::allocate_stack(&mut active_table) {
            Some(stack) => {
                println!("int tss stack at {:#x}", stack.top);
                tss.esp0 = stack.top;
                tss.ss0 = 0x8;
                tss.cr3 = control::Cr3::read_u32();
                // tss.esp = stack.top;
                // tss.es = 0x8;
                // tss.cs = 0x8;
                // tss.ss = 0x8;
                // tss.ds = 0x8;
                // tss.fs = 0x8;
                // tss.gs = 0x8;
                // tss.link = 0x10; //main tss
                tss.reserved_iopb = 1; //T debug bit
            }
            _ => panic!("There is no stack available for tss"),
        };
        tss
    });

    let mut code_selector = gdt::SegmentSelector(0);
    let mut tss_main_selector = gdt::SegmentSelector(0);
    let mut tss_int_selector = gdt::SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_main_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss_main));
        tss_int_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss_int));
        gdt
    });

    // println!("gdt 0 upper: {:#x}", gdt.table[0] as u32);
    // println!("gdt 0 lower: {:#x}", gdt.table[0] >> 32 as u32);
    // println!("gdt 1 upper: {:#x}", gdt.table[1] as u32);
    // println!("gdt 1 lower: {:#x}", gdt.table[1] >> 32 as u32);
    // println!("gdt 2 upper: {:#x}", gdt.table[2] as u32);
    // println!("gdt 2 lower: {:#x}", gdt.table[2] >> 32 as u32);
    // println!("gdt 3 upper: {:#x}", gdt.table[3] as u32);
    // println!("gdt 3 lower: {:#x}", gdt.table[3] >> 32 as u32);
    // println!("gdt 4 upper: {:#x}", gdt.table[4] as u32);
    // println!("gdt 4 lower: {:#x}", gdt.table[4] >> 32 as u32);
    flush!();

    gdt.load();
    unsafe {
        // reload code segment register
        println!("set_cs({:#x})", code_selector.0);
        set_cs(code_selector);
        // load TSS
        println!("loading tss {:?}", tss_main_selector);
        load_tss(tss_main_selector);
    }
}
