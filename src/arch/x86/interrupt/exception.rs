// https://wiki.osdev.org/Exceptions

macro_rules! exception {
    ($name:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(stack_frame: &mut ExceptionStackFrame)
        {
            println!("#{}", stringify!($name));
            println!("{:#?}", stack_frame);
            flush!();

            #[allow(unused_variables)]
            fn inner(stack: &mut ExceptionStackFrame) {
                $func
            }
            inner(stack_frame);
        }
    }
}

macro_rules! exception_err {
    ($name:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(
            stack_frame: &mut ExceptionStackFrame, error_code: u32)
        {
            println!("#{}({})", stringify!($name), error_code);
            println!("{:#?}", stack_frame);
            flush!();

            #[allow(unused_variables)]
            fn inner(stack: &mut ExceptionStackFrame) {
                $func
            }
            inner(stack_frame);
        }
    }
}

use x86::structures::idt::*;

exception!(divide_by_zero, {
    panic!("CPU exception: division by zero")
});

exception!(debug, {});
exception!(non_maskable, {});
exception!(breakpoint, {});
exception!(overflow, {});
exception!(bound_range, {});
exception!(invalid_opcode, {});
exception!(device_not_available, {});
exception_err!(double_fault, {
    panic!("double fault non recoverable");
});
exception!(coprocessor_segment_overrun, {});
exception_err!(invalid_tss, {});
exception_err!(segment_not_present, {});
exception_err!(stack_segment, {});
exception_err!(general_protection, {
    panic!("cannot recover from #GP");
});

pub extern "x86-interrupt" fn page_fault(
    stack_frame: &mut ExceptionStackFrame,
    code: PageFaultErrorCode,
) {
    use x86::registers::control::Cr2;
    println!("Exception: page_fault");
    println!("Error code: {:?}", code);
    println!("PFLA: {:?}", Cr2::read());
    println!("{:#?}", stack_frame);
    flush!();
    panic!("cannot recover from #PF")
}

exception!(x87_fpu, {});
exception_err!(alignment_check, {});
exception!(machine_check, {});
exception!(simd, {});
exception!(virtualization, {});
exception_err!(security, {});
