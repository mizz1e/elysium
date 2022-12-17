use crate::{Error, Ptr};
use bevy::prelude::*;
use std::collections::HashMap;
use std::ffi;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct ModuleInner {
    name: Box<str>,
    handle: libloading::Library,
    interfaces: HashMap<Box<str>, Ptr>,
}

/// A module.
#[derive(Clone, Debug)]
pub struct Module(Arc<Mutex<ModuleInner>>);

impl Module {
    /// Find and load a dynamic library.
    ///
    /// # Safety
    ///
    /// See [`Library::new`](libloading::Library::new).
    unsafe fn open(name: impl AsRef<str>) -> Result<Self, Error> {
        let name = name.as_ref();
        let handle = libloading::Library::new(name)?;

        tracing::trace!("added module: {name:?}");

        Ok(Module(Arc::new(Mutex::new(ModuleInner {
            name: Box::from(name),
            handle,
            interfaces: HashMap::new(),
        }))))
    }

    /// Module's name.
    pub fn name(&self) -> Box<str> {
        Box::from(&*self.0.lock().expect("module mutex poisoned").name)
    }

    /// Get a pointer to a function or static by symbol name.
    ///
    /// # Safety
    ///
    /// See [`Library::get`](libloading::Library::get).
    pub unsafe fn symbol<T: Copy>(&self, symbol: impl AsRef<[u8]>) -> Result<T, Error> {
        let symbol = symbol.as_ref();
        let symbol: T = *self
            .0
            .lock()
            .expect("module mutex poisoned")
            .handle
            .get(symbol)?;

        Ok(symbol)
    }

    /// Get a pointer to an interface.
    ///
    /// # Safety
    ///
    /// - Invokes a foreign function (`CreateInterface`).
    /// - See [`Library::get`](libloading::Library::get).
    pub unsafe fn create_interface(&self, name: impl AsRef<str>) -> Result<Ptr, Error> {
        {
            #[repr(C)]
            pub struct Interface {
                pub new: unsafe extern "C" fn() -> *mut ffi::c_void,
                pub name: *const ffi::c_char,
                pub next: *const Interface,
            }

            let interface_list: *const *const Interface = self.symbol("s_pInterfaceRegs\0")?;
            let mut interface_list: *const Interface = *interface_list;

            while let Some(interface) = interface_list.as_ref() {
                interface_list = interface.next;

                println!("{:?}", ffi::CStr::from_ptr(interface.name));
            }
        }

        // Explicit nul-termination saves the need to allocate.
        let create_interface: unsafe extern "C" fn(
            *const ffi::c_char,
            result: *mut i32,
        ) -> *mut u8 = self.symbol("CreateInterface\0")?;

        let name = name.as_ref();
        let cname = ffi::CString::new(name)?;

        let mut result = 1;
        let interface = (create_interface)(cname.as_ptr(), &mut result);

        if result != 0 {
            return Err(Error::Interface(Box::from(name)));
        }

        // Extra check on top of result claiming it's okay.
        let interface =
            Ptr::new("Interface", interface).ok_or_else(|| Error::Interface(Box::from(name)))?;

        tracing::trace!("added interface: {name:?} from module {:?}", self.name());

        self.0
            .lock()
            .expect("module mutex poisoned")
            .interfaces
            .insert(Box::from(name), interface.clone());

        Ok(interface)
    }

    /// Get a pointer to an existing interface.
    pub fn get_interface(&self, name: &str) -> Option<Ptr> {
        self.0
            .lock()
            .expect("module mutex poisoned")
            .interfaces
            .get(name)
            .cloned()
    }
}

/// A map of modules.
#[derive(Debug, Default, Resource)]
pub struct ModuleMap {
    modules: HashMap<Box<str>, Module>,
}

impl ModuleMap {
    /// Find and load a module.
    ///
    /// # Safety
    ///
    /// See [`Library::new`](libloading::Library::new).
    pub unsafe fn open(&mut self, name: impl AsRef<str>) -> Result<&Module, Error> {
        let name = name.as_ref();
        let contains_key = self.modules.contains_key(name);

        if contains_key {
            let module_ref = self.modules.get(name).unwrap();

            Ok(module_ref)
        } else {
            let module = Module::open(name)?;
            let module_ref = &*self.modules.entry(Box::from(name)).or_insert(module);

            Ok(module_ref)
        }
    }

    /// Get a reference to a loaded module.
    pub fn get_module(&self, module: &str) -> Option<&Module> {
        self.modules.get(module)
    }

    /// Get a pointer to an existing interface in the provided module.
    pub fn get_interface(&self, module: &str, interface: &str) -> Option<Ptr> {
        self.get_module(module)?.get_interface(interface)
    }
}
