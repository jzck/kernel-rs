// https://wiki.osdev.org/Exceptions

use x86::structures::idt::*;

interrupt!(divide_by_zero, {});
interrupt!(debug, {});
interrupt!(non_maskable, {});
interrupt!(breakpoint, {
    println!("testing here dont mind me");
    flush!();
});
interrupt!(overflow, {});
interrupt!(bound_range, {});
interrupt!(invalid_opcode, {});
interrupt!(device_not_available, {});
interrupt_err!(double_fault, {});
interrupt!(coprocessor_segment_overrun, {});
interrupt_err!(invalid_tss, {});
interrupt_err!(segment_not_present, {});
interrupt_err!(stack_segment, {});
interrupt_err!(general_protection, {});

pub extern "x86-interrupt" fn page_fault(
    stack_frame: &mut ExceptionStackFrame, code: PageFaultErrorCode)
{
    println!("Exception: page_fault");
    println!("Error code: {:#b}", code);
    println!("{:#?}", stack_frame);
    flush!();
}

interrupt!(x87_fpu, {});
interrupt_err!(alignment_check, {});
interrupt!(machine_check, {});
interrupt!(simd, {});
interrupt!(virtualization, {});
interrupt_err!(security, {});
