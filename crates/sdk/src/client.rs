use crate::{
    global, networked, ptr, CGlobalVarsBase, CInput, ClientClass, IClientEntityList, IClientMode,
    IVEngineClient, Ptr,
};
use bevy::prelude::*;
use std::{ffi, mem};

const FRAME_NET_UPDATE_END: ffi::c_int = 4;
const FRAME_RENDER_START: ffi::c_int = 5;
const FRAME_RENDER_END: ffi::c_int = 6;

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
pub struct OriginalViewAngle(pub(crate) Vec3);

#[derive(Resource)]
pub struct IBaseClientDLL {
    pub(crate) ptr: Ptr,
}

impl IBaseClientDLL {
    #[inline]
    pub(crate) unsafe fn setup(&self) {
        tracing::trace!("setup IBaseClientDLL");

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

            let activate_mouse = self.ptr.vtable_entry::<ptr::FnPtr>(16) as *const u8;
            let hud_update = self.ptr.vtable_entry::<ptr::FnPtr>(11) as *const u8;

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

            networked::setup(self.all_classes());
        });
    }

    #[inline]
    pub(crate) fn all_classes(&self) -> *const ClientClass {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ClientClass =
            unsafe { self.ptr.vtable_entry(8) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    #[inline]
    fn deactivate_mouse(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) = unsafe { self.ptr.vtable_entry(15) };

        unsafe {
            (method)(self.ptr.as_ptr());
        }
    }

    #[inline]
    fn activate_mouse(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) = unsafe { self.ptr.vtable_entry(16) };

        unsafe {
            (method)(self.ptr.as_ptr());
        }
    }

    #[inline]
    unsafe fn setup_client_mode(&self) -> IClientMode {
        tracing::trace!("obtain IClientMode");

        let hud_process_input = self.ptr.vtable_entry::<ptr::FnPtr>(10) as *const u8;
        let call_client_mode = hud_process_input.byte_add(11);
        let client_mode = elysium_mem::next_abs_addr_ptr::<u8>(call_client_mode)
            .unwrap_or_else(|| panic!("unable to find IClientMode"));

        let client_mode: unsafe extern "C" fn() -> *mut u8 = mem::transmute(client_mode);
        let ptr = client_mode();
        let ptr =
            Ptr::new("IClientMode", ptr).unwrap_or_else(|| panic!("unable to find IClientMode"));

        let client_mode = IClientMode { ptr };

        client_mode.setup();
        client_mode
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
    let method = global::with_app_mut(|app| {
        if !app.world.contains_resource::<IClientMode>() {
            let client = app.world.resource::<IBaseClientDLL>();

            app.insert_resource(client.setup_client_mode());
        }

        let engine = app.world.resource::<IVEngineClient>();
        let entity_list = app.world.resource::<IClientEntityList>();
        let view_angle = engine.view_angle();
        let local_player_index = engine.local_player_index();

        match frame {
            FRAME_RENDER_START => {
                if let Some(player) = entity_list.get(local_player_index) {
                    app.insert_resource(OriginalViewAngle(player.view_angle()));
                    player.set_view_angle(view_angle + Vec3::new(0.0, 0.0, 15.0));
                }

                app.update();
            }
            FRAME_RENDER_END => {
                if let Some(original_view_angle) = app.world.get_resource::<OriginalViewAngle>() {
                    if let Some(player) = entity_list.get(local_player_index) {
                        player.set_view_angle(original_view_angle.0);
                    }
                }
            }
            _ => {}
        }

        app.world.resource::<FrameStageNotify>().0
    });

    (method)(this, frame)
}
