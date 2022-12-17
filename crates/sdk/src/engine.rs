use crate::{
    client_state, global, intrinsics,
    model_render::{self, RenderFlags},
    pattern, ClientState, IClientEntity, INetChannel, Ptr,
};

use bevy::prelude::*;
use std::ffi::{CStr, CString, OsStr};
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

    pub fn run_command(&self, command: &str) {
        let method: unsafe extern "C" fn(this: *mut u8, command: *const ffi::c_char) =
            unsafe { self.ptr.vtable_entry(113) };

        let Ok(command) = CString::new(command) else {
            return;
        };

        unsafe { (method)(self.ptr.as_ptr(), command.as_ptr()) }
    }

    pub fn is_in_game(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(26) };

        unsafe { (method)(self.ptr.as_ptr()) }
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

    let engine = link::load_module("engine_client.so").unwrap();
    let bytes = engine.bytes();
    let host_runframe_input = pattern::HOST_RUNFRAME_INPUT.find(bytes).unwrap().1;

    elysium_mem::next_abs_addr::<u8>(host_runframe_input);

    let addr = host_runframe_input.as_ptr().byte_add(196) as *mut u8;
    let rel = addr.cast::<i32>().read() as isize;
    let cl_move_ptr = addr.byte_add(4).byte_offset(rel);

    //let [a, b, c, d, e, f, g, h] = (cl_move as *mut u8).addr().to_ne_bytes();

    //let code = [
    //    0x48, 0xB8, a, b, c, d, e, f, g, h, // mov rax, addr
    //    0xFF, 0xE0, // jmp rax
    //];

    //ptr::replace_protected(cl_move_ptr.cast(), code);

    let cl_move = cl_move_ptr;

    tracing::trace!("CL_Move = {cl_move:?}");

    let addr = cl_move.byte_add(64) as *mut u8;
    let rel = addr.cast::<i32>().read() as isize;
    let host_should_run = addr.byte_add(4).byte_offset(rel);

    tracing::trace!("obtain GetBaseLocalClient");

    let addr = cl_move.byte_add(47) as *mut u8;
    let rel = addr.cast::<i32>().read() as isize;
    let get_base_local_client = addr.byte_add(4).byte_offset(rel);

    tracing::trace!("GetBaseLocalClient = {get_base_local_client:?}");
    tracing::trace!("obtain CL_SendMove");

    let addr = cl_move.byte_add(913) as *mut u8;
    let rel = addr.cast::<i32>().read() as isize;
    let cl_sendmove = addr.byte_add(4).byte_offset(rel);

    tracing::trace!("CL_SendMove = {cl_sendmove:?}");

    global::with_app_mut(|app| {
        app.insert_resource(InsertIntoTree(addr));

        //.insert_resource(HostShouldRun(mem::transmute(host_should_run)))
        app.insert_resource(client_state::GetBaseLocalClient(mem::transmute(
            get_base_local_client,
        )));
        //.insert_resource(ClSendMove(mem::transmute(cl_sendmove)))
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

/// See [disable model occlusion](https://www.unknowncheats.me/forum/counterstrike-global-offensive/330483-disable-model-occulusion.html).
pub unsafe extern "C" fn list_leaves_in_box(
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

    // `CClientLeafSystem::InsertIntoTree` in `game/client/clientleafsystem.cpp`.
    if return_addr != insert_into_tree {
        return (method)(this, min, max, list, list_max);
    }

    // Get RenderableInfo_t from stack.
    let info = &mut **(frame_addr.byte_add(2392) as *const *mut model_render::RenderableInfo_t);

    // Get IClientRenderable from RenderableInfo_t.
    let ptr = info.renderable.byte_sub(mem::size_of::<*mut u8>()) as *mut u8;
    let renderable = Ptr::new("IClientRenderable", ptr).unwrap();

    // Get IClientEntity from IClientRenderable.
    let index: unsafe extern "C" fn(this: *mut u8) -> ffi::c_int =
        unsafe { renderable.vtable_entry(8) };
    let index = (index)(renderable.as_ptr());

    let Some(entity) = IClientEntity::from_index(index) else {
        return (method)(this, min, max, list, list_max);
    };

    if !entity.is_player() {
        return (method)(this, min, max, list, list_max);
    }

    // Fix render order, force translucent group.
    //
    // See `CClientLeafSystem::AddRenderablesToRenderLists` in `game/client/clientleafsystem.cpp`.
    info.flags.remove(RenderFlags::FORCE_OPAQUE_PASS);
    info.flags2.insert(RenderFlags::BOUNDS_ALWAYS_RECOMPUTE);

    let max_coord = Vec3::splat(16_384.0);
    let min_coord = Vec3::splat(-16_384.0);

    (method)(this, &min_coord, &max_coord, list, list_max)
}
