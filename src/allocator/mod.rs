pub use self::slab::Allocator;
mod slab;

use x86::*;
use x86::structures::paging::*;
use arch::x86::paging::*;

fn map_heap(active_table: &mut ActivePageTable, offset: u32, size: u32) {
    let heap_start_page = Page::containing_address(VirtAddr::new(offset));
    let heap_end_page = Page::containing_address(VirtAddr::new(offset + size - 1));

    for page in heap_start_page..heap_end_page + 1 {
        //we really should only map 1 huge page instead of 1024 small pages
        active_table.map(page, PageTableFlags::WRITABLE);
    }
}

/// should be called only once
pub unsafe fn init(active_table: &mut ActivePageTable) {
    let offset = ::KERNEL_HEAP_OFFSET;
    let size = ::KERNEL_HEAP_SIZE;

    map_heap(active_table, offset, size);

    Allocator::init(offset as usize, size as usize);
}
