use elysium_math::Matrix3x4;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};
use elysium_sdk::{Command, Vdf, View};
use sdl2_sys::{SDL_Event, SDL_Window};

pub type ClMove = unsafe extern "C" fn(extra_samples: f32, final_tick: bool);
pub type ClSendMove = unsafe extern "C" fn();
pub type CreateMove =
    unsafe extern "C" fn(this: *const u8, sample_time: f32, command: &mut Command) -> bool;

pub type DrawModel = unsafe extern "C" fn(
    this: *const ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
    unk1: usize,
);

pub type FrameStageNotify = unsafe extern "C" fn(this: *const u8, frame: i32);
pub type OverrideView = unsafe extern "C" fn(this: *const u8, view: &mut View);
pub type PollEvent = unsafe extern "C" fn(event: *mut SDL_Event) -> i32;
pub type SwapWindow = unsafe extern "C" fn(window: *mut SDL_Window);
pub type WriteUserCommand =
    unsafe extern "C" fn(buffer: *mut u8, from: *const u8, to: *const u8) -> bool;

pub type VdfInit = unsafe extern "C" fn(vdf: *mut Vdf, name: *const u8, unk1: i32, unk2: i32);
pub type VdfFromBytes =
    unsafe extern "C" fn(name: *const u8, value: *const u8, _unk1: *const u8) -> *const Vdf;

pub struct Hooks {
    pub cl_move: ClMove,
    pub cl_send_move: ClSendMove,
    pub create_move: CreateMove,
    pub draw_model: DrawModel,
    pub frame_stage_notify: FrameStageNotify,
    pub override_view: OverrideView,
    pub poll_event: PollEvent,
    pub swap_window: SwapWindow,
    pub write_user_command: WriteUserCommand,
    pub vdf_init: VdfInit,
    pub vdf_from_bytes: VdfFromBytes,
}
