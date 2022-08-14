//! Memory related functions.

#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

use core::ptr;
use dismal::InstIter;

/// The size of a page.
pub const PAGE_SIZE: usize = 4096;

/// Mask used to obtain a page address from an arbitary address.
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

const RWX: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
const RX: i32 = libc::PROT_READ | libc::PROT_EXEC;

/// Set protection for the page of the given pointer.
#[inline]
pub unsafe fn protect<T>(ptr: *const T, protection: i32) {
    let page = ptr.map_addr(|addr| addr & PAGE_MASK);

    libc::mprotect(page as *mut libc::c_void, PAGE_SIZE, protection);
}

/// Temporarily disable protection for the page of the given pointer.
#[inline]
pub unsafe fn unprotect<T, F>(ptr: *const T, f: F)
where
    F: FnOnce(*mut T, i32) -> i32,
{
    protect(ptr, RWX);

    let prot = f(ptr as *mut T, RX);

    protect(ptr, prot)
}

/// Searches `ptr` for the next instruction that makes use of a relative address, then resolves and
/// returns the absolute address.
#[inline]
pub unsafe fn next_abs_addr<T>(base: *const T) -> *const T {
    let insts = InstIter::from_bytes(base.addr(), &*base.cast::<[u8; 15]>());

    for inst in insts {
        println!(
            "elysium | {:0x?} {:0x?} {:02X?}",
            inst.ip(),
            *inst,
            inst.to_bytes()
        );

        if let Some(addr) = inst.abs_addr() {
            return ptr::from_exposed_addr(addr);
        }
    }

    ptr::null()
}

/// Searches `ptr` for the next instruction that makes use of a relative address, then resolves and
/// returns the absolute address. Mutable variant.
#[inline]
pub unsafe fn next_abs_addr_mut<T>(base: *mut T) -> *mut T {
    next_abs_addr(base) as *mut T
}
