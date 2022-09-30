//! Shared global state variables.

use crate::anti_aim::AntiAim;
use crate::entity::{Player as _, PlayerRef};
use crate::ui;
use crate::Networked;
use elysium_math::Vec3;
use elysium_sdk::material::Material;
use elysium_sdk::network::Flow;
use elysium_sdk::{Globals, Input, Interfaces, Vars};
use iced_glow::glow;
use iced_native::{Point, Size};
use palette::Srgba;
use std::cell::SyncUnsafeCell;
use std::ptr;
use std::time::Instant;

pub use cache::{Player, Players};
pub use hooks::*;
pub use local::Local;

mod cache;
mod hooks;
mod local;

// static references to hooked materials
pub mod material {
    use elysium_sdk::material::Material;
    use elysium_sdk::AtomicMut;

    // blood
    pub static BLOOD: AtomicMut<Material> = AtomicMut::new();

    // world & bullet decals
    pub static DECAL: AtomicMut<Material> = AtomicMut::new();

    // fire
    pub static FIRE: AtomicMut<Material> = AtomicMut::new();

    // smoke
    pub static SMOKE: AtomicMut<Material> = AtomicMut::new();

    // muzzle flash
    pub static MUZZLE_FLASH: AtomicMut<Material> = AtomicMut::new();

    // grenade paths
    pub static PATH: AtomicMut<Material> = AtomicMut::new();

    // impacts
    pub static IMPACT: AtomicMut<Material> = AtomicMut::new();

    // particle
    pub static PARTICLE: AtomicMut<Material> = AtomicMut::new();

    // props
    pub static PROP: AtomicMut<Material> = AtomicMut::new();

    // trees
    pub static TREE: AtomicMut<Material> = AtomicMut::new();

    // regular cham materials
    pub static FLAT: AtomicMut<Material> = AtomicMut::new();
    pub static GLOW: AtomicMut<Material> = AtomicMut::new();
}

#[repr(transparent)]
struct Wrap(State);

unsafe impl Sync for Wrap {}

static SHARED: SyncUnsafeCell<Wrap> = SyncUnsafeCell::new(Wrap(NEW));

const fn const_srgba(r: f32, g: f32, b: f32, a: f32) -> Srgba<f32> {
    use palette::{rgb, Alpha};
    use std::marker::PhantomData;

    Alpha {
        color: rgb::Rgb {
            red: r,
            green: g,
            blue: b,
            standard: PhantomData,
        },
        alpha: a,
    }
}

const NEW: State = State {
    context: None,
    proc_address: None,
    menu: None,
    menu_open: (false, false),
    cursor_position: Point::new(0.0, 0.0),
    window_size: Size::new(0, 0),
    hooks: Hooks::new(),
    networked: Networked::new(),
    vars: None,
    interfaces: None,
    globals: None,
    input: None,
    players: Players::new(),
    local: Local::new(),
    send_packet: ptr::null_mut(),
    view_angle: Vec3::splat(0.0),
    fog: const_srgba(0.7, 0.7, 0.7, 0.7),
    fog_start: 0.1,
    fog_end: 30_000.0,
    fog_clip: 0.0,
    bloom: 2.0,
    exposure_min: 0.2,
    exposure_max: 0.2,
    fake_lag: 1,
    anti_untrusted: true,
    anti_aim: AntiAim::new(),
    last_command: 0,
    ffa: false,
    update_materials: true,
    new_game: true,

    world: Vec::new(),
    smoke: Vec::new(),
    players_m: Vec::new(),
    particles: Vec::new(),

    init_time: None,

    create: ptr::null(),
    find: ptr::null(),
};

