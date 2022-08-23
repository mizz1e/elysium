use core::fmt;

pub use id::EntityId;
pub use list::EntityList;
pub use networkable::{DataUpdateKind, Networkable};
pub use renderable::Renderable;

mod id;
mod list;
mod networkable;
mod renderable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum WeaponSound {
    Empty = 0,
    Single = 1,
    SingleNpc = 2,
    Double = 3,
    DoubleNpc = 4,
    Burst = 5,
    Reload = 6,
    ReloadNpc = 7,
    MeleeMiss = 8,
    MeleeHit = 9,
    MeleeHitWorld = 10,
    Special1 = 11,
    Special2 = 12,
    Special3 = 13,
    Taunt = 14,
    FastReload = 15,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum MoveKind {
    None = 0,
    Isometric = 1,
    Walk = 2,
    Step = 3,
    Fly = 4,
    FlyGravity = 5,
    VPhysics = 6,
    Push = 7,
    NoClip = 8,
    Ladder = 9,
    Observer = 10,
    Custom = 11,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Team {
    None = 0,
    Spectators = 1,
    Terrorist = 2,
    Counter = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ObserverMode {
    None = 0,
    Deathcam = 1,
    Freezecam = 2,
    Fixed = 3,
    InEye = 4,
    Chase = 5,
    Roaming = 6,
}

impl ObserverMode {
    /// if the observer mode breaks thirdperson
    #[inline]
    pub const fn breaks_thirdperson(&self) -> bool {
        matches!(
            self,
            ObserverMode::InEye | ObserverMode::Chase | ObserverMode::Roaming
        )
    }
}

const ON_GROUND: i32 = 1 << 0;
const DUCKING: i32 = 1 << 1;
const WATER_JUMP: i32 = 1 << 2;
// const ON_TRAIN: i32 = 1 << 3;
// const IN_RAIN: i32 = 1 << 4;
// const FROZEN: i32 = 1 << 5;
// const CONTROL_OTHER: i32 = 1 << 6;
// const IS_PLAYER: i32 = 1 << 7;
const IS_BOT: i32 = 1 << 8;
const IN_WATER: i32 = 1 << 9;
const MASK: i32 = ON_GROUND | DUCKING | WATER_JUMP | IS_BOT | IN_WATER;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct PlayerFlags(i32);

impl PlayerFlags {
    #[inline]
    pub const fn new(flags: i32) -> Self {
        // unknown flags would break PartialEq, so strip them
        Self(flags & MASK)
    }

    #[inline]
    const fn has(&self, flag: i32) -> bool {
        (self.0 & flag) != 0
    }

    #[inline]
    pub const fn on_ground(&self) -> bool {
        self.has(ON_GROUND)
    }

    #[inline]
    pub const fn ducking(&self) -> bool {
        self.has(DUCKING)
    }

    #[inline]
    pub const fn water_jump(&self) -> bool {
        self.has(WATER_JUMP)
    }

    #[inline]
    pub const fn is_bot(&self) -> bool {
        self.has(IS_BOT)
    }

    #[inline]
    pub const fn in_water(&self) -> bool {
        self.has(IN_WATER)
    }
}

impl fmt::Debug for PlayerFlags {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        enum Flag {
            OnGround,
            Ducking,
            WaterJump,
            IsBot,
            InWater,
        }

        let mut list = fmt.debug_list();

        if self.on_ground() {
            list.entry(&Flag::OnGround);
        }

        if self.ducking() {
            list.entry(&Flag::Ducking);
        }

        if self.water_jump() {
            list.entry(&Flag::WaterJump);
        }

        if self.is_bot() {
            list.entry(&Flag::IsBot);
        }

        if self.in_water() {
            list.entry(&Flag::InWater);
        }

        list.finish()
    }
}
