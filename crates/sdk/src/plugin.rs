use crate::entity::AnimState;
use crate::{
    config, engine, event::EventManager, global, ptr, Args, Config, EngineVGui, Error,
    IBaseClientDLL, IClientEntityList, IEngineTrace, IMaterialSystem, IPhysicsSurfaceProps,
    IVEngineClient, IVModelRender, KeyValues, ModuleMap, OnceLoaded, SourceSettings, Surface,
    WindowMode,
};
use bevy::prelude::App;

pub unsafe fn init(app: &mut App) {
    if !app.world.contains_resource::<Config>() {
        app.insert_resource(config::load());
    }

    let mut init = || unsafe {
        let mut module_map = ModuleMap::default();
        let world = &mut app.world;

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

        let ptr = engine_module.create_interface("VEngineVGui001")?;
        let engine_vgui = EngineVGui { ptr };

        engine_vgui.setup();
        world.insert_resource(engine_vgui);

        let ptr = engine_module.create_interface("GAMEEVENTSMANAGER002")?;
        let event_manager = EventManager { ptr };

        event_manager.setup();
        world.insert_resource(event_manager);

        let _tier0_module = module_map.open("libtier0_client.so")?;
        let _studio_render_module = module_map.open("studiorender_client.so")?;

        let material_system_module = module_map.open("materialsystem_client.so")?;
        let ptr = material_system_module.create_interface("VMaterialSystem080")?;

        world.insert_resource(IMaterialSystem { ptr });

        let _module = module_map.open("vphysics_client.so").unwrap();
        let _module = module_map.open("vgui2_client.so").unwrap();
        let _module = module_map.open("vguimatsurface_client.so").unwrap();
        let _module = module_map.open("inputsystem_client.so").unwrap();

        Ok::<_, Error>(module_map)
    };

    let module_map = (init)().unwrap();

    app.insert_resource(module_map);
}
