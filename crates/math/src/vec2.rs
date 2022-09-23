use crate::{impl_all_ops, impl_methods};
use core::simd::{Simd, SimdFloat};

/// 2D vector.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub const fn from_array(array: [f32; 2]) -> Self {
        let [x, y] = array;

        Self { x, y }
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 2] {
        let Self { x, y } = self;

        [x, y]
    }

    #[inline]
    pub(crate) const fn from_simd(simd: Simd<f32, 2>) -> Self {
        Self::from_array(simd.to_array())
    }

    #[inline]
    pub(crate) const fn to_simd(self) -> Simd<f32, 2> {
        Simd::from_array(self.to_array())
    }

    /// Construct a vector where all elements are set to the given value.
    #[inline]
    pub const fn splat(value: f32) -> Self {
        Self::from_array([value; 2])
    }

    #[inline]
    pub fn product(self) -> f32 {
        self.to_simd().reduce_product()
    }

    impl_methods! {}
}

impl_all_ops! { Vec2 }
