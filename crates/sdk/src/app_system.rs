use core::{fmt, mem};

pub mod method {
    use core::ffi;

    pub type CreateInterface<T> =
        unsafe extern "C" fn(name: *const ffi::c_char, status: &mut super::InitResult) -> *mut T;

    pub type Connect<T> =
        unsafe extern "C" fn(this: *const T, create_interface: CreateInterface<T>) -> bool;

    pub type Disconnect<T> = unsafe extern "C" fn(this: *const T);

    pub type QueryInterface<T> =
        unsafe extern "C" fn(this: *const T, name: *const ffi::c_char) -> *mut T;

    pub type Init<T> = unsafe extern "C" fn(this: *const T) -> ffi::c_int;

    pub type Shutdown<T> = unsafe extern "C" fn(this: *const T) -> ffi::c_int;

    pub type Dependencies<T> = unsafe extern "C" fn(this: *const T) -> *const ();

    pub type Tier<T> = unsafe extern "C" fn(this: *const T) -> super::Tier;

    pub type Reconnect<T> = unsafe extern "C" fn(this: *const T) -> ffi::c_int;

    pub type IsSingleton<T> = unsafe extern "C" fn(this: *const T) -> ffi::c_int;
}

#[derive(Debug)]
#[repr(u32)]
pub enum InitResult {
    Err = 0,
    Ok = 1,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Tier {
    Tier0 = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
    Other = 4,
}

#[repr(C)]
pub struct AppSystemVTable<T> {
    connect: method::Connect<T>,
    disconnect: method::Disconnect<T>,
    query_interface: method::QueryInterface<T>,
    init: method::Init<T>,
    shutdown: method::Shutdown<T>,
    dependencies: method::Dependencies<T>,
    tier: method::Tier<T>,
    reconnect: method::Reconnect<T>,
    is_singleton: method::IsSingleton<T>,
}

impl<T> AppSystemVTable<T> {
    #[inline]
    pub const fn new(
        connect: method::Connect<T>,
        disconnect: method::Disconnect<T>,
        query_interface: method::QueryInterface<T>,
        init: method::Init<T>,
        shutdown: method::Shutdown<T>,
        dependencies: method::Dependencies<T>,
        tier: method::Tier<T>,
        reconnect: method::Reconnect<T>,
        is_singleton: method::IsSingleton<T>,
    ) -> Self {
        Self {
            connect,
            disconnect,
            query_interface,
            init,
            shutdown,
            dependencies,
            tier,
            reconnect,
            is_singleton,
        }
    }

    #[inline]
    pub fn cast<U>(self) -> AppSystemVTable<U> {
        // SAFETY: AppSystemVTable doesn't actually store T, so the layout shouldn't ever differ
        // for U.
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn tier(&self, this: *const T) -> Tier {
        unsafe { (self.tier)(this) }
    }
}

// Manually implement debug as derive(Debug) emits T: Debug.
impl<T> fmt::Debug for AppSystemVTable<T> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("AppSystemVTable")
            .field("connect", &self.connect)
            .field("disconnect", &self.disconnect)
            .field("query_interface", &self.query_interface)
            .field("init", &self.init)
            .field("shutdown", &self.shutdown)
            .field("dependencies", &self.dependencies)
            .field("tier", &self.tier)
            .field("reconnect", &self.reconnect)
            .field("is_singleton", &self.is_singleton)
            .finish()
    }
}
