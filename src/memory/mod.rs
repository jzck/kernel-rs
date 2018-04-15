mod bump;
mod recycle;
mod stack_allocator;

use multiboot2;
use x86::structures::paging::*;
use x86::*;
use spin::Mutex;

use self::bump::BumpFrameAllocator;
use self::recycle::RecycleAllocator;
use self::stack_allocator::StackAllocator;

pub trait FrameAllocator {
    fn allocate_frames(&mut self, size: usize) -> Option<PhysFrame>;
    fn deallocate_frames(&mut self, frame: PhysFrame, size: usize);
}

pub struct MemoryControler {
    frame_allocator: RecycleAllocator<BumpFrameAllocator>,
    stack_allocator: StackAllocator,
}

static MEMORY_CONTROLER: Mutex<Option<MemoryControler>> = Mutex::new(None);

pub fn init(boot_info: &multiboot2::BootInformation) {
    let elf_sections_tag = boot_info.elf_sections_tag().unwrap();
    let memory_map_tag = boot_info.memory_map_tag().unwrap();

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address())
        .min()
        .unwrap();

    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address() + s.size())
        .max()
        .unwrap();

    let bump_allocator = BumpFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas(),
    );

    let frame_allocator = RecycleAllocator::new(bump_allocator);

    let heap_end_page =
        Page::containing_address(VirtAddr::new(::HEAP_START as u32 + ::HEAP_SIZE as u32 - 1));

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = stack_alloc_start..stack_alloc_end + 1;
        StackAllocator::new(stack_alloc_range)
    };

    *MEMORY_CONTROLER.lock() = Some(MemoryControler {
        frame_allocator,
        stack_allocator,
    });
}

pub fn allocate_frames(count: usize) -> Option<PhysFrame> {
    if let Some(ref mut controler) = *MEMORY_CONTROLER.lock() {
        controler.frame_allocator.allocate_frames(count)
    } else {
        panic!("frame allocator not initialized!");
    }
}

pub fn deallocate_frames(frame: PhysFrame, count: usize) {
    if let Some(ref mut controler) = *MEMORY_CONTROLER.lock() {
        controler.frame_allocator.deallocate_frames(frame, count)
    } else {
        panic!("frame allocator not initialized!");
    }
}

pub fn allocate_stack() {
    if let Some(ref mut controler) = *MEMORY_CONTROLER.lock() {
        controler.stack_allocator.allocate_stack()
    } else {
        panic!("frame allocator not initialized!");
    }
}

/// Init memory module after core
/// Must be called once, and only once,
pub fn init_noncore() {
    if let Some(ref mut controler) = *MEMORY_CONTROLER.lock() {
        controler.frame_allocator.set_core(true);
    } else {
        panic!("frame allocator not initialized");
    }
}
