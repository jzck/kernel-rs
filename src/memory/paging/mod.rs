#![allow(dead_code)]

mod entry;
mod table;
mod temporary_page;
mod mapper;

use memory::PAGE_SIZE;
use memory::*;
use self::mapper::Mapper;
use self::temporary_page::TemporaryPage;
use core::ops::{Deref, DerefMut};
use multiboot2::BootInformation;
use x86;

pub use self::entry::*;
pub use self::table::*;

// x86 non PAE has 1024 entries per table
const ENTRY_COUNT: usize = 1024;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        // assert!(address < 0x0000_8000_0000_0000 ||
        //         address >= 0xffff_8000_0000_0000,
        //         "invalid addres: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p2_index(&self) -> usize {
        (self.number >> 10) & 0x3ff
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0x3ff
    }
}

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }

    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage,
                   f: F)
        where F: FnOnce(&mut Mapper)
        {
            let backup = Frame::containing_address(x86::cr3());

            // map temp page to current p2
            let p2_table = temporary_page.map_table_frame(backup.clone(), self);

            // overwrite recursive map
            self.p2_mut()[1023].set(table.p2_frame.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);
            x86::tlb::flush_all();

            // execute f in the new context
            f(self);

            // TODO restore recursive mapping to original p2 table
            p2_table[1023].set(backup, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {

        let p2_frame = Frame::containing_address(x86::cr3() as usize);

        println!("old p2_frame at {}", p2_frame.number);
        let old_table = InactivePageTable {
            p2_frame,
        };

        unsafe {
            let frame = Frame::containing_address(new_table.p2_frame.start_address());
            println!("new p2_frame at {:#x}", new_table.p2_frame.start_address());
            x86::cr3_write(frame.start_address());
        }

        old_table
    }
}

pub struct InactivePageTable {
    p2_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage,
               ) -> InactivePageTable {
        {
            let table = temporary_page.map_table_frame(frame.clone(),
            active_table);
            table.zero();

            // set up recursive mapping for the table
            table[1023].set(frame.clone(), EntryFlags::PRESENT | EntryFlags:: WRITABLE)
        }
        temporary_page.unmap(active_table);
        InactivePageTable { p2_frame: frame }
    }
}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation)
    -> ActivePageTable
    where A: FrameAllocator
{
    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafe },
                                                allocator);
    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {

        // identity map the VGA text buffer
        let vga_buffer_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_buffer_frame, EntryFlags::WRITABLE, allocator);

        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");

        for section in elf_sections_tag.sections() {
            use self::entry::EntryFlags;

            if !section.is_allocated() {
                continue;
            }
            assert!(section.start_address() % PAGE_SIZE as u64 == 0,
            "sections need to be page aligned");

            println!("mapping section at addr: {:#x}, size: {:#x}",
                     section.start_address(), section.size());

            let flags = EntryFlags::from_elf_section_flags(&section);
            let start_frame = Frame::containing_address(section.start_address() as usize);
            let end_frame = Frame::containing_address(section.end_address() as usize - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }

        let multiboot_start = Frame::containing_address(boot_info.start_address());
        let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, EntryFlags::PRESENT, allocator);
        }
    });

    let old_table = active_table.switch(new_table);

    let old_p2_page  = Page::containing_address(
        old_table.p2_frame.start_address()
        );

    active_table.unmap(old_p2_page, allocator);

    println!("guard page at {:#x}", old_p2_page.start_address());
    println!("cr3 = {:#x}", x86::cr3());

    active_table
}

pub fn test_paging<A>(allocator: &mut A)
    where A: FrameAllocator
{
    let mut page_table = unsafe { ActivePageTable::new() };

    let addr = 0xffff_f000;
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}",
             page_table.translate(addr),
             frame);
    println!("check 0");
    flush!();
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("check 1");
    flush!();
    println!("Some = {:?}", page_table.translate(addr));
    flush!();
    println!("next free frame: {:?}", allocator.allocate_frame());
    flush!();

}
