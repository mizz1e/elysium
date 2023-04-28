use {
    super::SdlContext,
    bevy::{ecs::schedule::ScheduleLabel, prelude::*},
    bevy_source_internal::{app_mut, assert_mnemonic, assert_non_null, inline_mov_jmp, FnPtr, Ptr},
    sdl2::{event, sys},
    std::ffi,
};

pub type Fn = unsafe extern "C" fn(*mut sys::SDL_Event) -> ffi::c_int;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ScheduleLabel)]
pub struct FnSchedule;

#[derive(Resource)]
pub struct FnArgs(event::Event);

pub unsafe extern "C" fn hook(raw_event: *mut sys::SDL_Event) -> ffi::c_int {
    assert_non_null!(raw_event);

    let app = app_mut!();
    let event = event::Event::from_ll(raw_event.read_unaligned());

    if !(event.is_drop()
        || event.is_render()
        || event.is_unknown()
        || event.is_user()
        || event.is_user_event())
    {
        app.insert_resource(FnArgs(event));
        app.world.run_schedule_ref(&FnSchedule);
    }

    let context = app.world.resource::<SdlContext>();

    (context.poll_event)(raw_event)
}

pub fn prologue(args: Res<FnArgs>) -> event::Event {
    args.0.clone()
}

pub fn add_schedule<M>(
    app: &mut App,
    system: impl IntoSystem<event::Event, (), M>,
) -> Result<Fn, &'static str> {
    let mut schedule = Schedule::default();

    schedule.add_system(prologue.pipe(system));
    app.add_schedule(FnSchedule, schedule);

    let method: Fn = sys::SDL_PollEvent;
    let instruction = method
        .last_instruction()
        .map_err(|_| "failed to disassemble SDL_PollEvent")?;

    assert_mnemonic!(instruction, Jmp, "incompatible libSDL.so detected.",);

    let original = unsafe {
        method
            .transmute::<*mut Fn>()
            .map_addr(|_addr| instruction.ip_rel_memory_address() as usize)
            .read_unaligned()
    };

    inline_mov_jmp!(method, unsafe extern "C" fn(raw_event: *mut sys::SDL_Event) -> ffi::c_int {
        hook(raw_event)
    });

    Ok(original)
}
