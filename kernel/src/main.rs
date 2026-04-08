#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]
#![no_main]

extern crate alloc;

use crate::gfx::{Display, TextInfo};
use crate::reqs::{BASE_REVISION, FRAMEBUFFER};
use alloc::vec;
#[cfg(not(test))]
use core::panic::PanicInfo;
use gfx::DISPLAY;

mod crt;
mod gfx;
mod reqs;

mod alloc_handler;
mod interrupt;

/// # Safety
/// I mean, it's the entry point. What could go wrong?
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain() -> ! {
    DISPLAY.0.lock().call_once(|| Display {
        inner: FRAMEBUFFER
            .response()
            .unwrap()
            .framebuffers()
            .first()
            .unwrap(),
        text_info: TextInfo { pos: (0, 20) },
    });
    interrupt::init_interrupt_table();
    alloc_handler::init_global();

    hcf();
}

#[panic_handler]
#[cfg(not(test))]
fn rust_panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hcf()
}

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
