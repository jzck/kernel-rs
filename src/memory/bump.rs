use multiboot2::{MemoryArea, MemoryAreaIter};
use x86::*;
use x86::structures::paging::PhysFrame;
use super::FrameAllocator;

pub struct BumpFrameAllocator {
    next_free_frame: PhysFrame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: PhysFrame,
    kernel_end: PhysFrame,
    multiboot_start: PhysFrame,
    multiboot_end: PhysFrame,
}

impl BumpFrameAllocator {
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        memory_areas: MemoryAreaIter,
    ) -> BumpFrameAllocator {
        let mut allocator = BumpFrameAllocator {
            next_free_frame: PhysFrame { number: 0 },
            current_area: None,
            areas: memory_areas,
            kernel_start: PhysFrame::containing_address(PhysAddr::new(kernel_start as u32)),
            kernel_end: PhysFrame::containing_address(PhysAddr::new(kernel_end as u32)),
            multiboot_start: PhysFrame::containing_address(PhysAddr::new(multiboot_start as u32)),
            multiboot_end: PhysFrame::containing_address(PhysAddr::new(multiboot_end as u32)),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        // get next area with free frames
        self.current_area = self.areas
            .clone()
            .filter(|area| {
                area.end_address() >= self.next_free_frame.start_address().as_u32() as usize
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame =
                PhysFrame::containing_address(PhysAddr::new(area.start_address() as u32));
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for BumpFrameAllocator {
    fn allocate_frames(&mut self, count: usize) -> Option<PhysFrame> {
        if count == 0 {
            return None;
        };
        if let Some(area) = self.current_area {
            let start_frame = PhysFrame {
                number: self.next_free_frame.number,
            };
            let end_frame = PhysFrame {
                number: self.next_free_frame.number + count as u32 - 1,
            };

            let current_area_last_frame =
                PhysFrame::containing_address(PhysAddr::new(area.end_address() as u32));
            if end_frame > current_area_last_frame {
                // all frames are taken in this area
                self.choose_next_area();
            } else if (start_frame >= self.kernel_start && start_frame <= self.kernel_end)
                || (end_frame >= self.kernel_start && end_frame <= self.kernel_end)
            {
                // frame used by kernel
                self.next_free_frame = PhysFrame {
                    number: self.kernel_end.number + 1,
                };
            } else if (start_frame >= self.multiboot_start && start_frame <= self.multiboot_end)
                || (end_frame >= self.multiboot_start && end_frame <= self.multiboot_end)
            {
                // frame used by multiboot
                self.next_free_frame = PhysFrame {
                    number: self.multiboot_end.number + 1,
                };
            } else {
                self.next_free_frame.number += count as u32;
                return Some(start_frame);
            }
            // try again with next_free_frame
            self.allocate_frames(count)
        } else {
            None
        }
    }

    fn deallocate_frames(&mut self, frame: PhysFrame, count: usize) {
        // bump doesnt deallocate, must be used inside of a recycler
        println!(
            "lost frames {:#x} ({})",
            frame.start_address().as_u32(),
            count
        );
        // unimplemented!();
    }
}
