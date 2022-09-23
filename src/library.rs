//! interfaced youre mother

use cake::ffi::CUtf8Str;
use elysium_sdk::LibraryKind;
use std::time::Duration;
use std::{fmt, mem, ptr, thread};

#[inline]
fn is_exact(target: &str) -> bool {
    target.chars().rev().take(3).all(char::is_numeric)
}

/// An interface.
#[repr(C)]
pub struct Interface {
    new: unsafe extern "C" fn() -> *mut u8,
    name: CUtf8Str<'static>,
    next: *const Interface,
}

impl Interface {
    #[inline]
    pub fn new(&self) -> *mut u8 {
        unsafe { (self.new)() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn iter(&self) -> InterfaceIter<'_> {
        InterfaceIter { iter: Some(self) }
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

impl fmt::Debug for Interface {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Interface")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

pub struct InterfaceIter<'a> {
    iter: Option<&'a Interface>,
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = &'a Interface;

    #[inline]
    fn next(&mut self) -> Option<&'a Interface> {
        let next = unsafe { self.iter?.next.as_ref() };

        mem::replace(&mut self.iter, next)
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
                .address as *const *const Interface;

            let interfaces = &**address;
            let interface = interfaces.get(name);

            interface
        })
    }
}

#[inline]
pub fn wait_for_serverbrowser() {
    use std::borrow::Cow;
    use std::collections::HashSet;

    let mut modules = HashSet::new();

    while !link::is_module_loaded(LibraryKind::ServerBrowser.path()) {
        let mut new_modules = HashSet::new();

        link::iterate_modules(|module| {
            new_modules.insert(module.path);
        });

        let yes = new_modules
            .iter()
            .filter(|path| !modules.contains(&**path))
            .flat_map(|path| Some(path.file_name()?.to_str()?))
            .intersperse(&Cow::Borrowed(", "))
            .collect::<String>();

        println!("{yes}");

        modules = new_modules;
    }
}
