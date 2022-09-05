use super::Table;
use crate::entity::EntityId;
use cake::ffi::CUtf8Str;
use core::fmt;

#[non_exhaustive]
#[repr(C)]
pub struct Class {
    new: unsafe extern "C" fn(entity: i32, serial: i32) -> *const u8,
    new_event: unsafe extern "C" fn() -> *const u8,
    name: *const libc::c_char,
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
    pub fn name(&self) -> Box<str> {
        unsafe {
            let name = CUtf8Str::from_ptr(self.name).as_str();

            Box::from(name)
        }
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
