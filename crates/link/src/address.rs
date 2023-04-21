use super::module::{iterate_modules, Module};
use std::ffi::{CStr, OsStr};
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStrExt;

/// Address information.
#[derive(Debug)]
pub struct Address {
    /// The module this address exists within.
    pub module: Module,

    /// Symbol information related to the address.
    pub symbol: Option<Symbol>,
}

/// Symbol address information.
///
/// Contains the same information as [`Address`], but `symbol` is never `None`.
#[derive(Debug)]
pub struct SymbolAddress {
    /// The module this address exists within.
    pub module: Module,

    /// Symbol information related to the address.
    pub symbol: Symbol,
}

/// Symbol information.
#[derive(Debug)]
pub struct Symbol {
    /// The address of the symbol (may be different to the address provided to [`query_address`]).
    pub address: *const u8,

    /// The symbol name associated with this address.
    pub name: Box<OsStr>,
}

/// Obtains the symbol name from the `dli_sname` pointer.
#[inline]
unsafe fn symbol_name(name: *const libc::c_char) -> Option<Box<OsStr>> {
    if name.is_null() {
        None
    } else {
        let name = CStr::from_ptr(name).to_bytes();
        let name = Box::from(OsStr::from_bytes(name));

        Some(name)
    }
}

/// Actual implementation of the query.
#[inline]
unsafe fn query_address_inner<T: ?Sized>(address: *const T) -> Option<Address> {
    let mut info = MaybeUninit::uninit();
    let result = libc::dladdr(address.cast(), info.as_mut_ptr());

    if result == 0 {
        return None;
    }

    let info = MaybeUninit::assume_init(info);
    let address = info.dli_saddr.cast::<u8>();
    let symbol = (!address.is_null())
        .then(|| {
            let name = symbol_name(info.dli_sname)?;

            Some(Symbol { address, name })
        })
        .flatten();

    let module_address = info.dli_fbase.cast::<u8>();
    let mut current_module = None;

    iterate_modules(|module| {
        if module.address == module_address {
            current_module = Some(module);
        }
    });

    let module = current_module.expect("module");

    Some(Address { module, symbol })
}

/// Query information about an address.
#[inline]
pub fn query_address<T: ?Sized>(address: *const T) -> Option<Address> {
    unsafe { query_address_inner(address) }
}
