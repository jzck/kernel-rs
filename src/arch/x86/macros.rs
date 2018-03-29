#[macro_export]
macro_rules! interrupt {
    ($name:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(stack_frame: &mut ExceptionStackFrame)
        {
            println!("Exception: {}", stringify!($name));
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

#[macro_export]
macro_rules! interrupt_err {
    ($name:ident, $func:block) => {
        pub extern "x86-interrupt" fn $name(
            stack_frame: &mut ExceptionStackFrame, _error_code: u32)
        {
            println!("Exception: {}", stringify!($name));
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
