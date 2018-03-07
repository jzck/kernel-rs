//! x86 (32 bit) only

pub fn cr3() -> usize {
    let ret: usize;
    unsafe { asm!("mov %cr3, $0" : "=r" (ret)) };
    ret
}

pub unsafe fn cr3_write(val: usize) {
    asm!("mov $0, %cr3" :: "r" (val) : "memory");
}
