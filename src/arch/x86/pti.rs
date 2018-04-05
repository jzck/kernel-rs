use core::ptr;

use super::paging::ActivePageTable;
// use super::paging::entry::EntryFlags;

#[thread_local]
pub static mut PTI_CPU_STACK: [u8; 256] = [0; 256];

#[thread_local]
pub static mut PTI_CONTEXT_STACK: usize = 0;

#[inline(always)]
unsafe fn switch_stack(old: usize, new: usize) {
    let old_esp: usize;
    asm!("" : "={esp}"(old_esp) : : : "intel", "volatile");

    let offset_esp = old - old_esp;

    let new_esp = new - offset_esp;

    ptr::copy_nonoverlapping(
        old_esp as *const u8,
        new_esp as *mut u8,
        offset_esp
    );

    asm!("" : : "{esp}"(new_esp) : : "intel", "volatile");
}

#[inline(always)]
pub unsafe fn map() {
    // {
    //     let mut active_table = unsafe { ActivePageTable::new() };
    //
    //     // Map kernel heap
    //     let address = active_table.p4()[::KERNEL_HEAP_PML4].address();
    //     let frame = Frame::containing_address(address);
    //     let mut flags = active_table.p4()[::KERNEL_HEAP_PML4].flags();
    //     flags.remove(EntryFlags::PRESENT);
    //     active_table.p4_mut()[::KERNEL_HEAP_PML4].set(frame, flags);
    //
    //     // Reload page tables
    //     active_table.flush_all();
    // }

    // Switch to per-context stack
    switch_stack(PTI_CPU_STACK.as_ptr() as usize + PTI_CPU_STACK.len(), PTI_CONTEXT_STACK);
}

#[inline(always)]
pub unsafe fn unmap() {
    // Switch to per-CPU stack
    switch_stack(PTI_CONTEXT_STACK, PTI_CPU_STACK.as_ptr() as usize + PTI_CPU_STACK.len());

    // {
    //     let mut active_table = unsafe { ActivePageTable::new() };
    //
    //     // Unmap kernel heap
    //     let address = active_table.p4()[::KERNEL_HEAP_PML4].address();
    //     let frame = Frame::containing_address(address);
    //     let mut flags = active_table.p4()[::KERNEL_HEAP_PML4].flags();
    //     flags.insert(EntryFlags::PRESENT);
    //     active_table.p4_mut()[::KERNEL_HEAP_PML4].set(frame, flags);
    //
    //     // Reload page tables
    //     active_table.flush_all();
    // }
}
