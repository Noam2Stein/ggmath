use std::ops::Mul;

use ggmath::{Rotor3, Rotor3A, Vec3, Vec3A};
use wide::f32x4;

use crate::{ARRAY_LEN, BATCH_LEN, bench};

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Rotor3<f32> as Mul>::mul),
    (aligned, <Rotor3A<f32> as Mul>::mul),
    (aligned_glam, <glam::Quat as Mul>::mul),
    (x4_unaligned, <Rotor3<f32x4> as Mul>::mul),
);

bench!(
    vector_mul,
    ARRAY_LEN,
    (unaligned, <Vec3<f32> as Mul<Rotor3<f32>>>::mul),
    (aligned, <Vec3A<f32> as Mul<Rotor3A<f32>>>::mul),
    (aligned_glam, <glam::Quat as Mul<glam::Vec3A>>::mul),
    (x4_unaligned, <Vec3<f32x4> as Mul<Rotor3<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    ARRAY_LEN / BATCH_LEN,
    (
        unaligned,
        |vectors: [Vec3<f32>; BATCH_LEN], r: Rotor3<f32>| vectors.map(|v| v * r)
    ),
    (
        aligned,
        |vectors: [Vec3A<f32>; BATCH_LEN], r: Rotor3A<f32>| vectors.map(|v| v * r)
    ),
    (
        aligned_glam,
        |vectors: [glam::Vec3A; BATCH_LEN], r: glam::Quat| vectors.map(|v| r * v)
    ),
    (
        x4_unaligned,
        |vectors: [Vec3<f32x4>; BATCH_LEN], r: Rotor3<f32x4>| vectors.map(|v| v * r)
    ),
);
