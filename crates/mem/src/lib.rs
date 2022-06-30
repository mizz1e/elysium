//! Memory related functions.

#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]
#![feature(strict_provenance)]

use core::{mem, ptr};
use dismal::{Inst, InstIter};
use frosting::ffi::CSignature;

pub use shared::Shared;

mod shared;

/// The size of a page.
pub const PAGE_SIZE: usize = 4096;

/// Mask used to obtain a page address from an arbitary address.
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

const UNPROTECTED: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

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

/// Set protection for the page of the given pointer.
#[inline]
pub unsafe fn protect<T>(ptr: *const T, protection: i32) {
    let page = ptr.map_addr(|addr| addr & PAGE_MASK);

    libc::mprotect(page as *mut libc::c_void, PAGE_SIZE, protection);
}

/// Disable protection for the page of the given pointer.
///
/// Convenience function for `protect(ptr, READ | WRITE | EXECUTE)`.
#[inline]
pub unsafe fn unprotect<T>(ptr: *const T) -> i32 {
    protect(ptr, UNPROTECTED);

    libc::PROT_READ | libc::PROT_EXEC
}

/// Obtain the address of an `extern "C"` function pointer.
#[inline]
pub unsafe fn fn_addr<F>(f: F) -> usize
where
    F: CSignature,
{
    mem::transmute_copy(&f)
}

/// Calculate a relative offset for `f` based on the given `ip`.
#[inline]
pub unsafe fn relative_of<T, F>(ip: *const T, f: F) -> i32
where
    F: CSignature,
{
    let addr = fn_addr(f) as *const T;
    let rel = addr.byte_offset_from(ip);

    if !((i32::MIN as isize)..=(i32::MAX as isize)).contains(&rel) {
        panic!("outside i32 range");
    }

    rel as i32
}

/// Rewrite code at `ptr` to `call`/`jmp`/`lea` to `hook`, rather than whatever is there by default
#[inline]
pub unsafe fn rewrite_code<T, F>(ptr: *mut T, hook: F)
where
    F: CSignature + Copy,
{
    let mut code = Vec::new();
    let insts = InstIter::from_bytes(ptr.addr(), &*ptr.cast::<[u8; 15]>());

    println!("elysium | original code");

    for inst in insts {
        println!(
            "elysium | {:0x?} {:0x?} {:02X?}",
            inst.ip(),
            *inst,
            inst.to_bytes()
        );

        match *inst {
            Inst::Call(_) => {
                // relative addresses are resolved via the rip address after the current
                // instruction.
                let ip = inst.next_ip();
                let new_rel = relative_of(ip as *const u8, hook);
                let inst = Inst::Call(new_rel);

                code.extend_from_slice(&inst.to_bytes());
            }
            Inst::Ret => {
                code.extend_from_slice(&Inst::Ret.to_bytes());

                break;
            }
            inst => code.extend_from_slice(&inst.to_bytes()),
        }
    }

    println!("elysium | new code");

    let insts = InstIter::from_bytes(ptr.addr(), code.as_slice());

    for inst in insts {
        println!(
            "elysium | {:0x?} {:0x?} {:02X?}",
            inst.ip(),
            *inst,
            inst.to_bytes()
        );
    }

    println!("about to write");
    protect(ptr, libc::PROT_READ | libc::PROT_WRITE);

    ptr::copy_nonoverlapping(code.as_ptr(), ptr.cast::<u8>(), code.len());
    println!("written");
}
