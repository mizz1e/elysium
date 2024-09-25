//#![deny(warnings)]
#![feature(c_variadic)]
#![feature(strict_provenance)]

use {
    crate::internal::{app_mut, inline_mov_jmp, inline_mov_jmp_variadic, set_app, FnPtr},
    bevy::prelude::*,
    leafwing_input_manager::prelude::*,
    std::ffi,
};

pub use {
    crate::traits::{AppExt, Interface, Module, WorldExt},
    bevy_source_internal as internal, source_sys as sys,
};

mod macros;
mod tier0;
mod traits;

pub mod overlay;
pub mod prelude;

pub mod interfaces {
    crate::macros::interfaces! {
        pub struct Client = "VClient005";
    }
}

pub mod modules {
    crate::macros::modules! {
        pub struct Client = "client";
        pub struct Engine = "engine";
        pub struct Input = "inputsystem";
        pub struct Launcher = "launcher";
        pub struct Material = "materialsystem";
        pub struct Tier0 = "libtier0";
        pub struct VGui = "vgui2";
        pub struct VPhysics = "vphysics";
    }
}

#[derive(Debug)]
pub struct SourcePlugin;

#[derive(Debug)]
pub enum GameEvent {
    PlayerHurt,
}

/// Player input.
#[derive(Actionlike, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Input {
    Look,
    Move,
}

impl Input {
    /// Returns the default input map.
    pub fn default_input_map() -> InputMap<Self> {
        let mut map = InputMap::default();

        map.insert(DualAxis::mouse_motion(), Self::Look);
        map.insert(VirtualDPad::wasd(), Self::Move);
        map
    }
}

impl Plugin for SourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>()
            .add_plugin(InputManagerPlugin::<Input>::default())
            .add_startup_system(move |mut commands: Commands| {
                commands.spawn(InputManagerBundle::<Input>::default());
            })
            .set_runner(move |mut app: App| {
                // Various Source engine modules have `.init_array` routines which are executed on
                // load, thus modules have to be loaded in a semi-specific order.

                // Launcher is always first.
                //
                // `csgo_linux64` would normally immediately execute `LauncherMain`, however some
                // hooks need to be made beforehand.
                app.init_module::<modules::Launcher>();

                // Tier0 is always second.
                //
                // Take control of `CommandLine`, and `LoggingSystem_Log` early.
                app.init_module::<modules::Tier0>();

                let tier0 = app.world.module::<modules::Tier0>();

                let command_line = tier0
                    .get::<sys::CommandLine>(c"CommandLine")
                    .expect("no command line")
                    .unwrap();

                let logging_system_log = tier0
                    .get::<sys::LoggingSystem_Log>(c"LoggingSystem_Log")
                    .expect("no logging system")
                    .unwrap();

                inline_mov_jmp!(command_line, unsafe extern "C" fn() -> *mut sys::ICommandLine {
                    tier0::command_line()
                });

                inline_mov_jmp_variadic!(logging_system_log, unsafe extern "C" fn(
                    _channelID: sys::LoggingChannelID_t,
                    severity: sys::LoggingSeverity_t,
                    pMessageFormat: *const ffi::c_char;
                    mut args: ...
                ) -> sys::LoggingResponse_t {
                    tier0::log(pMessageFormat, severity, args)
                });

                // Now it is safe to load the other modules.
                app.init_module::<modules::Client>();
                app.init_module::<modules::Engine>();
                app.init_module::<modules::Input>();
                app.init_module::<modules::Material>();
                app.init_module::<modules::VGui>();
                app.init_module::<modules::VPhysics>();

                // Finally, run Source engine, as usual.
                let launcher = app.world.module::<modules::Launcher>();
                let main = launcher
                    .get::<sys::LauncherMain_t>(c"LauncherMain")
                    .expect("no launcher main")
                    .unwrap();

                app.add_plugin(overlay::OverlayPlugin::default());

                // Set the global App. `main` is copy, launcher exists within App, so this is safe.
                set_app!(app);

                // TODO: Remove.
                unsafe {
                    elysium_sdk::init(app_mut!());
                }

                debug!("invoking LauncherMain");

                unsafe {
                    // In the nigh impossible event, that the above hooks did nothing, and didn't
                    // error, run `LauncherMain` with normal argumemts.
                    let args = [c"csgo_linux64".as_ptr()];

                    main.call((args.len() as ffi::c_int, args.as_ptr().cast_mut().cast()));
                }
            });
    }
}
