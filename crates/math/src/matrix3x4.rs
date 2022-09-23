use crate::Vec3;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::simd::Which::{First, Second};
use core::simd::{simd_swizzle, Simd};

/// 3x4 matrix.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x4 {
    matrix: [[f32; 4]; 3],
}

impl Matrix3x4 {
    /// Construct a matrix from a flat array of floats.
    #[inline]
    pub const fn from_array(array: [f32; 12]) -> Self {
        unsafe { mem::transmute(array) }
    }

    /// Convert this matrix to a flat array of floats.
    #[inline]
    pub const fn to_array(self) -> [f32; 12] {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub(crate) fn from_simd(simd: Simd<f32, 16>) -> Self {
        let array = simd.to_array();
        let array = unsafe { mem::transmute_copy(&array) };

        Self::from_array(array)
    }

    #[inline]
    pub(crate) fn to_simd(self) -> Simd<f32, 16> {
        let mut simd = Simd::splat(0.0);

        simd.as_mut_array()[..12].copy_from_slice(&self.to_array());
        simd
    }

    /// Construct a matrix where all elements are set to the given value.
    #[inline]
    pub const fn splat(value: f32) -> Self {
        let matrix = [[value; 4]; 3];

        Self { matrix }
    }

    /// Construct a model matrix where all elements are set to the given value.
    #[inline]
    pub const fn splat_model(value: f32) -> [Self; 256] {
        [Self::splat(value); 256]
    }

    /// Create a matrix where
    ///  x axis = forward
    ///  y axis = left
    ///  z axis = up
    ///  w axis = origin
    #[inline]
    pub fn from_xyzw(x: Vec3, y: Vec3, z: Vec3, w: Vec3) -> Self {
        Self::splat(0.0)
            .with_x_axis(x)
            .with_y_axis(y)
            .with_z_axis(z)
            .with_w_axis(w)
    }

    /// Returns the x axis (forward).
    #[inline]
    pub fn x_axis(self) -> Vec3 {
        let matrix = self.to_simd();

        Vec3::from_simd(simd_swizzle!(matrix, [0, 4, 9, 0]))
    }

    /// Returns the y axis (left).
    #[inline]
    pub fn y_axis(self) -> Vec3 {
        let matrix = self.to_simd();

        Vec3::from_simd(simd_swizzle!(matrix, [1, 5, 10, 0]))
    }

    /// Returns the z axis (up).
    #[inline]
    pub fn z_axis(self) -> Vec3 {
        let matrix = self.to_simd();

        Vec3::from_simd(simd_swizzle!(matrix, [2, 6, 11, 0]))
    }

    /// Returns the w axis (origin).
    #[inline]
    pub fn w_axis(self) -> Vec3 {
        let matrix = self.to_simd();

        Vec3::from_simd(simd_swizzle!(matrix, [3, 7, 12, 0]))
    }

    /// Set the x axis (forward).
    #[inline]
    pub fn with_x_axis(self, axis: Vec3) -> Self {
        let axis = axis.to_simd_16();
        let matrix = self.to_simd();
        let matrix = simd_swizzle!(
            matrix,
            axis,
            [
                Second(0),
                First(1),
                First(2),
                First(3),
                Second(1),
                First(5),
                First(6),
                First(7),
                Second(2),
                First(9),
                First(10),
                First(11),
                First(0),
                First(0),
                First(0),
                First(0),
            ]
        );

        Self::from_simd(matrix)
    }

    /// Set the y axis (left).
    #[inline]
    pub fn with_y_axis(self, axis: Vec3) -> Self {
        let axis = axis.to_simd_16();
        let matrix = self.to_simd();
        let matrix = simd_swizzle!(
            matrix,
            axis,
            [
                First(0),
                Second(0),
                First(2),
                First(3),
                First(4),
                Second(1),
                First(6),
                First(7),
                First(8),
                Second(2),
                First(10),
                First(11),
                First(0),
                First(0),
                First(0),
                First(0),
            ]
        );

        Self::from_simd(matrix)
    }
    /// Set the z axis (up).
    #[inline]
    pub fn with_z_axis(self, axis: Vec3) -> Self {
        let axis = axis.to_simd_16();
        let matrix = self.to_simd();
        let matrix = simd_swizzle!(
            matrix,
            axis,
            [
                First(0),
                Second(0),
                First(2),
                First(3),
                First(4),
                Second(1),
                First(6),
                First(7),
                First(8),
                Second(2),
                First(10),
                First(11),
                First(0),
                First(0),
                First(0),
                First(0),
            ]
        );

        Self::from_simd(matrix)
    }

    /// Set the w axis (orign).
    #[inline]
    pub fn with_w_axis(self, axis: Vec3) -> Self {
        let axis = axis.to_simd_16();
        let matrix = self.to_simd();
        let matrix = simd_swizzle!(
            matrix,
            axis,
            [
                First(0),
                Second(0),
                First(2),
                First(3),
                First(4),
                Second(1),
                First(6),
                First(7),
                First(8),
                Second(2),
                First(10),
                First(11),
                First(0),
                First(0),
                First(0),
                First(0),
            ]
        );

        Self::from_simd(matrix)
    }
}

impl Deref for Matrix3x4 {
    type Target = [[f32; 4]; 3];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl DerefMut for Matrix3x4 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}
