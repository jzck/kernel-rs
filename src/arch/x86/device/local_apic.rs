use ::arch::x86::paging::ActivePageTable;

pub static mut LOCAL_APIC: LocalApic = LocalApic {
    address: 0,
    x2: false
};

pub unsafe fn init(active_table: &mut ActivePageTable) {
    LOCAL_APIC.init(active_table);
}

pub struct LocalApic {
    pub address: usize,
    pub x2: bool
}

impl LocalApic {
    unsafe fn init(&mut self, active_table: &mut ActivePageTable) {
        // let efer = Efer::read();
        // println!("efer = {:?}", efer);
        // flush!();
    }
}
