//! Memory related functions.

#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]
#![feature(strict_provenance)]

use core::mem;
use frosting::ffi::CSignature;

pub use shared::Shared;

mod shared;

/// The size of a page.
pub const PAGE_SIZE: usize = 4096;

/// Mask used to obtain a page address from an arbitary address.
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

const UNPROTECTED: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

/// Creates a new pointer with the given address and size.
#[inline]
pub unsafe fn to_absolute<T>(base: *const T, addr: isize, size: usize) -> *const T {
    base.map_addr(|base| (base as isize + addr) as usize)
        .byte_add(size)
}

unsafe fn offset_of<T>(base: *const T) -> isize {
    base.cast::<i32>().read() as isize
}

/// Magic.
#[inline]
pub unsafe fn to_absolute_with_offset<T>(base: *const T, offset: usize, size: usize) -> *const T {
    let offset_address = base.byte_add(offset);
    let offset = offset_of(offset_address);

    base.byte_offset(offset).byte_add(size)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    CallR32,
    CallM32,
    JmpM32,
}

impl Op {
    /// Attempt to disassemble `bytes` (very limited, only does `call` and `jmp`).
    #[inline]
    pub const fn from_bytes(bytes: &[u8; 2]) -> Option<Self> {
        let op = match bytes {
            // call r32
            [0xE8, _] => Op::CallR32,
            // jmp m32
            [0xFF, 0x25] => Op::JmpM32,
            // call m32
            [0xFF, _] => Op::CallM32,
            _ => return None,
        };

        Some(op)
    }

    /// Returns the relative address offset.
    #[inline]
    pub const fn offset(&self) -> usize {
        match self {
            Op::CallR32 => 1,
            Op::CallM32 => 2,
            Op::JmpM32 => 2,
        }
    }

    /// Returns the instruction length.
    #[inline]
    pub const fn len(&self) -> usize {
        match self {
            Op::CallR32 => 4,
            Op::CallM32 => 5,
            Op::JmpM32 => 5,
        }
    }
}

/// Obtain the address of an `extern "C"` function.
#[inline]
pub unsafe fn fn_addr<F>(f: F) -> usize
where
    F: CSignature,
{
    mem::transmute_copy(&f)
}

#[inline]
pub unsafe fn relative_of<T, F>(ip: *const T, f: F) -> i32
where
    F: CSignature,
{
    let addr = fn_addr(f) as *const T;

    addr.byte_offset_from(ip) as i32
}

/// Hook a function.
#[inline]
pub unsafe fn hook<T, F>(ptr: *const T, hook: F) -> Option<F>
where
    F: CSignature + Copy,
{
    let hook_addr = fn_addr(hook) as *const u8;

    println!("elysium | hooking {ptr:?} w/ {hook_addr:?}");

    // Read the first two bytes (enough to determine the opcode).
    let opcode = ptr.cast::<[u8; 2]>().read();

    println!("elysium | bytes = {opcode:02X?}");

    let opcode = Op::from_bytes(&opcode)?;

    println!("elysium | opcode = {opcode:?}");

    let offset = opcode.offset();
    let size = opcode.len();
    let original_fn = to_absolute_with_offset(ptr, offset, size);
    let rel = ptr.byte_add(offset).cast::<i32>();
    let rip = rel.byte_add(size).as_mut();
    let relative = relative_of(rip, hook);

    println!("elysium | relative = {relative:?}");

    //let protection = unprotect(rip);
    rip.write(relative);
    println!("wrote relative");
    //protect(rip, protection);

    Some(mem::transmute_copy(&original_fn))
}

#[cfg(test)]
mod tests {
    const CODE: [u8; 6] = [0xFF, 0x25, 0xCA, 0xFC, 0x32, 0x00];
    const ADDRESS: isize = i32::from_le_bytes([0xCA, 0xFC, 0x32, 0x00]) as isize;

    #[test]
    fn to_absolute() {
        unsafe {
            let code = CODE.as_ptr();
            let rip = std::ptr::invalid::<u8>(0);
            let addr = code.byte_add(2).cast::<i32>().read() as isize;
            let dest = super::to_absolute(rip, addr, 6);

            assert_eq!(dest, rip.byte_offset(ADDRESS).byte_add(6));
        }
    }
}
