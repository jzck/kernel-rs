use super::*;

pub fn flush(addr: usize) {
    unsafe { asm!("invlpg ($0)" :: "r"(addr) : "memory")}
}

pub fn flush_all() {
    let cr3 = cr3();
    unsafe { cr3_write(cr3); }
}
