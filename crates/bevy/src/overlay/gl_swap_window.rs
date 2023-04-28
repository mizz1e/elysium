use {
    super::OverlayContext,
    bevy::{ecs::schedule::ScheduleLabel, prelude::*},
    bevy_source_internal::{app_mut, assert_mnemonic, assert_non_null, inline_mov_jmp, FnPtr, Ptr},
    sdl2::sys,
    std::ffi::CStr,
};

pub type Fn = unsafe extern "C" fn(*mut sys::SDL_Window);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ScheduleLabel)]
pub struct FnSchedule;

pub unsafe extern "C" fn hook(raw_window: *mut sys::SDL_Window) {
    assert_non_null!(raw_window);

    let app = app_mut!();
    let mut context = app.world.resource_mut::<OverlayContext>();
    let gl_swap_window = context.gl_swap_window;

    if !context.is_initialized {
        let title = sys::SDL_GetWindowTitle(raw_window);

        assert_non_null!(title);

        let title = CStr::from_ptr(title);

        if !title.is_empty() {
            info!("Using SDL window: {title:?}");
            context.is_initialized = true;
        }
    }

    if context.is_initialized {
        app.world.run_schedule_ref(&FnSchedule);
    }

    (gl_swap_window)(raw_window)
}

pub fn add_schedule<M>(
    app: &mut App,
    system: impl IntoSystem<(), (), M>,
) -> Result<Fn, &'static str> {
    let mut schedule = Schedule::default();

    schedule.add_system(system);
    app.add_schedule(FnSchedule, schedule);

    let method: Fn = sys::SDL_GL_SwapWindow;
    let instruction = method
        .last_instruction()
        .map_err(|_| "failed to disassemble SDL_GL_SwapWindow")?;

    assert_mnemonic!(instruction, Jmp, "incompatible libSDL.so detected.",);

    let original = unsafe {
        method
            .transmute::<*mut Fn>()
            .map_addr(|_addr| instruction.ip_rel_memory_address() as usize)
            .read_unaligned()
    };

    inline_mov_jmp!(method, unsafe extern "C" fn(raw_window: *mut sys::SDL_Window) -> () {
        hook(raw_window)
    });

    Ok(original)
}
