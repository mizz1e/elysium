use elysium_math::Matrix3x4;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};
use elysium_sdk::ClientMode;
use elysium_sdk::{Command, Vdf, View};
use sdl2_sys::{SDL_Event, SDL_Window};

pub type ClMove = unsafe extern "C" fn(extra_samples: f32, final_tick: bool);
pub type ClSendMove = unsafe extern "C" fn();

pub type CreateMove = unsafe extern "C" fn(
    client_mode: &mut ClientMode,
    sample_time: f32,
    command: &mut Command,
) -> bool;

pub type DrawModel = unsafe extern "C" fn(
    model_render: &mut ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
);

pub type FrameStageNotify = unsafe extern "C" fn(this: *const u8, frame: i32);
pub type OverrideView = unsafe extern "C" fn(this: *const u8, view: &mut View);
pub type PollEvent = unsafe extern "C" fn(event: *mut SDL_Event) -> i32;
pub type SwapWindow = unsafe extern "C" fn(window: *mut SDL_Window);
pub type WriteUserCommand =
    unsafe extern "C" fn(buffer: *mut u8, from: *const u8, to: *const u8) -> bool;

pub type VdfFromBytes =
    unsafe extern "C" fn(name: *const u8, value: *const u8, _unk1: *const u8) -> *const Vdf;

// `Option<fn()>` is niche-optimized, btw, this will be the same size as a bunch of fn pointers
pub struct Hooks {
    pub cl_move: Option<ClMove>,
    pub cl_send_move: Option<ClSendMove>,
    pub create_move: Option<CreateMove>,
    pub draw_model: Option<DrawModel>,
    pub frame_stage_notify: Option<FrameStageNotify>,
    pub override_view: Option<OverrideView>,
    pub poll_event: Option<PollEvent>,
    pub swap_window: Option<SwapWindow>,
    pub write_user_command: Option<WriteUserCommand>,
    pub vdf_from_bytes: Option<VdfFromBytes>,
}

impl Hooks {
    const NEW: Self = Self {
        cl_move: None,
        cl_send_move: None,
        create_move: None,
        draw_model: None,
        frame_stage_notify: None,
        override_view: None,
        poll_event: None,
        swap_window: None,
        write_user_command: None,
        vdf_from_bytes: None,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::NEW
    }
}
