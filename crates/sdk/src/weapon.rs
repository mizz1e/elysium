use std::mem::MaybeUninit;

/// kind of weapon
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum WeaponKind {
    Knife = 0,
    Pistol = 1,
    SMG = 2,
    Rifle = 3,
    Shotgun = 4,
    SniperRifle = 5,
    Machinegun = 6,
    C4 = 7,
    Placeholder = 8,
    Grenade = 9,
    Unknown = 10,
    StackableItem = 11,
    Fists = 12,
    BreachCharge = 13,
    BumpMine = 14,
    Tablet = 15,
    Melee = 16,
}

impl WeaponKind {
    pub const fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// information about a weapon
#[derive(Debug)]
#[repr(C)]
pub struct WeaponInfo {
    _pad0: MaybeUninit<[u8; 32]>,
    pub max_clip: i32,
    _pad1: MaybeUninit<[u8; 204]>,
    pub name: *const u8,
    _pad2: MaybeUninit<[u8; 72]>,
    pub kind: WeaponKind,
    _pad3: MaybeUninit<[u8; 4]>,
    pub price: i32,
    _pad4: MaybeUninit<[u8; 12]>,
    pub cycle_time: f32,
    _pad5: MaybeUninit<[u8; 12]>,
    pub full_auto: bool,
    _pad6: MaybeUninit<[u8; 3]>,
    pub damage: i32,
    pub headshot_multiplier: f32,
    pub armor_ratio: f32,
    pub bullets: i32,
    pub penetration: f32,
    _pad7: MaybeUninit<[u8; 8]>,
    pub range: f32,
    pub range_modifier: f32,
    _pad8: MaybeUninit<[u8; 16]>,
    pub silencer: bool,
    _pad9: MaybeUninit<[u8; 23]>,
    pub max_speed: f32,
    pub max_speed_alt: f32,
    _pad10: MaybeUninit<[u8; 100]>,
    pub recoil_magnitude: f32,
    pub recoil_magnitude_alt: f32,
    _pad11: MaybeUninit<[u8; 16]>,
    pub recovery_time_stand: f32,
}
