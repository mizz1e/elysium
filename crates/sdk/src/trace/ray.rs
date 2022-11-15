use bevy::math::{Vec3, Vec4};
use cake::ffi::BytePad;

/// Ray to be traced.
#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Ray {
    pub start: Vec4,
    _pad0: BytePad<4>,
    pub delta: Vec4,
    _pad1: BytePad<44>,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        let delta = end - start;
        let is_ray = true;
        let is_swept = delta.length() != 0.0;
        let delta = delta.extend(0.0);
        let start = start.extend(0.0);

        Self {
            start,
            _pad0: BytePad::uninit(),
            delta,
            _pad1: BytePad::uninit(),
            is_ray,
            is_swept,
        }
    }
}
