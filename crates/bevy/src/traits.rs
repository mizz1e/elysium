use {crate::internal::Library, bevy::prelude::*, std::ops::Deref};

/// A Source engine module.
pub trait Module: Deref<Target = Library> + Resource {
    const IDENT: &'static str;

    #[doc(hidden)]
    fn from_world(world: &mut World) -> Self;
}

/// A Source engine interface.
pub trait Interface: Resource {
    const IDENT: &'static str;

    #[doc(hidden)]
    fn from_world(world: &mut World) -> Self;
}

/// [`App`](App) extension.
pub trait AppExt {
    fn init_interface<I: Interface>(&mut self);
    fn init_module<M: Module>(&mut self);
}

/// [`World`](World) extension.
pub trait WorldExt {
    fn init_interface<I: Interface>(&mut self);
    fn init_module<M: Module>(&mut self);
    fn interface<I: Interface>(&self) -> &I;
    fn interface_mut<I: Interface>(&mut self) -> Mut<'_, I>;
    fn module<M: Module>(&self) -> &M;
    fn module_mut<M: Module>(&mut self) -> Mut<'_, M>;
}

impl AppExt for App {
    #[inline]
    fn init_interface<I: Interface>(&mut self) {
        self.world.init_interface::<I>()
    }

    #[inline]
    fn init_module<M: Module>(&mut self) {
        self.world.init_module::<M>()
    }
}

impl WorldExt for World {
    #[inline]
    fn init_interface<I: Interface>(&mut self) {
        let interface = I::from_world(self);

        self.insert_resource(interface);
    }

    #[inline]
    fn init_module<M: Module>(&mut self) {
        let module = M::from_world(self);

        self.insert_resource(module);
    }

    #[inline]
    fn interface<I: Interface>(&self) -> &I {
        self.resource()
    }

    #[inline]
    fn interface_mut<I: Interface>(&mut self) -> Mut<'_, I> {
        self.resource_mut()
    }

    #[inline]
    fn module<M: Module>(&self) -> &M {
        self.resource()
    }

    #[inline]
    fn module_mut<M: Module>(&mut self) -> Mut<'_, M> {
        self.resource_mut()
    }
}
