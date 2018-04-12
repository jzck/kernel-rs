use x86::structures::gdt;
use x86::structures::tss;
use x86::instructions::segmentation::set_cs;
use x86::instructions::tables::load_tss;
use spin::Once;

static GDT: Once<gdt::Gdt> = Once::new();
static TSS: Once<tss::TaskStateSegment> = Once::new();

pub fn init() {
    let tss = tss::TaskStateSegment::new();
    let tss = TSS.call_once(|| {
        let tss = tss::TaskStateSegment::new();
        tss
    });

    let mut code_selector = gdt::SegmentSelector(0);
    let mut tss_selector = gdt::SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        println!("cs: {:?}", code_selector);
        gdt
    });

    println!("0 upper: {:#x}", gdt.table[0] as u32);
    println!("0 lower: {:#x}", gdt.table[0] >> 32 as u32);
    println!("1 upper: {:#x}", gdt.table[1] as u32);
    println!("1 lower: {:#x}", gdt.table[1] >> 32 as u32);
    println!("2 upper: {:#x}", gdt.table[2] as u32);
    println!("2 lower: {:#x}", gdt.table[2] >> 32 as u32);
    println!("3 upper: {:#x}", gdt.table[3] as u32);
    println!("3 lower: {:#x}", gdt.table[3] >> 32 as u32);
    flush!();

    gdt.load();
    unsafe {
        // reload code segment register
        set_cs(code_selector);
        // load TSS
        load_tss(tss_selector);
    }
}
