use x86::structures::idt::*;

#[macro_use] pub mod exception;
#[macro_use] pub mod irq;


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
        idt[33].set_handler_fn(irq::keyboard);
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
