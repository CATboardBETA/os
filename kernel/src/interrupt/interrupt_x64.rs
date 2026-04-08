use crate::println;
use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handle_breakpoint);
    idt.double_fault.set_handler_fn(handle_double_fault);
    idt.page_fault.set_handler_fn(handle_page_fault);
    idt.alignment_check.set_handler_fn(handle_alignment_check);
    idt.general_protection_fault.set_handler_fn(handle_gpf);
    idt
});

pub fn init_interrupt_table() {
    IDT.load()
}

extern "x86-interrupt" fn handle_page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT: {error_code:?}\n{stack_frame:#?}")
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT ENCOUNTERED\n{stack_frame:#?}")
}

extern "x86-interrupt" fn handle_double_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn handle_alignment_check(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn handle_gpf(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}
