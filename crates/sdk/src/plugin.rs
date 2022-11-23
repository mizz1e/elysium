use crate::entity::AnimState;
use crate::{
    config, engine, global, sdl, Args, Config, Error, GlLoader, IBaseClientDLL, IClientEntityList,
    IEngineTrace, IMaterialSystem, IPhysicsSurfaceProps, IVEngineClient, IVModelRender, KeyValues,
    ModuleMap, OnceLoaded, SourceSettings, WindowMode,
};
use bevy::prelude::{App, Plugin};

/// Source engine bevy plugin.
pub struct SourcePlugin;

impl Plugin for SourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SourceSettings>()
            .init_resource::<ModuleMap>()
            .set_runner(source_runner);

        if !app.world.contains_resource::<Config>() {
            app.insert_resource(config::load());
        }
    }
}

unsafe fn source_setup() -> Result<(), Error> {
    let launcher_main = global::with_app_mut::<Result<_, Error>>(|app| {
        app.world
            .resource_scope::<ModuleMap, _>(|world, mut module_map| {
                let loader = GlLoader::setup();

                world.insert_resource(loader);

                let client_module = module_map.open("client_client.so")?;

                AnimState::setup();
                KeyValues::setup();

                let ptr = client_module.create_interface("VClientEntityList003")?;

                world.insert_resource(IClientEntityList { ptr });

                let ptr = client_module.create_interface("VClient018")?;
                let client = IBaseClientDLL { ptr };

                client.setup();
                world.insert_resource(client);

                let engine_module = module_map.open("engine_client.so")?;

                engine::setup();

                let ptr = engine_module.create_interface("VEngineClient014")?;

                world.insert_resource(IVEngineClient { ptr });

                let ptr = engine_module.create_interface("VEngineModel016")?;
                let model_render = IVModelRender { ptr };

                model_render.setup();
                world.insert_resource(model_render);

                let ptr = engine_module.create_interface("EngineTraceClient004")?;

                world.insert_resource(IEngineTrace { ptr });

                let _tier0_module = module_map.open("libtier0_client.so")?;
                let _studio_render_module = module_map.open("studiorender_client.so")?;

                let material_system_module = module_map.open("materialsystem_client.so")?;
                let ptr = material_system_module.create_interface("VMaterialSystem080")?;

                world.insert_resource(IMaterialSystem { ptr });

                let _vphysics_module = module_map.open("vphysics_client.so").unwrap();

                let launcher_module = module_map.open("launcher_client.so")?;
                let launcher_main = launcher_module.symbol("LauncherMain\0")?;

                Ok(launcher_main)
            })
    })?;

    global::with_app(|app| {
        let settings = app.world.resource::<SourceSettings>();
        let mut args = Args::default();

        args.push("csgo_linux64");

        if !settings.no_vac {
            args.push("-steam");
        }

        args.push(settings.renderer.arg());

        if let Some(ref max_fps) = settings.max_fps {
            args.push("+fps_max").push(max_fps.to_string());
        }

        match settings.once_loaded {
            OnceLoaded::ConnectTo(ref address) => {
                args.push("+connect").push(address.to_string());
            }
            OnceLoaded::LoadMap(ref map) => {
                args.push("+map").push(map);
            }
            _ => {}
        }

        match settings.window_mode {
            WindowMode::Windowed => {
                args.push("-windowed");
            }
            WindowMode::Fullscreen => {
                args.push("-fullscreen");
            }
            _ => {}
        }

        args.exec(launcher_main);
    });

    Ok(())
}

fn source_runner(app: App) {
    global::set_app(app);

    unsafe {
        let _ = source_setup();
    }
}
