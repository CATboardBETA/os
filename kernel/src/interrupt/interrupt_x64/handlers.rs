//! Submodule for `x86_64` handlers.
//! 
//! Note that these handlers are also known as `ISR`s, or Interrupt Service Routines.
#![allow(clippy::missing_docs_in_private_items)]
use crate::println;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub(super) extern "x86-interrupt" fn handle_page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT: {error_code:?}\n{stack_frame:#?}");
}

pub(super) extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT ENCOUNTERED\n{stack_frame:#?}");
}

pub(super) extern "x86-interrupt" fn handle_double_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}
pub(super) extern "x86-interrupt" fn handle_alignment_check(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}
pub(super) extern "x86-interrupt" fn handle_gpf(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}
