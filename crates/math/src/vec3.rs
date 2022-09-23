use crate::{impl_all_ops, impl_methods, Vec2};
use core::simd::{Simd, SimdFloat};

/// 3D vector.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub const fn from_array(array: [f32; 3]) -> Self {
        let [x, y, z] = array;

        Self { x, y, z }
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 3] {
        let Self { x, y, z } = self;

        [x, y, z]
    }

    #[inline]
    pub(crate) const fn from_simd(simd: Simd<f32, 4>) -> Self {
        let [x, y, z, w] = simd.to_array();
        let _ = w;

        Self::from_array([x, y, z])
    }

    #[inline]
    pub(crate) const fn to_simd(self) -> Simd<f32, 4> {
        self.to_simd_w(0.0)
    }

    #[inline]
    pub(crate) fn to_simd_16(self) -> Simd<f32, 16> {
        let mut simd = Simd::splat(0.0);

        simd.as_mut_array()[..4].copy_from_slice(&self.to_array());
        simd
    }

    #[inline]
    pub(crate) const fn to_simd_w(self, w: f32) -> Simd<f32, 4> {
        let [x, y, z] = self.to_array();

        Simd::from_array([x, y, z, w])
    }

    /// Construct a vector where all elements are set to the given value.
    #[inline]
    pub const fn splat(value: f32) -> Self {
        Self::from_array([value; 3])
    }

    #[inline]
    pub fn product(self) -> f32 {
        self.to_simd_w(1.0).reduce_product()
    }

    #[inline]
    pub fn xy(self) -> Vec2 {
        let [x, y, z] = self.to_array();
        let _ = z;

        Vec2::from_array([x, y])
    }

    #[inline]
    pub fn cos(self) -> Self {
        let [x, y, z] = self.to_array();
        let x = x.cos();
        let y = y.cos();
        let z = z.cos();

        Self { x, y, z }
    }

    #[inline]
    pub fn sin(self) -> Self {
        let [x, y, z] = self.to_array();
        let x = x.sin();
        let y = y.sin();
        let z = z.sin();

        Self { x, y, z }
    }

    #[inline]
    pub fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    /// Convert an angle into three normalized vectors.
    #[inline]
    pub fn to_vectors(&self) -> (Self, Self, Self) {
        let angle = self.to_radians();
        let (sin, cos) = angle.sin_cos();

        let x = cos.x * cos.y;
        let y = cos.x * sin.y;
        let z = -sin.x;
        let forward = Vec3::from_array([x, y, z]);

        let x = (-sin.z * sin.x * cos.y) + (-cos.z * -sin.y);
        let y = (-sin.z * sin.x * sin.y) + (-cos.z * cos.y);
        let z = -sin.z * cos.x;
        let right = Vec3::from_array([x, y, z]);

        let x = (cos.z * sin.x * cos.y) + (-sin.z * -sin.y);
        let y = (cos.z * sin.x * sin.y) + (-sin.z * cos.y);
        let z = cos.z * cos.x;
        let up = Vec3::from_array([x, y, z]);

        (forward, right, up)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();

        if magnitude != 0.0 {
            self / Self::splat(magnitude)
        } else {
            Self::from_array([0.0, 0.0, 1.0])
        }
    }

    #[inline]
    pub fn dir(self, forward: Self, right: Self) -> Self {
        let x = forward.x * self.x + right.x * self.y;
        let y = forward.y * self.x + right.y * self.y;
        let z = 0.0;

        Vec3::from_array([x, y, z])
    }

    /// Calculate movement vectors from the current view angle and a wish view angle.
    #[inline]
    pub fn movement(mut self, curr_angle: Self, wish_angle: Self) -> Self {
        let (mut curr_forward, mut curr_right, _curr_up) = curr_angle.to_vectors();
        let (mut wish_forward, mut wish_right, _wish_up) = wish_angle.to_vectors();

        curr_forward.z = 0.0;
        curr_right.z = 0.0;
        wish_forward.z = 0.0;
        wish_right.z = 0.0;

        wish_forward = wish_forward.normalize();
        wish_right = wish_right.normalize();
        curr_forward = curr_forward.normalize();
        curr_right = curr_right.normalize();

        // self is command.movement
        let curr_dir = self.dir(curr_forward, curr_right);
        let wish_dir = self.dir(wish_forward, wish_right);

        if wish_dir != curr_dir {
            let denominator = curr_right.y * curr_forward.x - curr_right.x * curr_forward.y;

            self.x = (wish_dir.x * curr_right.y - wish_dir.y * curr_right.x) / denominator;
            self.y = (wish_dir.y * curr_forward.x - wish_dir.x * curr_forward.y) / denominator;
        }

        self
    }

    #[inline]
    pub fn normalize_angle(mut self) -> Self {
        while self.x > 89.0 {
            self.x -= 180.0;
        }

        while self.x < -89.0 {
            self.x += 180.0;
        }

        while self.y > 180.0 {
            self.y -= 360.0;
        }

        while self.y < -180.0 {
            self.y += 360.0;
        }

        while self.z > 180.0 {
            self.z -= 360.0;
        }

        while self.z < -180.0 {
            self.z += 360.0;
        }

        self
    }

    #[inline]
    pub fn clamp_angle(mut self) -> Self {
        self.x = self.x.clamp(-89.0, 89.0);
        self.y = self.y.clamp(-180.0, 180.0);
        self.z = self.z.clamp(-50.0, 50.0);
        self
    }

    #[inline]
    pub fn sanitize_angle(self) -> Self {
        self.normalize_angle().clamp_angle()
    }

    /// Forward direction vector to euler angles.
    #[inline]
    pub fn to_angle(self) -> Self {
        let [x, y, z] = self.to_array();

        let (x, y) = if !(x != 0.0 || y != 0.0) {
            let pitch = if z > 0.0 { 270.0 } else { 90.0 };
            let yaw = 0.0;

            (pitch, yaw)
        } else {
            let mut pitch = (-z).atan2(self.xy().magnitude()).to_degrees();

            if pitch < 0.0 {
                pitch += 360.0;
            }

            let mut yaw = y.atan2(x).to_degrees();

            if yaw < 0.0 {
                yaw += 360.0;
            }

            (pitch, yaw)
        };

        let z = 0.0;

        Self::from_array([x, y, z])
    }

    impl_methods! {}
}

impl_all_ops! { Vec3 }