/// variables that need to be shared between hooks
pub struct State {
    /// opengl context
    pub context: Option<glow::Context>,
    /// opengl get proc address
    pub proc_address: Option<unsafe extern "C" fn(symbol: *const u8) -> *const u8>,
    /// menu context
    pub menu: Option<ui::Context>,
    /// first boolean determines whether the menu is visible, second prevents input from being
    /// spaz
    pub menu_open: (bool, bool),
    /// the cursor position
    pub cursor_position: Point,
    /// csgos window size
    pub window_size: Size<u32>,
    /// csgo, sdl, etc hooks
    pub hooks: Hooks,
    /// netvars
    pub networked: Networked,
    /// cvars
    pub vars: Option<Vars>,
    /// source engine interfaces
    pub interfaces: Option<Interfaces>,
    /// globals
    pub globals: Option<&'static mut Globals>,
    /// cinput
    pub input: Option<&'static mut Input>,
    /// efficient cache of players and their data (btw why is entitylist a linked list?)
    pub players: Players,
    /// local player variables
    pub local: Local,
    /// cl_move send_packet
    pub send_packet: *mut bool,
    /// engine view_angle
    pub view_angle: Vec3,
    /// fog colour
    pub fog: Srgba,
    pub fog_start: f32,
    pub fog_end: f32,
    pub fog_clip: f32,
    pub bloom: f32,
    pub exposure_min: f32,
    pub exposure_max: f32,
    pub fake_lag: u8,
    pub anti_untrusted: bool,
    pub anti_aim: AntiAim,
    pub last_command: i32,
    pub ffa: bool,
    pub update_materials: bool,
    pub new_game: bool,

    pub world: Vec<&'static mut Material>,
    pub smoke: Vec<&'static mut Material>,
    pub players_m: Vec<&'static mut Material>,
    pub particles: Vec<&'static mut Material>,

    pub init_time: Option<Instant>,

    pub create: *const (),
    pub find: *const (),
}

impl State {
    #[inline]
    pub fn get() -> &'static mut State {
        // SAFETY: Wrap is repr(transparent)
        unsafe { &mut *SyncUnsafeCell::raw_get(&SHARED).cast() }
    }

    /// toggle menu
    #[inline]
    pub fn toggle_menu(&mut self) {
        if !self.menu_open.1 {
            self.menu_open.0 ^= true;
            self.menu_open.1 = true;
        }
    }

    /// release menu toggle lock
    #[inline]
    pub fn release_menu_toggle(&mut self) {
        self.menu_open.1 = false;
    }
}

/// Determine whether a player's position (in time) can be backtracked.
pub fn is_record_valid(simulation_time: f32) -> Option<bool> {
    let state = State::get();
    let Interfaces { engine, .. } = state.interfaces.as_ref().unwrap();
    let globals = state.globals.as_mut().unwrap();
    let vars = state.vars.as_ref().unwrap();
    let local_vars = &mut state.local;
    let channel = engine.network_channel()?;
    let local = unsafe { PlayerRef::from_raw(local_vars.player).unwrap() };
    let unlag_max = vars.unlag_max.read();

    // https://www.unknowncheats.me/forum/counterstrike-global-offensive/359885-fldeadtime-int.html
    if simulation_time < globals.current_time - unlag_max {
        return Some(false);
    }

    let interp = vars.interp.read();
    let interp_ratio = vars.interp_ratio.read();
    let interp_ratio_min = vars.interp_ratio_min.read();
    let interp_ratio_max = vars.interp_ratio_max.read();
    let interp_ratio = interp_ratio.clamp(interp_ratio_min, interp_ratio_max);

    let update_rate = vars.update_rate.read();
    let update_rate_max = vars.update_rate_max.read();
    let update_rate = update_rate.max(update_rate_max);

    let lerp = interp.max(interp_ratio / update_rate);

    let delta =
        dbg!(channel.latency(Flow::Incoming)) + dbg!(channel.latency(Flow::Outgoing)) + lerp;

    let delta = delta.clamp(0.0, unlag_max)
        - globals.ticks_to_time(dbg!(local.tick_base()) as i32)
        - simulation_time;

    Some(delta.abs() <= unlag_max)
}
