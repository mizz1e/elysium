use crate::entity::AnimState;
use crate::{
    global, Args, Error, IBaseClientDLL, IClientEntityList, IVEngineClient, KeyValues, ModuleMap,
    OnceLoaded, SourceSettings,
};
use bevy::prelude::*;

/// Source engine bevy plugin.
pub struct SourcePlugin;

impl Plugin for SourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SourceSettings>()
            .init_resource::<ModuleMap>()
            .set_runner(source_runner);
    }
}

unsafe fn source_setup() -> Result<(), Error> {
    let launcher_main = global::with_app_mut::<Result<_, Error>>(|app| {
        app.world
            .resource_scope::<ModuleMap, _>(|world, mut module_map| {
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
                let ptr = engine_module.create_interface("VEngineClient014")?;

                world.insert_resource(IVEngineClient { ptr });

                let _tier0_module = module_map.open("libtier0_client.so")?;
                let _studio_render_module = module_map.open("studiorender_client.so")?;

                let material_system_module = module_map.open("materialsystem_client.so")?;
                let material_system =
                    material_system_module.create_interface("VMaterialSystem080")?;

                let launcher_module = module_map.open("launcher_client.so")?;
                let launcher_main = launcher_module.symbol("LauncherMain\0")?;

                Ok(launcher_main)
            })
    })?;

    global::with_app(|app| {
        let settings = app.world.resource::<SourceSettings>();
        let mut args = Args::default();

        args.push("csgo_linux64")
            .push("-steam")
            .push(settings.renderer.arg());

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
