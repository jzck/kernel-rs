use super::{VirtualAddress, PhysicalAddress, Page, ENTRY_COUNT};
use super::entry::*;
use super::table::{self, Table, Level2, Level1};
use memory::{PAGE_SIZE, Frame, FrameAllocator};
use core::ptr::Unique;

pub struct Mapper {
    p2: Unique<Table<Level2>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper {
            p2: Unique::new_unchecked(table::P2),
        }
    }

    // the remaining mapping methods, all public
    pub fn p2(&self) -> &Table<Level2> {
        unsafe { self.p2.as_ref() }
    }

    pub fn p2_mut(&mut self) -> &mut Table<Level2> {
        unsafe { self.p2.as_mut() }
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress>
    {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    pub fn translate_page(&self, page: Page) -> Option<Frame> {

        let p1 = self.p2().next_table(page.p2_index());

        let huge_page = || {
            let p2_entry = &self.p2()[page.p2_index()];
            if let Some(start_frame) = p2_entry.pointed_frame() {
                if p2_entry.flags().contains(EntryFlags::HUGE_PAGE) {
                    // 2MiB alignment check
                    assert!(start_frame.number % ENTRY_COUNT == 0);
                    return Some(Frame {
                        number: start_frame.number + page.p1_index()
                    });
                }
            }
            None
        }; 

        p1.and_then(|p1| p1[page.p1_index()].pointed_frame())
          .or_else(huge_page)
    }


    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags,
                     allocator: &mut A)
        where A: FrameAllocator
        {
            let p2 = self.p2_mut();
            let p1 = p2.next_table_create(page.p2_index(), allocator);

            assert!(p1[page.p1_index()].is_unused());
            p1[page.p1_index()].set(frame, flags | EntryFlags::PRESENT);
        }

    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
        {
            let frame = allocator.allocate_frame().expect("out of memory");
            self.map_to(page, frame, flags, allocator)
        }

    pub fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
        {
            let page = Page::containing_address(frame.start_address());
            self.map_to(page, frame, flags, allocator);
        }

    pub fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
        {
            assert!(self.translate(page.start_address()).is_some());

            let p1 = self.p2_mut()
                .next_table_mut(page.p2_index())
                .expect("mapping code does not support huge pages");
            let frame = p1[page.p1_index()].pointed_frame().unwrap();
            p1[page.p1_index()].set_unused();
            // TODO flush the tlb
            allocator.deallocate_frame(frame);
        }
}
