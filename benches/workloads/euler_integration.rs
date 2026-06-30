use std::hint::black_box;

use ggmath::{Vec2, Vec3, Vec3A};
use wide::{f32x4, f32x8};

use crate::{ARRAY_LEN, bench};

macro_rules! function {
    ($Vector:ty) => {
        |mut position: $Vector, mut velocity: $Vector| {
            let acceleration = black_box(<$Vector>::splat(20.0));
            let delta_time = black_box(0.02);

            velocity += acceleration * delta_time;
            position += velocity * delta_time;

            (position, velocity)
        }
    };
}

macro_rules! soa_function {
    ($Vector:ty, $Simd:ty) => {
        |mut position: $Vector, mut velocity: $Vector| {
            let acceleration = black_box(<$Vector>::splat(<$Simd>::splat(20.0)));
            let delta_time = black_box(<$Simd>::splat(0.02));

            velocity += acceleration * delta_time;
            position += velocity * delta_time;

            (position, velocity)
        }
    };
}

bench!(
    vec2f32,
    ARRAY_LEN,
    (unaligned, function!(Vec2<f32>)),
    (unaligned_glam, function!(glam::Vec2)),
    (x4_unaligned, soa_function!(Vec2<f32x4>, f32x4)),
    (x8_unaligned, soa_function!(Vec2<f32x8>, f32x8)),
);

bench!(
    vec3f32,
    ARRAY_LEN,
    (unaligned, function!(Vec3<f32>)),
    (unaligned_glam, function!(glam::Vec3)),
    (aligned, function!(Vec3A<f32>)),
    (aligned_glam, function!(glam::Vec3A)),
    (x4_unaligned, soa_function!(Vec3<f32x4>, f32x4)),
    (x8_unaligned, soa_function!(Vec3<f32x8>, f32x8)),
);
