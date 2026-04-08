// Copyright © 2026, __robot@PLT
// SPDX-License-Identifier: MIT

#![allow(unsafe_op_in_unsafe_fn)]

use core::ffi::c_int;

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(str: *const u8) -> usize {
    let mut cur = str;
    while cur.read_volatile() != 0 {
        cur = cur.add(1);
    }
    cur.offset_from_unsigned(str)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, val: c_int, size: usize) -> *mut u8 {
    let val = val as u8;
    for i in 0..size {
        dest.add(i).write_volatile(val);
    }
    dest
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const i8, b: *const i8, size: usize) -> c_int {
    for i in 0..size {
        let a = a.add(i).read_volatile();
        let b = b.add(i).read_volatile();
        if a != b {
            return a as c_int - b as c_int;
        }
    }
    0
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    memmove(dest, src, size)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    if (dest as *const u8) < src {
        for i in 0..size {
            (dest.add(i)).write_volatile(src.add(i).read_volatile());
        }
    } else {
        for i in (0..size).rev() {
            (dest.add(i)).write_volatile(src.add(i).read_volatile());
        }
    }

    dest
}
