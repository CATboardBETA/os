#![cfg(not(test))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![cfg_attr(not(test), no_main)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
// TODO: remove unused allow
#![allow(unused)]

extern crate alloc;

use crate::gfx::{Color, Gfx};
use crate::reqs::FRAMEBUFFER;

mod crt;
mod gfx;
mod reqs;

mod alloc_handler;
mod interrupt;

/// # Safety
/// I mean, it's the entry point. What could go wrong?
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

    gfx.fill_rect((40, 20), (700, 300), Color::BLUE).unwrap();
    gfx.draw_line((10,30), (550, 700), Color::RED).unwrap();


    hcf();
}

mod panic;

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
