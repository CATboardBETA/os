//! An Operating System, in pure rust.[^justrust] Consider reading the `README.md` for general
//! information
//!
//! This is the *kernel* crate, which holds the main logic behind the whole of the operating
//! system. The bootloader, in this case [`limine`], calls our entrypoint [`kmain`].
//!
//! For documentation on how to run, either read `README.md` or
//!
//! [^justrust]: Although the kernel is written in pure rust, the bootloader is C.
#![cfg(not(test))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![cfg_attr(not(test), no_main)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
// TODO: remove unused allow
#![allow(unused)]

extern crate alloc;

use crate::gfx::{Color, Gfx};
use crate::reqs::FRAMEBUFFER;

mod alloc_handler;
mod gfx;
mod interrupt;
mod panic;
mod reqs;


/// # Safety
/// As this is the entrypoint, all safety expectations from the rest of the kernel must be upheld
///
/// One safety issue specific to [`kmain`] is the `limine` [`BaseRevision`](limine::BaseRevision).
/// If the `BaseRevision` is too low (what is considered too low depends on what features of
/// [the protocol](https://github.com/Limine-Bootloader/limine-protocol/blob/trunk/PROTOCOL.md) are used)
///
/// # Panics
///
/// This can technically panic. This can happen in one of a few cases:
/// - Any of the many limine requests fail (an irrecoverable error)
/// - There are no framebuffers
///   - TODO: can try VGA on `x86_64`
/// - The graphics functions return [a `DrawError`](gfx::DrawError)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain() -> ! {
    alloc_handler::init_global();
    #[cfg(target_arch = "x86_64")]
    interrupt::init_interrupt_table();

    let gfx = Gfx::from(
        *FRAMEBUFFER
            .response()
            .unwrap()
            .framebuffers()
            .first()
            .unwrap(),
    );

    gfx.fill_rect((40, 20), (700, 300), Color::BLUE);
    gfx.draw_line((10, 30), (550, 700), Color::RED);

    hcf();
}


/// Runs the appropriate assembler on loop, to halt
fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            core::arch::asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            core::arch::asm!("wfi");
        }
    }
}

#[cfg(test)]
fn main() {
    panic!("no no no no");
}
