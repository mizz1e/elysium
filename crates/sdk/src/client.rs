use crate::{
    convar, global, material, networked, ptr, CGlobalVarsBase, CInput, CUserCmd, ClientClass,
    Config, IClientEntity, IClientMode, ICvar, IMaterialSystem, IPhysicsSurfaceProps,
    IVEngineClient, InputStackSystem, KeyValues, ModuleMap, Ptr, Surface, Ui,
};
use bevy::{ecs::system::SystemState, prelude::*};
use dismal::{assert_mnemonic, iced_x86, FnPtr, Ptr as _};
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
        info!("setup client");

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

            info!("obtain cinput");

            let activate_mouse = self.ptr.vtable_entry::<ptr::FnPtr>(16);
            let instruction = activate_mouse
                .disassemble()
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            assert_mnemonic!(
                instruction,
                Lea,
                "client_client.so has updated, unable to obtain cinput."
            );

            let ptr = activate_mouse
                .transmute::<*mut *mut u8>()
                .map_addr(|_addr| instruction.ip_rel_memory_address() as usize)
                .read_unaligned();

            let ptr = Ptr::new("CInput", ptr).unwrap_or_else(|| panic!("unable to find CInput"));
            let cinput = CInput { ptr };

            cinput.setup();
            app.insert_resource(cinput);

            networked::setup(self.all_classes());
        });
    }

    pub(crate) fn all_classes(&self) -> *const ClientClass {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ClientClass =
            unsafe { self.ptr.vtable_entry(8) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    unsafe fn setup_client_mode(&self) -> IClientMode {
        info!("obtain iclientmode");

        let hud_process_input = self.ptr.vtable_entry::<ptr::FnPtr>(10);
        let instruction = hud_process_input.last_instruction().unwrap();

        assert_mnemonic!(
            instruction,
            Call,
            "client_client.so has updated, unable to obtain iclientmode."
        );

        let get_client_mode = hud_process_input
            .transmute::<*const u8>()
            .map_addr(|_addr| instruction.memory_displacement64() as usize);

        info!("invoking get_client_mode @ {get_client_mode:?}");

        let client_mode = get_client_mode
            .transmute::<unsafe extern "C" fn() -> *mut u8>()
            .call(());

        let ptr = Ptr::new("IClientMode", client_mode)
            .unwrap_or_else(|| panic!("unable to find IClientMode"));

        let client_mode = IClientMode { ptr };

        client_mode.setup();
        client_mode
    }

    unsafe fn setup_global_vars(&self) -> CGlobalVarsBase {
        info!("obtain cglobalvarsbase");

        let hud_update = self.ptr.vtable_entry::<ptr::FnPtr>(11);
        let instruction = hud_update
            .disassemble()
            .unwrap()
            .into_iter()
            .nth(6)
            .unwrap();

        assert_mnemonic!(
            instruction,
            Mov,
            "client_client.so has updated, unable to obtain cglobalvarsbase."
        );

        let global_vars = hud_update
            .transmute::<*mut *mut u8>()
            .map_addr(|_addr| instruction.memory_displacement64() as usize)
            .read_unaligned();

        let ptr = Ptr::new("CGlobalVarsBase", global_vars)
            .unwrap_or_else(|| panic!("unable to find CGlobalVarsBase"));

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
            trace!("client mode doesnt exist, creating");

            let client = app.world.resource::<IBaseClientDLL>();
            let client_mode = client.setup_client_mode();
            let global_vars = client.setup_global_vars();

            app.insert_resource(client_mode);
            app.insert_resource(global_vars);

            let module_map = app.world.resource::<ModuleMap>();

            let engine_module = module_map.get_module("inputsystem_client.so").unwrap();
            let input_stack_system = engine_module
                .create_interface("InputStackSystemVersion001")
                .unwrap();

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
            let recoil_scale = convar::RecoilScale(cvar.find_var("weapon_recoil_scale").unwrap());

            let input_stack_system = InputStackSystem {
                ptr: input_stack_system,
            };

            input_stack_system.setup();
            app.insert_resource(input_stack_system);

            app.insert_resource(cvar);

            app.insert_resource(ffa);
            app.insert_resource(panorama_disable_blur);
            app.insert_resource(recoil_scale);

            let material_system = app.world.resource::<IMaterialSystem>();

            // $envmapfresnelminmaxexp [0 1] is broken
            let keyvalues = KeyValues::from_str(
                "VertexLitGeneric",
                "
                    $additive 1
                    $alpha 0.8
                    $envmap models/effects/cube_white
                    $envmapfresnel 1
                    $envmapanisotropy 1
                    $envmapanisotropyscale 5
                    $envtintmap [1 1 1]

                    
	  $envmapcontrast 1
	  $nofog 1
	  $model 1
	  $nocull 0
	  $selfillum 1
	  $halflambert 1
	  $znearer 0
	  $flat 1
                ",
            )
            .unwrap();

            let glow = material_system.create("elysium/glow", &keyvalues).unwrap();
            let keyvalues = KeyValues::from_str("UnlitGeneric", "").unwrap();
            let flat = material_system.create("elysium/flat", &keyvalues).unwrap();

            app.insert_resource(material::Glow(glow));
            app.insert_resource(material::Flat(flat));

            let engine = app.world.resource::<IVEngineClient>();
            let bsp_tree_query = engine.bsp_tree_query().unwrap();

            bsp_tree_query.setup();

            app.insert_resource(IPhysicsSurfaceProps { ptr });
        }

        if !app.world.contains_resource::<Surface>() {
            let module_map = app.world.resource::<ModuleMap>();
            let engine_module = module_map.get_module("vguimatsurface_client.so").unwrap();

            if let Ok(ptr) = engine_module.create_interface("VGUI_Surface031") {
                let surface = Surface { ptr };

                surface.setup();
                app.insert_resource(surface);
            } else {
                tracing::trace!("fuck you");
            }
        }

        let mut system_state: SystemState<(
            Res<Config>,
            Res<IBaseClientDLL>,
            Res<IVEngineClient>,
            Res<CInput>,
            Res<convar::PanoramaDisableBlur>,
            ResMut<Ui>,
        )> = SystemState::new(&mut app.world);

        let (config, client, engine, input, panorama_disable_blur, mut ui) =
            system_state.get_mut(&mut app.world);
        let view_angle = engine.view_angle();

        ui.setup_hooks().unwrap_or_else(|error| {
            panic!("unable to setup SDL hooks: {error:?}");
        });

        match frame {
            FRAME_RENDER_START => {
                panorama_disable_blur.write(true);

                if let Some(local_player) = IClientEntity::local_player() {
                    let in_thirdperson = input.in_thirdperson();

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
