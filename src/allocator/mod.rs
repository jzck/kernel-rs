pub use self::slab::Allocator;
mod slab;

use x86::*;
use x86::structures::paging::*;
use arch::x86::paging::*;

fn map_heap(active_table: &mut ActivePageTable, offset: usize, size: usize)
{
    let heap_start_page = Page::containing_address(VirtAddr::new(offset as u32));
    let heap_end_page = Page::containing_address(VirtAddr::new(
            offset as u32 + size as u32 - 1));

    for page in heap_start_page..heap_end_page + 1 {
        active_table.map(page, PageTableFlags::WRITABLE);
    }
}

/// should be called only once
pub unsafe fn init(active_table: &mut ActivePageTable) {
    let offset = ::HEAP_START;
    let size = ::HEAP_SIZE;

    map_heap(active_table, offset, size);

    Allocator::init(offset, size);
}
