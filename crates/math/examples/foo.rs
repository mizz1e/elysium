use elysium_math::{Vec2, Vec3, Vec4};

fn main() {
    println!("{:?}", Vec3::splat(1.0) + Vec3::splat(2.0));
    println!("{:?}", Vec3::splat(1.0) * Vec3::splat(2.0));
    println!("{:?}", Vec3::splat(1.0) / Vec3::splat(2.0));
    println!("{:?}", Vec3::splat(1.0) % Vec3::splat(2.0));
    println!("{:?}", Vec4::splat(1.0) + Vec4::splat(2.0));

    println!("{:?}", Vec2::splat(1.0).product());
    println!("{:?}", Vec3::splat(1.0).product());
    println!("{:?}", Vec4::splat(1.0).product());
    println!("{:?}", Vec3::splat(1.0).sum());

    let a = Vec3::from_array([1.0, 1.0, 0.0]);
    let b = Vec3::from_array([1.0, 0.0, 0.0]);

    println!("{:?}", a.distance(b));
}
