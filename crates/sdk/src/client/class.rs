use super::Table;
use crate::entity::EntityId;
use crate::ffi;
use core::fmt;

#[non_exhaustive]
#[repr(C)]
pub struct Class {
    new: unsafe extern "C" fn(entity: i32, serial: i32) -> *const u8,
    new_event: unsafe extern "C" fn() -> *const u8,
    name: *const u8,
    pub table: Option<&'static Table>,
    pub(super) next: *mut Class,
    pub entity_id: EntityId,
}

impl Class {
    #[inline]
    pub fn new(&self, entity: i32, serial: i32) -> *const u8 {
        unsafe { (self.new)(entity, serial) }
    }

    #[inline]
    pub fn new_event(&self) -> *const u8 {
        unsafe { (self.new_event)() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        unsafe { ffi::str_from_ptr_nullable(self.name) }
    }
}

impl fmt::Debug for Class {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Class")
            .field("name", &self.name())
            .field("table", &self.table)
            .field("entity_id", &self.entity_id)
            .finish()
    }
}
