use crate::reqs::{HHDM, MEMMAP};
use limine::memmap::MEMMAP_USABLE;
use spin::Mutex;
use talc::source::Manual;
use talc::TalcLock;
use crate::println;

#[global_allocator]
static GLOBAL: TalcLock<Mutex<()>, Manual> = TalcLock::new(Manual);

pub(crate) fn init_global() {
    let offset = HHDM.response().unwrap().offset;
    let entries = MEMMAP.response().unwrap().entries();
    let mut total = 0;
    for entry in entries {
        if entry.type_ == MEMMAP_USABLE {
            unsafe {
                GLOBAL
                    .lock()
                    .claim((entry.base + offset) as _, entry.length as _);
            }
            println!("mapping addr {:#010X} to heap", entry.base);
            total += entry.length;
        }
    }

    println!("Total memory allocated: {:.5}GiB", bytesize::ByteSize::b(total).as_gib())
}
