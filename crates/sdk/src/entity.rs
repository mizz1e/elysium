use core::{fmt, mem};

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

impl MoveKind {
    /// Convert an integer representation to the enum.
    ///
    /// Invalid variants are substituted with [`MoveKind::None`].
    #[inline]
    pub const fn from_raw(kind: i32) -> Self {
        const START: i32 = MoveKind::None.to_i32();
        const END: i32 = MoveKind::Custom.to_i32();

        if matches!(kind, START..=END) {
            unsafe { mem::transmute(kind) }
        } else {
            // NOTE: Default to none as it seems move kind may possibly be uninitialized whilest
            // players are being created and initialized?
            //
            // During testing, variants such as `33554432` were encountered.
            //
            // Besides, nothing exposes a way of mutating a player's move kind (not that you should
            // do so) so this is perfectly safe!
            MoveKind::None
        }
    }

    /// Returns the integer representation of the enum value.
    #[inline]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }
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
//const ANIM_DUCKING: i32 = 1 << 2;
const WATER_JUMP: i32 = 1 << 3;
//const ON_TRAIN: i32 = 1 << 4;
//const IN_RAIN: i32 = 1 << 5;
//const FROZEN: i32 = 1 << 6;
//const AT_CONTROLS: i32 = 1 << 7;
//const CLIENT: i32 = 1 << 8;
const FAKE_CLIENT: i32 = 1 << 9;
const IN_WATER: i32 = 1 << 10;
//const FLY: i32 = 1 << 11;
//const SWIM: i32 = 1 << 12;
//const CONVEYOR: i32 = 1 << 13;
//const NPC: i32 = 1 << 14;
const GOD_MODE: i32 = 1 << 15;
//const NO_TARGET: i32 = 1 << 16;
//const AIM_TARGET: i32 = 1 << 17;
const PARTIAL_GROUND: i32 = 1 << 18;
//const STATIC_PROP: i32 = 1 << 19;
//const GRAPHED: i32 = 1 << 20;
//const GRENADE: i32 = 1 << 21;
//const STEP_MOVEMENT: i32 = 1 << 22;
//const DONT_TOUCH: i32 = 1 << 23;
//const BASE_VELOCITY: i32 = 1 << 24;
//const WORLD_BRUSH: i32 = 1 << 25;
//const OBJECT: i32 = 1 << 26;
//const KILL_ME: i32 = 1 << 27;
//const ON_FIRE: i32 = 1 << 28;
//const DISSOLVING: i32 = 1 << 29;
//const TRANS_RAGDOLL: i32 = 1 << 30;
//const UNBLOCKABLE_BY_PLAYER: i32 = 1 << 31;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct PlayerFlags(i32);

impl PlayerFlags {
    #[inline]
    pub const fn new(flags: i32) -> Self {
        Self(flags)
    }

    #[inline]
    const fn has(&self, flag: i32) -> bool {
        (self.0 & flag) != 0
    }

    #[inline]
    pub const fn ducking(&self) -> bool {
        self.has(DUCKING)
    }

    #[inline]
    pub const fn god_mode(&self) -> bool {
        self.has(GOD_MODE)
    }

    #[inline]
    pub const fn is_bot(&self) -> bool {
        self.has(FAKE_CLIENT)
    }

    #[inline]
    pub const fn in_water(&self) -> bool {
        self.has(IN_WATER)
    }

    #[inline]
    pub const fn on_ground(&self) -> bool {
        self.has(ON_GROUND)
    }

    #[inline]
    pub const fn partially_on_ground(&self) -> bool {
        self.has(PARTIAL_GROUND)
    }

    #[inline]
    pub const fn water_jump(&self) -> bool {
        self.has(WATER_JUMP)
    }
}

impl fmt::Debug for PlayerFlags {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        enum Flag {
            Ducking,
            GodMode,
            IsBot,
            InWater,
            OnGround,
            PartiallyOnGround,
            WaterJump,
        }

        let mut list = fmt.debug_list();

        if self.ducking() {
            list.entry(&Flag::Ducking);
        }

        if self.god_mode() {
            list.entry(&Flag::GodMode);
        }

        if self.is_bot() {
            list.entry(&Flag::IsBot);
        }

        if self.in_water() {
            list.entry(&Flag::InWater);
        }

        if self.on_ground() {
            list.entry(&Flag::OnGround);
        }

        if self.partially_on_ground() {
            list.entry(&Flag::PartiallyOnGround);
        }

        if self.water_jump() {
            list.entry(&Flag::WaterJump);
        }

        list.finish()
    }
}
