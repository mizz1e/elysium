use crate::{assets, config::Pitch, global, Config, Error, Result, WalkingAnimation};
use bevy::{
    ecs::system::{ResMut, SystemState},
    prelude::Resource,
};

use iced_glow::{
    glow::{self, HasContext},
    Backend as GlowBackend, Renderer as GlowRenderer, Settings as GlowSettings, Viewport,
};

use iced_native::{
    clipboard::Null as Clipboard,
    color,
    keyboard::{Event::KeyPressed, KeyCode},
    mouse::{
        Button::Other,
        Event::{ButtonPressed, CursorMoved},
    },
    program::State as IcedState,
    renderer,
    theme::{Palette, Theme},
    Color, Command, Debug, Element as IcedElement, Event, Point, Program as IcedProgram, Size,
};

use sdl2::sys::{self, SDL_Event as RawSdlEvent, SDL_Window as RawSdlWindow};
use std::{
    ffi::{self, CString},
    fmt, ptr, slice,
};

pub mod conversion;
pub mod hud;
pub mod menu;

type GlxGetProcAddr = unsafe extern "C" fn(symbol: *const ffi::c_char) -> *const ffi::c_void;
type PollEvent = unsafe extern "C" fn(event: *mut RawSdlEvent) -> ffi::c_int;
type SwapWindow = unsafe extern "C" fn(window: *mut RawSdlWindow);

pub type Element<'a> = IcedElement<'a, Message, GlowRenderer>;

const DEBUG_OVERLAY: &[&str] = &[];

/// Replace the target address, if present.
pub unsafe fn replace_target<T: Copy>(dst: *mut u8, src: T) -> Result<T> {
    let code = slice::from_raw_parts_mut(dst, 128);
    let info = dismal::disassemble(code).ok_or(Error::NoJmp)?;

    tracing::trace!("target = {info:?}");

    let dst = dst.map_addr(|_addr| info.rel_addr).cast::<T>();

    tracing::trace!("target = {dst:?}");

    Ok(ptr::replace(dst, src))
}

#[derive(Clone, Debug)]
pub enum Message {
    None,

    AntiAim(bool),
    Pitch(Pitch),
    Roll(f32),
    YawOffset(f32),
    FakePitch(Pitch),
    FakeRoll(f32),
    FakeYawOffset(f32),
    FakeLag(i32),

    AutoShoot(bool),
    WalkingAnimation(WalkingAnimation),
    AntiAimTab,
    RageBotTab,
    VisualsTab,
    Thirdperson(bool),
    ChamColor(String),
    Load,
    Save,

    Command(String),
    RunCommand,
}

pub trait Layer: Send + Sync {
    fn update(&mut self, message: Message) -> Command<Message>;
    fn view(&self) -> Element<'_>;
}

pub struct LayerWrapper(Box<dyn Layer>);

impl IcedProgram for LayerWrapper {
    type Renderer = GlowRenderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.0.update(message)
    }

    fn view(&self) -> Element<'_> {
        self.0.view()
    }
}

pub struct Preload {
    pub glx: libloading::Library,
    pub glx_get_proc_addr: GlxGetProcAddr,
}

pub struct Hooked {
    pub glx: libloading::Library,
    pub glx_get_proc_addr: GlxGetProcAddr,
    pub poll_event: PollEvent,
    pub swap_window: SwapWindow,
}

pub struct Full {
    pub clipboard: Clipboard,
    pub cursor_position: Point,
    pub debug: Debug,
    pub glx: libloading::Library,
    pub gl_context: glow::Context,
    pub layers: Vec<IcedState<LayerWrapper>>,
    pub poll_event: PollEvent,
    pub renderer: GlowRenderer,
    pub swap_window: SwapWindow,
    pub viewport: Viewport,
}

/// UI-related resources.
#[derive(Resource)]
pub enum Ui {
    Preload(Preload),
    Hooked(Hooked),
    Full(Full),
}

unsafe impl Send for Ui {}
unsafe impl Sync for Ui {}

impl Ui {
    pub unsafe fn new() -> Result<Self> {
        let glx = libloading::Library::new("libGLX.so")?;

        tracing::trace!("glx = {glx:?}");

        let glx_get_proc_addr: GlxGetProcAddr = *glx.get(b"glXGetProcAddress\0")?;

        tracing::trace!("glx_get_proc_addr = {glx_get_proc_addr:?}");

        Ok(Self::Preload(Preload {
            glx,
            glx_get_proc_addr,
        }))
    }

    pub unsafe fn setup_hooks(&mut self) -> Result<()> {
        let Preload {
            glx,
            glx_get_proc_addr,
        } = match self {
            Self::Preload(preload) => ptr::read(preload),
            _ => return Ok(()),
        };

        let poll_event: PollEvent =
            replace_target(sys::SDL_PollEvent as *mut u8, poll_event as PollEvent)?;

        tracing::trace!("poll_event = {poll_event:?}");

        let swap_window: SwapWindow =
            replace_target(sys::SDL_GL_SwapWindow as *mut u8, swap_window as SwapWindow)?;

        tracing::trace!("swap_window = {swap_window:?}");

        let hooked = Hooked {
            glx,
            glx_get_proc_addr,
            poll_event,
            swap_window,
        };

        ptr::write(self, Self::Hooked(hooked));

        Ok(())
    }

