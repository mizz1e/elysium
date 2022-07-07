use daisy_chain::{Chain, ChainIter};
use link::Library;
use std::time::Duration;
use std::{fmt, ptr, thread};

/// An interface.
#[repr(C)]
pub struct Interface {
    new: unsafe extern "C" fn() -> *mut (),
    name: *const u8,
    next: *mut Interface,
}

impl Interface {
    #[inline]
    pub fn new(&self) -> *mut () {
        let new = self.new;

        unsafe { new() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        unsafe { elysium_sdk::ffi::str_from_ptr_nullable(self.name) }
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
    pub fn get(&self, target: &str) -> *mut () {
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
        elysium_sdk::Interfaces::from_loader(|interface_kind| {
            let library_kind = interface_kind.library();
            let library = match Library::load(library_kind.as_nul_str()) {
                Ok(library) => library,
                Err(error) => panic!("Failed to load library: {library_kind:?}: {error:?}"),
            };

            let symbol: *mut Interface = match library.symbol("s_pInterfaceRegs\0") {
                Some(symbol) => symbol,
                None => panic!("Failed to find interfaces within library: {library_kind:?}"),
            };

            let interfaces = Interfaces::from_ptr(symbol);
            let interface = interfaces.get(interface_kind.as_str());

            println!("elysium | loaded interface \x1b[38;5;2m{interface_kind:?}\x1b[m (\x1b[38;5;2m{:?}\x1b[m) within \x1b[38;5;2m{library_kind:?}\x1b[m (\x1b[38;5;2m{:?}\x1b[m) at \x1b[38;5;3m{interface:?}\x1b[m", interface_kind.as_str(), library_kind.as_str());

            interface
        })
    }
}

#[inline]
pub fn wait_for_serverbrowser() {
    // `serverbrowser_client.so` is the last library to be loaded.
    println!("elysium | waiting for \x1b[38;5;2m`serverbrowser_client.so`\x1b[m to load");

    while !Library::is_loaded("./bin/linux64/serverbrowser_client.so") {
        thread::sleep(Duration::from_millis(500));
    }

    println!("elysium | \x1b[38;5;2m`serverbrowser_client.so`\x1b[m loaded, continuing...");
}
