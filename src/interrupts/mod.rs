use x86::structures::idt::*;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[0x21].set_handler_fn(::keyboard::kbd_callback);
        idt
    };
}

pub fn init(memory_controller: &mut ::memory::MemoryController) {
    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");
    // println!("DF stack: {:#?}", double_fault_stack);
    // flush!();
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame, _error_code: u32)
{
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    flush!();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    flush!();
}
