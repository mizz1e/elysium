//! Shared library/module/interface loading.
// TODO: update with similar code to https://github.com/elysian6969/csgo-launcher ui rebuild branch

use cake::ffi::CUtf8Str;
use daisy_chain::{Chain, ChainIter};
use elysium_sdk::LibraryKind;
use std::time::Duration;
use std::{fmt, ptr, thread};

/// An interface.
#[repr(C)]
pub struct Interface {
    new: unsafe extern "C" fn() -> *mut u8,
    name: *const libc::c_char,
    next: *mut Interface,
}

impl Interface {
    #[inline]
    pub fn new(&self) -> *mut u8 {
        let new = self.new;

        unsafe { new() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        unsafe { CUtf8Str::from_ptr(self.name).as_str() }
    }

    #[inline]
    fn next(&self) -> *mut Interface {
        self.next
    }
}

impl fmt::Debug for Interface {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Interface")
            .field("new", &self.new)
            .field("name", &self.name())
            .finish()
    }
}

type Next = fn(&Interface) -> *mut Interface;

/// Linked list of interfaces.
pub struct Interfaces {
    inner: Chain<Interface, Next>,
}

#[inline]
fn is_exact(target: &str) -> bool {
    target.chars().rev().take(3).all(char::is_numeric)
}

impl Interfaces {
    #[inline]
    pub const unsafe fn from_ptr(head: *mut Interface) -> Self {
        let inner = Chain::from_ptr(head, Interface::next as Next);

        Self { inner }
    }

    #[inline]
    pub const fn iter<'a>(&'a self) -> InterfaceIter<'a> {
        let inner = self.inner.iter();

        InterfaceIter { inner }
    }

    #[inline]
    pub fn get(&self, target: &str) -> *mut u8 {
        let cmp = if is_exact(target) {
            |name: &str, target: &str| name == target
        } else {
            |name: &str, target: &str| {
                let name = unsafe { name.get_unchecked(0..name.len().saturating_sub(3)) };

                name == target
            }
        };

        for interface in self.iter() {
            let name = interface.name();

            if cmp(name, target) {
                return interface.new();
            }
        }

        ptr::null_mut()
    }
}

impl fmt::Debug for Interfaces {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, fmt)
    }
}

pub struct InterfaceIter<'a> {
    inner: ChainIter<'a, Interface, Next>,
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = &'a Interface;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[inline]
pub fn load_interfaces() -> elysium_sdk::Interfaces {
    unsafe {
        elysium_sdk::Interfaces::from_loader(|kind| {
            let path = kind.library().path();
            let name = kind.name();

            println!("load {path:?} {name:?}");

            link::iterate_modules(|module| println!("{module:?}"));

            let result = link::load_module(path);
            let module = match result {
                Ok(module) => module,
                Err(error) => panic!("{error:?}"),
            };

            let address = module
                .symbol("s_pInterfaceRegs")
                .expect("interface registry")
                .symbol
                .address as *const *mut Interface;

            let interfaces = Interfaces::from_ptr(*address);
            let interface = interfaces.get(name);

            interface
        })
    }
}

#[inline]
pub fn wait_for_serverbrowser() {
    // `serverbrowser_client.so` is the last library to be loaded.
    println!("elysium | waiting for \x1b[38;5;2m`serverbrowser_client.so`\x1b[m to load");

    while !link::is_module_loaded(LibraryKind::ServerBrowser.path()) {
        thread::sleep(Duration::from_millis(500));
    }

    println!("elysium | \x1b[38;5;2m`serverbrowser_client.so`\x1b[m loaded, continuing...");
}
