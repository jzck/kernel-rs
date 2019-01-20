// pub use self::ALLOCATOR;

use x86::structures::paging::*;
use arch::x86::paging::*;

fn map_heap(active_table: &mut ActivePageTable) {
    //zone for heap is predefined in `consts.rs`
    for page in ::KERNEL_HEAP_RANGE {
        active_table.map(page, PageTableFlags::WRITABLE);
    }
}

/// should be called only once
pub unsafe fn init(active_table: &mut ActivePageTable) {
    let offset = ::KERNEL_HEAP_OFFSET;
    let size = ::KERNEL_HEAP_SIZE;

    map_heap(active_table);

    //slab allocator
    super::ALLOCATOR.init(offset.as_u32() as usize, size as usize);
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler");
}
