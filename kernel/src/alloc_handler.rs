use crate::reqs::{HHDM, MEMMAP};
use limine::memmap::MEMMAP_USABLE;
use spin::Mutex;
use talc::source::Manual;
use talc::TalcLock;

#[global_allocator]
static GLOBAL: TalcLock<Mutex<()>, Manual> = TalcLock::new(Manual);

/// This cannot use println, for println use the allocator. doing so will cause a double fault.
pub fn init_global() {
    let offset = HHDM.response().unwrap().offset;
    let memmap = MEMMAP.response().unwrap().entries();
    for entry in memmap {
        if entry.type_ == MEMMAP_USABLE {
            unsafe {
                GLOBAL
                    .lock()
                    .claim((entry.base + offset) as _, entry.length as _);
            }
        }
    }
}
