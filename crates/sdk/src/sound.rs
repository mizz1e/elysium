use cake::ffi::BytePad;
use elysium_math::Vec3;

/// Active audio channels.
#[repr(C)]
pub struct ActiveChannels {
    pub count: i32,
    pub list: [u16; 128],
}

/// A audio channel.
#[repr(C)]
pub struct Channel {
    _pad0: BytePad<260>,
    pub sound_source: i32,
    _pad1: BytePad<56>,
    pub origin: Vec3,
    pub direction: Vec3,
    _pad2: BytePad<80>,
}
