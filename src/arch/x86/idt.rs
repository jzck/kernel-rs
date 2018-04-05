use x86::structures::idt::*;
use super::interrupt::*;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // set up CPU exceptions
        idt.breakpoint.set_handler_fn(exception::breakpoint);
        idt.debug.set_handler_fn(exception::debug);
        idt.non_maskable_interrupt.set_handler_fn(exception::non_maskable);
        idt.breakpoint.set_handler_fn(exception::breakpoint);
        idt.overflow.set_handler_fn(exception::overflow);
        idt.bound_range_exceeded.set_handler_fn(exception::bound_range);
        idt.invalid_opcode.set_handler_fn(exception::invalid_opcode);
        idt.device_not_available.set_handler_fn(exception::device_not_available);
        idt.double_fault.set_handler_fn(exception::double_fault);
        idt.segment_not_present.set_handler_fn(exception::segment_not_present);
        idt.stack_segment_fault.set_handler_fn(exception::stack_segment);
        idt.general_protection_fault.set_handler_fn(exception::general_protection);
        idt.page_fault.set_handler_fn(exception::page_fault);
        idt.x87_floating_point.set_handler_fn(exception::x87_fpu);
        idt.alignment_check.set_handler_fn(exception::alignment_check);
        idt.machine_check.set_handler_fn(exception::machine_check);
        idt.simd_floating_point.set_handler_fn(exception::simd);
        idt.virtualization.set_handler_fn(exception::virtualization);

        // set up IRQs
        idt[32].set_handler_fn(irq::pit);
        idt[33].set_handler_fn(irq::keyboard);
        idt[34].set_handler_fn(irq::cascade);
        idt[35].set_handler_fn(irq::com2);
        idt[36].set_handler_fn(irq::com1);
        idt[37].set_handler_fn(irq::lpt2);
        idt[38].set_handler_fn(irq::floppy);
        idt[39].set_handler_fn(irq::lpt1);
        idt[40].set_handler_fn(irq::rtc);
        idt[41].set_handler_fn(irq::pci1);
        idt[42].set_handler_fn(irq::pci2);
        idt[43].set_handler_fn(irq::pci3);
        idt[44].set_handler_fn(irq::mouse);
        idt[45].set_handler_fn(irq::fpu);
        idt[46].set_handler_fn(irq::ata1);
        idt[47].set_handler_fn(irq::ata2);

        idt
    };
}

// pub fn init(memory_controller: &mut ::memory::MemoryController) {
pub fn init() {
    // let double_fault_stack = memory_controller.alloc_stack(1)
    //     .expect("could not allocate double fault stack");
    // println!("DF stack: {:#?}", double_fault_stack);
    // flush!();
    IDT.load();
}
