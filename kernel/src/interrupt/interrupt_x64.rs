//! Interrupt handlers, descriptor table, etc. for `x86_64`

use core::ops::IndexMut;
use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

mod handlers;
mod irq;

/// Interrupt Descriptor Table. The [`x86_64`] crate is kind enough to provide a `struct` for
/// handling these. For each interrupt type, you simply run
/// `idt.<interrupt>.set_handler_fn(<handler>)`, and the `IDT` will call the handler whenever the
/// interrupt is triggered.
static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    use handlers::{
        handle_alignment_check, handle_breakpoint, handle_double_fault, handle_gpf,
        handle_page_fault,
    };
    // TODO: Add all the other exceptions n stuff here
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handle_breakpoint);
    idt.double_fault.set_handler_fn(handle_double_fault);
    idt.page_fault.set_handler_fn(handle_page_fault);
    idt.alignment_check.set_handler_fn(handle_alignment_check);
    idt.general_protection_fault.set_handler_fn(handle_gpf);
    idt
});

/// Currently, this function only initiallizes the `x86_64` [`IDT`]. However, it will eventually
/// also have helpers for the APIC
pub fn init_interrupt_table() {
    IDT.load();
}

