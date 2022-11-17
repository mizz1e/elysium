//! Memory related functions.

#![feature(ptr_sub_ptr)]
#![feature(strict_provenance)]

use core::ptr;
use dismal::InstIter;

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

            log::trace!("{ip:0x?} {inst:0x?} {bytes:02X?} -> {addr:0x?}");

            return Some(addr);
        } else {
            log::trace!("{ip:0x?} {inst:0x?} {bytes:02X?} (ignored)");
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
