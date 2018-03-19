pub const PAGE_SIZE: usize = 4096;

mod area_allocator;
mod heap_allocator;
mod paging;

pub use self::area_allocator::*;
pub use self::heap_allocator::*;
pub use self::paging::remap_the_kernel;
use multiboot2;
use x86::*;
use x86::structures::paging::*;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame>;
    fn deallocate_frame(&mut self, frame: PhysFrame);
}

/// memory initialisation should only be called once
pub fn init(boot_info: &multiboot2::BootInformation) {
    let elf_sections_tag = boot_info.elf_sections_tag().unwrap();
    let memory_map_tag = boot_info.memory_map_tag().unwrap();

    let kernel_start = elf_sections_tag.sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address())
        .min().unwrap();

    let kernel_end = elf_sections_tag.sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address() + s.size())
        .max().unwrap();

    let mut frame_allocator = self::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize,
        boot_info.start_address(), boot_info.start_address(),
        memory_map_tag.memory_areas());

    let mut active_table = paging::remap_the_kernel(&mut frame_allocator,
                                                    boot_info);
    use {HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(
        VirtAddr::new(HEAP_START as u32));
    let heap_end_page = Page::containing_address(
        VirtAddr::new(HEAP_START as u32 + HEAP_SIZE as u32 - 1));

    for page in heap_start_page..heap_end_page {
        active_table.map(page, PageTableFlags::WRITABLE, &mut frame_allocator);
    }
}
