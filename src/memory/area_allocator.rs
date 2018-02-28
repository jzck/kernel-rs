use memory::*;
use multiboot2::{MemoryAreaIter, MemoryArea};

pub struct AreaAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl AreaFrameAllocator {
    pub fn new(kernel_start: usize, kernel_end: usize
               multiboot_start: usize, multiboot_end: usize
               memory_areas: MemoryAreaIter) -> AreaAllocator {
        let mut allocator = AreaAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        }
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        // get next area with free frames
        self.current_area = self.areas.clone().filter(|area| {
            Frame::containing_address(area.end_address()) >= self.next_free_frame
        }).min_by_key(|area| area.start_addr());

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area.start_addr());
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = sef.current_area {
            let frame = Frame { number: self.next_free_frame.number },
            let current_area_last_frame = Frame::containing_address(area.end_address());
            if frame > current_area_last_frame {
                // all frames are taken in this area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= kernel_end {
                // frame used by kernel
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1;
                }
            } else if frame >= self.multiboot_start && frame <= multiboot_end {
                // frame used by multiboot
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1;
                }
            } else {
                self.next_free_frame_number += 1;
                return Some(Frame);
            }
        } else { None }
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!();
    }
}
