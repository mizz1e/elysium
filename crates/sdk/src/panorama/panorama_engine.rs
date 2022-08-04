use super::UIEngine;
use cake::ffi::VTablePad;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<11>,
    access_ui_engine: unsafe extern "thiscall" fn(this: *const PanoramaUIEngine) -> *const UIEngine,
}

/// Panorama UI Engine.
#[repr(C)]
pub struct PanoramaUIEngine {
    vtable: &'static VTable,
}

impl PanoramaUIEngine {
    #[inline]
    pub fn access_ui_engine(&self) -> *const UIEngine {
        unsafe { (self.vtable.access_ui_engine)(self) }
    }
}
