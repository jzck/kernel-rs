use memory::*;
use x86::structures::paging::*;
use x86::ux::*;

pub trait RecTable
{
    fn next_table_address(&self, index: u10) -> Option<u32>;
    fn next_table(&self, index: u10) -> Option<&PageTable>;
    fn next_table_mut(&mut self, index: u10) -> Option<&mut PageTable>;
    fn next_table_create<A: FrameAllocator>(&mut self,
                             index: u10,
                             allocator: &mut A)
            -> &mut PageTable;
}

impl RecTable for PageTable
{
    fn next_table_address(&self, index: u10) -> Option<u32> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PageTableFlags::PRESENT) && !entry_flags.contains(PageTableFlags::HUGE_PAGE) {
            let table_address = self as *const _ as u32;
            Some(table_address << 10 | u32::from(index << 12))
        } else {
            None
        }
    }

    fn next_table(&self, index: u10) -> Option<&PageTable> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    fn next_table_mut(&mut self, index: u10) -> Option<&mut PageTable> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    fn next_table_create<A>(&mut self,
                         index: u10,
                         allocator: &mut A) -> &mut PageTable
        where A: FrameAllocator
        {
            if self.next_table(index).is_none() {
                assert!(!self[index].flags().contains(PageTableFlags::HUGE_PAGE),
                "mapping code does not support huge pages");
                let frame = allocator.allocate_frame().expect("no frames available");
                self[index].set(frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
                self.next_table_mut(index).expect("next_table_mut gave None").zero()
            }
            self.next_table_mut(index).expect("no next table 2")
        }
}