    pub unsafe fn setup_render(&mut self, viewport: Viewport) -> Result<&mut Full> {
        let Hooked {
            glx,
            glx_get_proc_addr,
            poll_event,
            swap_window,
        } = match self {
            Self::Preload(_) => panic!("unexpected senario!"),
            Self::Hooked(hooked) => ptr::read(hooked),
            Self::Full(full) => return Ok(full),
        };

        let gl_context = glow::Context::from_loader_function(|symbol| {
            let symbol = CString::new(symbol).unwrap_or_else(|error| {
                panic!("glow requested an invalid symbol: {error}");
            });

            (glx_get_proc_addr)(symbol.as_ptr())
        });

        tracing::trace!("gl_context = {gl_context:?}");

        begin_render(&gl_context, &viewport);

        let settings = GlowSettings {
            default_font: Some(assets::QUICKSAND_REGULAR),
            ..GlowSettings::default()
        };

        let renderer = GlowRenderer::new(GlowBackend::new(&gl_context, settings));

        let mut full = Full {
            clipboard: Clipboard,
            debug: Debug::new(),
            cursor_position: Point::ORIGIN,
            glx,
            gl_context,
            layers: Vec::new(),
            poll_event,
            renderer,
            swap_window,
            viewport,
        };

        full.add_layer(hud::Hud);
        full.add_layer(menu::Menu);

        ptr::write(self, Self::Full(full));

        match self {
            Self::Full(full) => Ok(full),
            _ => unreachable!(),
        }
    }

    pub fn queue_event(&mut self, event: Event) {
        match self {
            Self::Full(full) => full.queue_event(event),
            _ => {}
        }
    }

    pub unsafe fn poll_event(&self, event: *mut RawSdlEvent) -> ffi::c_int {
        let poll_event = match self {
            Self::Hooked(Hooked { poll_event, .. }) => poll_event,
            Self::Full(Full { poll_event, .. }) => poll_event,
            _ => return 0,
        };

        (poll_event)(event)
    }
}

impl Full {
    pub fn add_layer(&mut self, layer: impl Layer + 'static) {
        self.layers.push(IcedState::new(
            LayerWrapper(Box::new(layer)),
            self.viewport.logical_size(),
            &mut self.renderer,
            &mut self.debug,
        ));
    }

    pub fn queue_event(&mut self, event: Event) {
        tracing::debug!("{event:?}");

        match &event {
            Event::Mouse(CursorMoved { position }) => self.cursor_position = *position,
            _ => {}
        }

        for layer in self.layers.iter_mut() {
            layer.queue_event(event.clone());
        }
    }

    pub fn render(&mut self) {
        let theme = Theme::custom(Palette {
            primary: color!(0xa00000),
            ..Palette::DARK
        });

        let style = renderer::Style {
            text_color: Color::WHITE,
        };

        for layer in self.layers.iter_mut() {
            layer.update(
                self.viewport.logical_size(),
                self.cursor_position,
                &mut self.renderer,
                &theme,
                &style,
                &mut self.clipboard,
                &mut self.debug,
            );

            self.renderer.with_primitives(|backend, primitives| {
                backend.present(&self.gl_context, primitives, &self.viewport, DEBUG_OVERLAY);
            });
        }
    }
}

fn begin_render(gl_context: &glow::Context, viewport: &Viewport) {
    let Size { width, height } = viewport.physical_size();

    unsafe {
        // enable auto-conversion from/to sRGB
        gl_context.enable(glow::FRAMEBUFFER_SRGB);

        // enable alpha blending to not break our fonts
        gl_context.enable(glow::BLEND);
        gl_context.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        // reset viewport dimensions
        gl_context.viewport(0, 0, width as i32, height as i32);
    }
}

/// [SDL_PollEvent](sys::SDL_PollEvent) hook.
pub unsafe extern "C" fn poll_event(event: *mut RawSdlEvent) -> ffi::c_int {
    global::with_app_mut(|app| {
        let mut system_state: SystemState<(ResMut<Config>, ResMut<Ui>)> =
            SystemState::new(&mut app.world);

        let (mut config, mut ui) = system_state.get_mut(&mut app.world);

        conversion::map_event(*event, |event| {
            match &event {
                // toggle menu
                Event::Keyboard(KeyPressed {
                    key_code: KeyCode::Insert,
                    ..
                }) => config.menu_open ^= true,

                // escape menu
                Event::Keyboard(KeyPressed {
                    key_code: KeyCode::Escape,
                    ..
                }) => config.menu_open = false,

                // toggle thirdperson
                Event::Mouse(ButtonPressed(Other(4))) => config.thirdperson_enabled ^= true,

                _ => {}
            };

            ui.queue_event(event);
        });

        ui.poll_event(event)
    })
}

/// Obtain the window's viewport.
///
/// See source of [Window::size](sdl2::video::Window::size).
pub unsafe fn window_viewport(window: *mut RawSdlWindow) -> Viewport {
    let mut width = 0;
    let mut height = 0;

    sys::SDL_GetWindowSize(window, &mut width, &mut height);

    Viewport::with_physical_size(Size::new(width as u32, height as u32), 1.0)
}

/// [SDL_GL_SwapWindow](sys::SDL_GL_SwapWindow) hook.
pub unsafe extern "C" fn swap_window(window: *mut RawSdlWindow) {
    global::with_app_mut(|app| {
        let mut ui = app.world.resource_mut::<Ui>();
        let viewport = window_viewport(window);

        let ui = ui.setup_render(viewport.clone()).unwrap_or_else(|error| {
            panic!("unable to create renderer: {error:?}");
        });

        begin_render(&ui.gl_context, &viewport);

        ui.viewport = viewport;
        ui.render();

        (ui.swap_window)(window)
    })
}
