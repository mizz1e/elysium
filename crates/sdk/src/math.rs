use bevy::prelude::*;

/// [game/shared/gamemovement.h#L104](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/shared/gamemovement.h#L104)
pub const AIR_SPEED_CAP: f32 = 30.0;

// [strafe theory](https://web.archive.org/web/20150217142101/http://www.funender.com/quake/articles/strafing_theory.html).
// [quake physics](https://www.quakeworld.nu/wiki/QW_physics_air).

#[inline]
pub fn normalize_component(mut value: f32) -> f32 {
    if !value.is_finite() {
        return 0.0;
    }

    while value > 180.0 {
        value -= 360.0;
    }

    while value < -180.0 {
        value += 360.0;
    }

    value
}

#[inline]
pub fn normalize_angle(mut angle: Vec3) -> Vec3 {
    angle.x = normalize_component(angle.x);
    angle.y = normalize_component(angle.y);
    angle.z = normalize_component(angle.z);
    angle
}

#[inline]
pub fn clamp_angle(angle: Vec3) -> Vec3 {
    const MAX: Vec3 = Vec3::new(89.0, 180.0, 50.0);

    angle.clamp(-MAX, MAX)
}

#[inline]
pub fn sanitize_angle(angle: Vec3) -> Vec3 {
    clamp_angle(normalize_angle(angle))
}

#[inline]
pub fn sin(v: Vec3) -> Vec3 {
    Vec3 {
        x: v.x.sin(),
        y: v.y.sin(),
        z: v.z.sin(),
    }
}

#[inline]
pub fn cos(v: Vec3) -> Vec3 {
    Vec3 {
        x: v.x.cos(),
        y: v.y.cos(),
        z: v.z.cos(),
    }
}

#[inline]
pub fn sin_cos(v: Vec3) -> (Vec3, Vec3) {
    (sin(v), cos(v))
}

#[inline]
pub fn to_radians(angle: Vec3) -> Vec3 {
    angle * 1.0_f32.to_radians()
}

#[inline]
pub fn to_degrees(angle: Vec3) -> Vec3 {
    angle * 1.0_f32.to_degrees()
}

#[inline]
pub fn to_vectors(angle: Vec3) -> (Vec3, Vec3, Vec3) {
    let (sin, cos) = sin_cos(to_radians(angle));

    let x = cos.x * cos.y;
    let y = cos.x * sin.y;
    let z = -sin.x;
    let forward = Vec3::new(x, y, z);

    let x = (-sin.z * sin.x * cos.y) + (-cos.z * -sin.y);
    let y = (-sin.z * sin.x * sin.y) + (-cos.z * cos.y);
    let z = -sin.z * cos.x;
    let right = Vec3::new(x, y, z);

    let x = (cos.z * sin.x * cos.y) + (-sin.z * -sin.y);
    let y = (cos.z * sin.x * sin.y) + (-sin.z * cos.y);
    let z = cos.z * cos.x;
    let up = Vec3::new(x, y, z);

    (forward, right, up)
}

#[inline]
pub fn direction(movement: Vec3, forward: Vec3, right: Vec3) -> Vec3 {
    let movement = movement.truncate();
    let forward = forward.truncate();
    let right = right.truncate();

    (forward * movement.x + right * movement.y).extend(0.0)
}

/// Calculate movement vectors from the current view angle and a wish view angle.
#[inline]
pub fn fix_movement(mut movement: Vec3, angle: Vec3, wish_angle: Vec3) -> Vec3 {
    let (mut forward, mut right, _up) = to_vectors(angle);
    let (mut wish_forward, mut wish_right, _wish_up) = to_vectors(wish_angle);

    forward.z = 0.0;
    right.z = 0.0;
    wish_forward.z = 0.0;
    wish_right.z = 0.0;

    forward = forward.normalize_or_zero();
    right = right.normalize_or_zero();
    wish_forward = wish_forward.normalize_or_zero();
    wish_right = wish_right.normalize_or_zero();

    let dir = direction(movement, forward, right);
    let wish_dir = direction(movement, wish_forward, wish_right);

    if wish_dir != dir {
        let denominator = right.y * forward.x - right.x * forward.y;

        movement.x = (wish_dir.x * right.y - wish_dir.y * right.x) / denominator;
        movement.y = (wish_dir.y * forward.x - wish_dir.x * forward.y) / denominator;
    }

    movement
}
