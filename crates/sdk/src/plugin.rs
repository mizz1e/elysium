use crate::entity::AnimState;
use crate::{
    config, engine, global, ptr, ui, Args, Config, EngineVGui, Error, IBaseClientDLL,
    IClientEntityList, IEngineTrace, IMaterialSystem, IPhysicsSurfaceProps, IVEngineClient,
    IVModelRender, KeyValues, ModuleMap, OnceLoaded, SourceSettings, Surface, Ui, WindowMode,
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

unsafe fn fix_vguimatsurface() {
    let lib = libloading::Library::new("vguimatsurface_client.so").unwrap();

    // thunked functions
    let fc_config_app_font_add_dir = *lib.get(b"FcConfigAppFontAddDir\0").unwrap();
    let fc_config_get_current = *lib.get(b"FcConfigGetCurrent\0").unwrap();
    let fc_config_substitute = *lib.get(b"FcConfigSubstitute\0").unwrap();
    let fc_default_substitute = *lib.get(b"FcDefaultSubstitute\0").unwrap();
    let fc_font_list = *lib.get(b"FcFontList\0").unwrap();
    let fc_font_match = *lib.get(b"FcFontMatch\0").unwrap();
    let fc_font_set_destroy = *lib.get(b"FcFontSetDestroy\0").unwrap();
    let fc_init = *lib.get(b"FcInit\0").unwrap();
    let fc_object_set_add = *lib.get(b"FcObjectSetAdd\0").unwrap();
    let fc_object_set_create = *lib.get(b"FcObjectSetCreate\0").unwrap();
    let fc_object_set_destroy = *lib.get(b"FcObjectSetDestroy\0").unwrap();
    let fc_pattern_add = *lib.get(b"FcPatternAdd\0").unwrap();
    let fc_pattern_create = *lib.get(b"FcPatternCreate\0").unwrap();
    let fc_pattern_destroy = *lib.get(b"FcPatternDestroy\0").unwrap();
    let fc_pattern_get_bool = *lib.get(b"FcPatternGetBool\0").unwrap();
    let fc_pattern_get_string = *lib.get(b"FcPatternGetString\0").unwrap();

    write_hook(fc_config_app_font_add_dir, fc_noop);
    write_hook(fc_config_get_current, fc_noop);
    write_hook(fc_config_substitute, fc_noop);
    write_hook(fc_default_substitute, fc_noop);
    write_hook(fc_font_list, fc_noop);
    write_hook(fc_font_match, fc_noop);
    write_hook(fc_font_set_destroy, fc_noop);
    write_hook(fc_init, fc_noop);
    write_hook(fc_object_set_add, fc_noop);
    write_hook(fc_object_set_create, fc_noop);
    write_hook(fc_object_set_destroy, fc_noop);
    write_hook(fc_pattern_add, fc_noop);
    write_hook(fc_pattern_create, fc_noop);
    write_hook(fc_pattern_destroy, fc_noop);
    write_hook(fc_pattern_get_bool, fc_noop);
    write_hook(fc_pattern_get_string, fc_noop);
}

unsafe fn write_hook(dst: unsafe extern "C" fn(), src: unsafe extern "C" fn()) {
    let dst = dst as *mut u8;
    let src = src as *const u8;

    tracing::trace!("before");

    dismal::assembly::disassemble(&*dst.cast::<[u8; 32]>());

    let [a, b, c, d, e, f, g, h] = src.addr().to_ne_bytes();

    let code = [
        0x48, 0xB8, a, b, c, d, e, f, g, h, // mov rax, addr
        0xFF, 0xE0, // jmp rax
    ];

    ptr::replace_protected(dst.cast(), code);

    tracing::trace!("after");

    dismal::assembly::disassemble(&*dst.cast::<[u8; 12]>());
}

#[inline(never)]
unsafe extern "C" fn fc_noop() {
    tracing::trace!("fontconfig noop'd");
}

unsafe fn source_setup() -> Result<(), Error> {
    let launcher_main = global::with_app_mut::<Result<_, Error>>(|app| {
        app.insert_resource(Ui::new()?);

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

                let _tier0_module = module_map.open("libtier0_client.so")?;
                let _studio_render_module = module_map.open("studiorender_client.so")?;

                let material_system_module = module_map.open("materialsystem_client.so")?;
                let ptr = material_system_module.create_interface("VMaterialSystem080")?;

                world.insert_resource(IMaterialSystem { ptr });

                let _module = module_map.open("vphysics_client.so").unwrap();
                let _module = module_map.open("vgui2_client.so").unwrap();
                let _module = module_map.open("vguimatsurface_client.so").unwrap();
                let _module = module_map.open("inputsystem_client.so").unwrap();

                let launcher_module = module_map.open("launcher_client.so")?;
                let launcher_main = launcher_module.symbol("LauncherMain\0")?;

                Ok(launcher_main)
            })
    })?;

    global::with_resource::<SourceSettings, _>(move |settings| {
        Args::from(settings).exec(launcher_main);
    });

    Ok(())
}

fn source_runner(app: App) {
    global::set_app(app);

    unsafe {
        if let Err(error) = source_setup() {
            tracing::error!("{error:?}");
            std::process::exit(1);
        }
    }
}
