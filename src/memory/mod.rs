mod area_allocator;
mod heap_allocator;
mod stack_allocator;
mod paging;

pub use self::area_allocator::*;
pub use self::heap_allocator::*;
pub use self::stack_allocator::*;
pub use self::paging::remap_the_kernel;
use multiboot2;
use x86::*;
use x86::structures::paging::*;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame>;
    fn deallocate_frame(&mut self, frame: PhysFrame);
}

pub struct MemoryController {
    active_table: paging::ActivePageTable,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController { ref mut active_table,
                                      ref mut frame_allocator,
                                      ref mut stack_allocator } = self;

        stack_allocator.alloc_stack(active_table, frame_allocator,
                                    size_in_pages)
    }
}

/// memory initialisation should only be called once
pub fn init(boot_info: &multiboot2::BootInformation) -> MemoryController {
    use x86::registers::control::{Cr0, Cr4, Cr0Flags, Cr4Flags};
    Cr4::add(Cr4Flags::PSE);
    Cr0::add(Cr0Flags::PAGING | Cr0Flags::WRITE_PROTECT);

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
        boot_info.start_address(), boot_info.end_address(),
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

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = stack_alloc_start..stack_alloc_end + 1;
        StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        active_table,
        frame_allocator,
        stack_allocator,
    }
}
