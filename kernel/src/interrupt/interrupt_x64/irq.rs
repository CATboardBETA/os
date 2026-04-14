//! This module exists purely just to prevent [`interrupt_x64`](super) from becoming to bloated
//! with a huge enum. This is also why the only item in this module is only `pub(super)`

/// This was primarily just taken from [the osdev wiki](https://wiki.osdev.org/Interrupts#General_IBM-PC_Compatible_Interrupt_Information)
///
// I think this is exhaustive of what I will actually get, but just in case...
#[non_exhaustive]
pub(super) enum Irq {
    /// Programmable Interrupt Timer Interrupt
    Pit,
    /// Keyboard interrupts yippee
    Keyboard,
    /// Never triggered, used internally by PICs
    Cascade,
    /// If enabled
    Com2,
    /// If enabled
    Com1,
    /// If enabled
    Lpt2,
    /// Floppy disk
    Floppy,
    /// Spurious interrupt
    Lpt1,
    /// A clock, on CMOS
    CmosClock,
    /// Any of these
    FreeScsiNic1,
    /// Any of these
    FreeScsiNic2,
    /// Any of these
    FreeScsiNic3,
    /// Mouse
    Ps2Mouse,
    /// Any of these
    FpuCoprocessorInterprocessor,
    /// Primary
    AtaHard1,
    /// Secondary
    AtaHard2,
}