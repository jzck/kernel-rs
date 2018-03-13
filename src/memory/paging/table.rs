use memory::*;
use x86::structures::paging::*;


pub trait TableNext<A>
    where A: FrameAllocator
{
    fn next_table_address(&self, index: usize) -> Option<usize>;
    fn next_table(&self, index: usize) -> Option<&PageTable>;
    fn next_table_mut(&mut self, index: usize) -> Option<&mut PageTable>;
    fn next_table_create<A>(&mut self,
                                index: usize,
                                allocator: &mut A) -> &mut PageTable;
}

impl TableNext<> for PageTable
{
    fn next_table_create<A>(&mut self,
                                index: usize,
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
