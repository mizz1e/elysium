#![deny(warnings)]
#![feature(arbitrary_self_types)]

use elysium_ptr::PtrMut;
use std::collections::HashMap;
use std::marker::PhantomPinned;

use module::Module;
use scope::Scope;
use system::System;

pub use error::Error;

mod error;
mod module;
mod scope;
mod system;

/// Application framework.
pub struct Framework {
    modules: HashMap<Box<str>, Module>,
    _pinned: PhantomPinned,
}

impl Framework {
    /// Construct a new application framework.
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            _pinned: PhantomPinned,
        }
    }

    /// Load a module.
    ///
    /// # Safety
    ///
    /// When loaded, initialization routines of a module are executed, which of course, may have
    /// unexpected side-effects.
    pub unsafe fn load(&mut self, module: &str) -> Result<(), Error> {
        let module_name = module;

        let module = Module::load(module_name)?;

        self.modules.insert(Box::from(module_name), module);

        Ok(())
    }

    /// Construct an interface from the specified module.
    ///
    /// # Safety
    ///
    /// Constructing certain interfaces may produce unexpected side-effects.
    pub unsafe fn new_interface<'a, T>(
        &mut self,
        module: &str,
        interface: &str,
    ) -> Result<PtrMut<'a, T>, Error> {
        let module_name = module;
        let interface_name = interface;

        let module = self
            .modules
            .get_mut(module_name)
            .ok_or(Error::UnknownModule)?;

        module.new_interface(interface_name)
    }

    /// Link an interface with other interfaces.
    ///
    /// # Safety
    ///
    /// Linking certain interfaces or not may produce unexpected side-effects.
    pub unsafe fn link(
        &mut self,
        interface: PtrMut<'_, ()>,
        with: &[(&str, PtrMut<'_, ()>)],
    ) -> Result<(), Error> {
        let interface = interface.cast::<System>();

        interface.link(with)
    }
}
