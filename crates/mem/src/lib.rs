//! Memory related functions.

#![feature(ptr_sub_ptr)]
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

/// Searches `pointer` for the next instruction that makes use of a relative address, then resolves and
/// returns the absolute address.
#[inline]
pub fn next_abs_addr<T>(bytes: &[u8]) -> Option<*const T> {
    let start_ip = bytes.as_ptr().addr();
    let insts = InstIter::from_bytes(start_ip, bytes);

    for inst in insts {
        let ip = inst.ip();
        let abs_addr = inst.abs_addr();
        let inst = *inst;
        let bytes = inst.to_bytes();

        if let Some(addr) = abs_addr {
            let addr = ptr::from_exposed_addr(addr);

            let ip_diff = ip - start_ip;

            log::trace!("disam {ip:0x?} {inst:0x?} {bytes:02X?} -> {addr:0x?} (we want this)");
            log::trace!("  start ip = {start_ip:?}");
            log::trace!("  end ip = {ip:?}");
            log::trace!("  ip diff = {ip_diff:?}");
            log::trace!(
                "  bytes = {:02X?}",
                bytes.get(..(ip_diff + bytes.len())).unwrap()
            );

            return Some(addr);
        } else {
            log::trace!("disam {ip:0x?} {inst:0x?} {bytes:02X?} (ignored)");
        }
    }

    None
}

/// Searches `pointer` for the next instruction that makes use of a relative address, then resolves and
/// returns the absolute address. Mutable variant.
#[inline]
pub fn next_abs_addr_mut<T>(bytes: &mut [u8]) -> Option<*mut T> {
    next_abs_addr::<T>(bytes).map(|pointer| pointer as *mut T)
}

/// Searches `pointer` for the next instruction that makes use of a relative address, then resolves and
/// returns the absolute address.
#[inline]
pub unsafe fn next_abs_addr_ptr<T>(pointer: *const u8) -> Option<*const T> {
    let address = link::query_address(pointer)?;
    let module_bytes = address.module.bytes();
    let offset = pointer.sub_ptr(module_bytes.as_ptr());
    let bytes = module_bytes.get_unchecked(offset..);

    next_abs_addr(bytes)
}

/// Searches `pointer` for the next instruction that makes use of a relative address, then resolves and
/// returns the absolute address.
#[inline]
pub unsafe fn next_abs_addr_mut_ptr<T>(pointer: *mut u8) -> Option<*mut T> {
    next_abs_addr_ptr::<T>(pointer).map(|pointer| pointer as *mut T)
}
