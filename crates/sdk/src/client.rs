use crate::{
    convar, gl, global, material, networked, ptr, sdl, CGlobalVarsBase, CInput, CUserCmd,
    ClientClass, Config, IClientEntity, IClientMode, ICvar, IMaterialSystem, IPhysicsSurfaceProps,
    IVEngineClient, KeyValues, ModuleMap, Ptr,
};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use iced_native::Point;
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
            let ptr = **elysium_mem::next_abs_addr_ptr::<*const *mut u8>(activate_mouse)
                .unwrap_or_else(|| panic!("unable to find CInput"));

            let ptr = Ptr::new("CInput", ptr).unwrap_or_else(|| panic!("unable to find CInput"));

            app.insert_resource(CInput { ptr });

            networked::setup(self.all_classes());
        });
    }

    pub(crate) fn all_classes(&self) -> *const ClientClass {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ClientClass =
            unsafe { self.ptr.vtable_entry(8) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

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

    unsafe fn setup_global_vars(&self) -> CGlobalVarsBase {
        tracing::trace!("obtain CGlobalVarsBase");

        let hud_update = self.ptr.vtable_entry::<ptr::FnPtr>(11) as *const u8;
        let address = hud_update.byte_add(13);
        let ptr = *elysium_mem::next_abs_addr_ptr::<*mut u8>(address)
            .unwrap_or_else(|| panic!("unable to find CGlobalVarsBase"));

        let ptr = Ptr::new("CGlobalVarsBase", ptr)
            .unwrap_or_else(|| panic!("unable to find CGlobalVarsBase"));

        tracing::trace!("CGlobalVarsBase = {:?}", ptr.as_ptr());

        CGlobalVarsBase { ptr }
    }
}

unsafe extern "C" fn level_init_pre_entity(this: *mut u8, path: *const ffi::c_char) {
    debug_assert!(!this.is_null());

    let method = global::with_resource::<LevelInitPreEntity, _>(|method| method.0);

    (method)(this, path)
}

unsafe extern "C" fn level_init_post_entity(this: *mut u8) {
    debug_assert!(!this.is_null());

    let method = global::with_resource::<LevelInitPostEntity, _>(|method| method.0);

    (method)(this)
}

unsafe extern "C" fn level_shutdown(this: *mut u8) {
    debug_assert!(!this.is_null());

    let method = global::with_resource::<LevelShutdown, _>(|method| method.0);

    (method)(this)
}

unsafe extern "C" fn frame_stage_notify(this: *mut u8, frame: ffi::c_int) {
    debug_assert!(!this.is_null());

    let method = global::with_app_mut(|app| {
        if !app.world.contains_resource::<IClientMode>() {
            let context = gl::setup();

            app.insert_resource(context);

            let (poll_event, swap_window) = sdl::setup();

            app.insert_resource(poll_event);
            app.insert_resource(swap_window);
            app.insert_resource(sdl::CursorPosition(Point::ORIGIN));

            let client = app.world.resource::<IBaseClientDLL>();
            let client_mode = client.setup_client_mode();
            let global_vars = client.setup_global_vars();

            app.insert_resource(client_mode);
            app.insert_resource(global_vars);

            let module_map = app.world.resource::<ModuleMap>();
            let material_system_module = module_map.get_module("materialsystem_client.so").unwrap();
            let cvar = material_system_module
                .create_interface("VEngineCvar007")
                .unwrap();

            let vphysics_module = module_map.get_module("vphysics_client.so").unwrap();
            let ptr = vphysics_module
                .create_interface("VPhysicsSurfaceProps001")
                .unwrap();

            let cvar = ICvar { ptr: cvar };

            let ffa = convar::Ffa(cvar.find_var("mp_teammates_are_enemies").unwrap());
            let panorama_disable_blur =
                convar::PanoramaDisableBlur(cvar.find_var("@panorama_disable_blur").unwrap());
            let sv_cheats = convar::SvCheats(cvar.find_var("sv_cheats").unwrap());
            let recoil_scale = convar::RecoilScale(cvar.find_var("weapon_recoil_scale").unwrap());

            app.insert_resource(cvar);

            app.insert_resource(ffa);
            app.insert_resource(panorama_disable_blur);
            app.insert_resource(recoil_scale);
            app.insert_resource(sv_cheats);

            let material_system = app.world.resource::<IMaterialSystem>();
            let keyvalues = KeyValues::from_str(
                "UnlitGeneric",
                r#"
                    $additive 1
                    $envmap models/effects/cube_white
                    $envmapfresnel 1
                    $alpha 0.8
                "#,
            )
            .unwrap();

            let material = material_system.create("elysium/glow", &keyvalues).unwrap();

            app.insert_resource(material::Glow(material));

            let engine = app.world.resource::<IVEngineClient>();
            let bsp_tree_query = engine.bsp_tree_query().unwrap();

            bsp_tree_query.setup();

            app.insert_resource(IPhysicsSurfaceProps { ptr });
        }

        let mut system_state: SystemState<(
            Res<Config>,
            Res<IBaseClientDLL>,
            Res<IVEngineClient>,
            Res<CInput>,
        )> = SystemState::new(&mut app.world);

        let (config, client, engine, input) = system_state.get(&app.world);
        let mut in_thirdperson = config.thirdperson_enabled & config.in_thirdperson;
        let view_angle = engine.view_angle();

        match frame {
            FRAME_NET_UPDATE_END => {
                let sv_cheats = app.world.resource::<convar::SvCheats>();

                sv_cheats.write(true);
            }
            FRAME_RENDER_START => {
                let panorama_disable_blur = app.world.resource::<convar::PanoramaDisableBlur>();

                panorama_disable_blur.write(true);

                // for the eventual UI replacement
                //
                // tracing::trace!("{:?}", engine.level_name());
                //
                // if let Some(channel) = engine.net_channel() {
                //     let info = channel.info();
                //
                //     tracing::trace!("{info:?}");
                // }

                if let Some(local_player) = IClientEntity::local_player() {
                    in_thirdperson &= !(local_player.observer_mode().breaks_thirdperson()
                        | local_player.is_scoped());

                    input.set_in_thirdperson(in_thirdperson);
                    app.insert_resource(OriginalViewAngle(local_player.view_angle()));

                    if in_thirdperson {
                        if let Some(last_command) = app.world.get_resource::<CUserCmd>() {
                            local_player.set_view_angle(last_command.view_angle);
                        }
                    } else {
                        let aim_punch = local_player.aim_punch();

                        local_player
                            .set_view_angle(view_angle - Vec3::new(0.0, 0.0, 15.0) - aim_punch);
                    }
                } else {
                    input.set_in_thirdperson(false);
                }

                app.update();
            }
            FRAME_RENDER_END => {
                if let Some(original_view_angle) = app.world.get_resource::<OriginalViewAngle>() {
                    if let Some(local_player) = IClientEntity::local_player() {
                        local_player.set_view_angle(original_view_angle.0);
                    }
                }
            }
            _ => {}
        }

        app.world.resource::<FrameStageNotify>().0
    });

    (method)(this, frame)
}
