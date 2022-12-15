use crate::{global, Config, IVEngineClient, Ptr, Surface};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

#[derive(Resource)]
struct EnableInputContext(unsafe extern "C" fn(this: *mut u8, context: *mut u8, enable: bool));

/// `inputsystem/inputstacksystem.cpp`
#[derive(Resource)]
pub struct InputStackSystem {
    pub(crate) ptr: Ptr,
}

impl InputStackSystem {
    pub unsafe fn setup(&self) {
        tracing::trace!("setup InputStackSystem");

        global::with_app_mut(|app| {
            app.insert_resource(EnableInputContext(
                self.ptr.vtable_replace(11, enable_input_context),
            ));
        });
    }

    pub fn enable_input_context(&self, context: *mut u8, enable: bool) {
        let method = global::with_resource::<EnableInputContext, _>(|f| f.0);

        unsafe { (method)(self.ptr.as_ptr(), context, enable) }
    }
}

unsafe extern "C" fn enable_input_context(this: *mut u8, context: *mut u8, mut enabled: bool) {
    debug_assert!(!this.is_null());

    global::with_app_mut(|app| {
        let mut state: SystemState<(
            Res<Config>,
            Res<IVEngineClient>,
            Res<InputStackSystem>,
            Res<Surface>,
        )> = SystemState::new(&mut app.world);

        let (config, engine, input_stack_system, surface) = state.get(&app.world);

        let vgui_context = surface.input_context();

        // Permanently disable VGUI's input context.
        if context == vgui_context {
            input_stack_system.enable_input_context(context, false);

            return;
        }

        // Enable PanoramaUI's input context state if the menu is open.
        enabled |= engine.is_in_game() && config.menu_open;

        input_stack_system.enable_input_context(context, enabled);
    });
}
