use crate::{global, intrinsics, pattern, IClientEntity, INetChannel, Ptr};
use bevy::prelude::*;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{ffi, mem};

/// `engine/cdll_engine_int.cpp`.
#[derive(Resource)]
pub struct IVEngineClient {
    pub(crate) ptr: Ptr,
}

impl IVEngineClient {
    pub fn view_angle(&self) -> Vec3 {
        let method: unsafe extern "C" fn(this: *mut u8, view_angle: *mut Vec3) =
            unsafe { self.ptr.vtable_entry(18) };

        let mut view_angle = Vec3::ZERO;

        unsafe {
            (method)(self.ptr.as_ptr(), &mut view_angle);
        }

        view_angle
    }

    pub fn set_view_angle(&self, view_angle: Vec3) {
        let method: unsafe extern "C" fn(this: *mut u8, view_angle: *const Vec3) =
            unsafe { self.ptr.vtable_entry(19) };

        unsafe {
            (method)(self.ptr.as_ptr(), &view_angle);
        }
    }

    pub fn level_name(&self) -> Option<Box<OsStr>> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(53) };

        let level_name = unsafe { (method)(self.ptr.as_ptr()) };

        if level_name.is_null() {
            return None;
        }

        let level_name = unsafe { CStr::from_ptr(level_name).to_bytes() };

        if level_name.is_empty() {
            None
        } else {
            Some(Box::from(OsStr::from_bytes(level_name)))
        }
    }

    pub fn local_player_index(&self) -> i32 {
        let method: unsafe extern "C" fn(this: *mut u8) -> i32 =
            unsafe { self.ptr.vtable_entry(12) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn net_channel(&self) -> Option<INetChannel> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(78) };

        let ptr = unsafe { (method)(self.ptr.as_ptr()) };
        let ptr = Ptr::new("INetChannel", ptr)?;

        Some(INetChannel { ptr })
    }

    pub fn bsp_tree_query(&self) -> Option<BSPTreeQuery> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(43) };

        let ptr = unsafe { (method)(self.ptr.as_ptr()) };
        let ptr = Ptr::new("BSPTreeQuery", ptr)?;

        Some(BSPTreeQuery { ptr })
    }
}

pub struct BSPTreeQuery {
    pub(crate) ptr: Ptr,
}

impl BSPTreeQuery {
    pub unsafe fn setup(&self) {
        global::with_app_mut(|app| {
            app.insert_resource(ListLeavesInBox(
                self.ptr.vtable_replace(6, list_leaves_in_box),
            ));
        });
    }
}

pub unsafe fn setup() {
    tracing::trace!("obtain CClientLeafSystem::InsertIntoTree");

    let module = link::load_module("client_client.so").unwrap();
    let bytes = module.bytes();
    let opcode = &pattern::INSERT_INTO_TREE.find(bytes).unwrap().1;
    let addr = opcode.as_ptr().byte_add(31) as *mut u8;

    tracing::trace!("CClientLeafSystem::InsertIntoTree = {addr:?}");

    global::with_app_mut(|app| {
        app.insert_resource(InsertIntoTree(addr));
    });
}

#[derive(Resource)]
pub struct InsertIntoTree(pub(crate) *mut u8);

unsafe impl Send for InsertIntoTree {}
unsafe impl Sync for InsertIntoTree {}

#[derive(Resource)]
pub struct ListLeavesInBox(
    pub(crate)  unsafe extern "C" fn(
        this: *mut u8,
        min: *const Vec3,
        max: *const Vec3,
        list: *const ffi::c_ushort,
        list_max: ffi::c_int,
    ) -> ffi::c_int,
);

unsafe extern "C" fn list_leaves_in_box(
    this: *mut u8,
    min: *const Vec3,
    max: *const Vec3,
    list: *const ffi::c_ushort,
    list_max: ffi::c_int,
) -> ffi::c_int {
    let frame_addr = intrinsics::frame_addr(0);
    let return_addr = intrinsics::return_addr(0);

    let (insert_into_tree, method) = global::with_app(|app| {
        let insert_into_tree = app.world.resource::<InsertIntoTree>().0;
        let list_leaves_in_box = app.world.resource::<ListLeavesInBox>().0;

        (insert_into_tree, list_leaves_in_box)
    });

    // `CClientLeafSystem::InsertIntoTree` @ `game/client/clientleafsystem.cpp`
    if return_addr == insert_into_tree {
        let info = &**(frame_addr.byte_add(2392) as *const *const internal::RenderableInfo_t);
        let ptr = info.renderable.byte_sub(mem::size_of::<*mut u8>()) as *mut u8;
        let renderable = Ptr::new("IClientRenderable", ptr).unwrap();
        let index: unsafe extern "C" fn(this: *mut u8) -> ffi::c_int =
            unsafe { renderable.vtable_entry(8) };
        let index = (index)(renderable.as_ptr());

        if let Some(entity) = IClientEntity::from_index(index) {
            let is_player = entity.is_player();

            if entity.is_player() {
                let max = Vec3::splat(16384.0);
                let min = -max;

                return (method)(this, &min, &max, list, list_max);
            }
        }
    }

    (method)(this, min, max, list, list_max)
}

mod internal {
    use std::ffi;

    #[repr(C)]
    pub struct IClientRenderable;

    #[repr(C)]
    pub struct CClientAlphaProperty;

    /// `struct RenderableInfo_t` @ `game/client/clientleafsystem.cpp`
    #[repr(C)]
    pub struct RenderableInfo_t {
        pub renderable: *const IClientRenderable,
        pub alpha_property: *const CClientAlphaProperty,
        pub enum_count: ffi::c_int,
        pub render_frame: ffi::c_int,
        pub first_shadow: ffi::c_ushort,
        pub leaf_list: ffi::c_ushort,
        pub area: ffi::c_short,
        pub flags: u16,
        // TODO: Add the rest of the fields. Reason I haven't is due to the fact I cannot be
        // bothered with figuring out the layout of C bitfields. Besides, these fields are not very
        // important.
    }
}
