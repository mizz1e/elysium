use crate::{impl_all_ops, impl_methods, Vec2, Vec3};
use core::simd::{Simd, SimdFloat};

/// 4D vector.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    #[inline]
    pub const fn from_array(array: [f32; 4]) -> Self {
        let [x, y, z, w] = array;

        Self { x, y, z, w }
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 4] {
        let Self { x, y, z, w } = self;

        [x, y, z, w]
    }

    #[inline]
    pub(crate) const fn from_simd(simd: Simd<f32, 4>) -> Self {
        Self::from_array(simd.to_array())
    }

    #[inline]
    pub(crate) const fn to_simd(self) -> Simd<f32, 4> {
        Simd::from_array(self.to_array())
    }

    /// Construct a vector where all elements are set to the given value.
    #[inline]
    pub const fn splat(value: f32) -> Self {
        Self::from_array([value; 4])
    }

    #[inline]
    pub fn product(self) -> f32 {
        self.to_simd().reduce_product()
    }

    impl_methods! {}
}

impl From<Vec2> for Vec4 {
    #[inline]
    fn from(vec: Vec2) -> Vec4 {
        let [x, y] = vec.to_array();
        let z = 0.0;
        let w = 0.0;

        Vec4::from_array([x, y, z, w])
    }
}

impl From<Vec3> for Vec4 {
    #[inline]
    fn from(vec: Vec3) -> Vec4 {
        let [x, y, z] = vec.to_array();
        let w = 0.0;

        Vec4::from_array([x, y, z, w])
    }
}

impl_all_ops! { Vec4 }
