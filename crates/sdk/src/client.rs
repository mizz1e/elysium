use crate::{global, CGlobalVarsBase, CInput, IClientMode, Ptr};
use bevy::prelude::*;
use std::{ffi, mem};

#[derive(Resource)]
pub struct LevelInitPreEntity(
    pub(crate) unsafe extern "C" fn(this: *mut u8, path: *const ffi::c_char),
);

#[derive(Resource)]
pub struct LevelInitPostEntity(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct LevelShutdown(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct FrameStageNotify(pub(crate) unsafe extern "C" fn(this: *mut u8, frame: ffi::c_int));

#[derive(Resource)]
pub struct IBaseClientDLL {
    pub(crate) ptr: Ptr,
}

impl IBaseClientDLL {
    #[inline]
    pub(crate) unsafe fn setup(&self) {
        global::with_app_mut(|app| {
            app.insert_resource(LevelInitPreEntity(
                self.ptr.vtable_replace(5, level_init_pre_entity),
            ));

            app.insert_resource(LevelInitPostEntity(
                self.ptr.vtable_replace(6, level_init_post_entity),
            ));

            app.insert_resource(LevelShutdown(self.ptr.vtable_replace(7, level_shutdown)));

            app.insert_resource(FrameStageNotify(
                self.ptr.vtable_replace(37, frame_stage_notify),
            ));

            let activate_mouse = self.ptr.vtable_entry(16) as *const u8;
            let hud_process_input = self.ptr.vtable_entry(10) as *const u8;
            let hud_update = self.ptr.vtable_entry(11) as *const u8;

            let call_client_mode = hud_process_input.byte_add(11);
            let client_mode = elysium_mem::next_abs_addr_ptr::<u8>(call_client_mode)
                .unwrap_or_else(|| panic!("unable to find IClientMode"));

            let client_mode: unsafe extern "C" fn() -> *mut u8 = mem::transmute(client_mode);
            let ptr = client_mode();
            let ptr = Ptr::new("IClientMode", ptr)
                .unwrap_or_else(|| panic!("unable to find IClientMode"));

            let client_mode = IClientMode { ptr };

            client_mode.setup();

            app.insert_resource(client_mode);

            let address = hud_update.byte_add(13);
            let ptr = *elysium_mem::next_abs_addr_ptr::<*mut u8>(address)
                .unwrap_or_else(|| panic!("unable to find CGlobalVarsBase"));

            let ptr = Ptr::new("CGlobalVarsBase", ptr)
                .unwrap_or_else(|| panic!("unable to find CGlobalVarsBase"));

            app.insert_resource(CGlobalVarsBase { ptr });

            let ptr = **elysium_mem::next_abs_addr_ptr::<*const *mut u8>(activate_mouse)
                .unwrap_or_else(|| panic!("unable to find CInput"));

            let ptr = Ptr::new("CInput", ptr).unwrap_or_else(|| panic!("unable to find CInput"));

            app.insert_resource(CInput { ptr });
        });
    }

    fn all_classes(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(8) };

        let classes = unsafe { (method)(self.ptr.as_ptr()) };
    }

    fn deactivate_mouse(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) = unsafe { self.ptr.vtable_entry(15) };

        unsafe {
            (method)(self.ptr.as_ptr());
        }
    }

    fn activate_mouse(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) = unsafe { self.ptr.vtable_entry(16) };

        unsafe {
            (method)(self.ptr.as_ptr());
        }
    }
}

unsafe extern "C" fn level_init_pre_entity(this: *mut u8, path: *const ffi::c_char) {
    let method = global::with_app(|app| app.world.resource::<LevelInitPreEntity>().0);

    (method)(this, path)
}

unsafe extern "C" fn level_init_post_entity(this: *mut u8) {
    let method = global::with_app(|app| app.world.resource::<LevelInitPostEntity>().0);

    (method)(this)
}

unsafe extern "C" fn level_shutdown(this: *mut u8) {
    let method = global::with_app(|app| app.world.resource::<LevelShutdown>().0);

    (method)(this)
}

unsafe extern "C" fn frame_stage_notify(this: *mut u8, frame: ffi::c_int) {
    let method = global::with_app(|app| app.world.resource::<FrameStageNotify>().0);

    (method)(this, frame)
}
