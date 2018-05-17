use arch::x86::paging::ActivePageTable;
pub mod pic;
pub mod local_apic;
pub mod cpu;

pub unsafe fn init(active_table: &mut ActivePageTable) {
    pic::init();
    local_apic::init(active_table);
}
