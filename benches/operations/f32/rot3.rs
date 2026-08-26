use std::ops::Mul;

use ggmath::{Rot3, Rot3A, Vec3, Vec3A};
use wide::f32x4;

use crate::{ARRAY_LEN, BATCH_LEN, bench};

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Rot3<f32> as Mul>::mul),
    (aligned, <Rot3A<f32> as Mul>::mul),
    (aligned_glam, <glam::Quat as Mul>::mul),
    (x4_unaligned, <Rot3<f32x4> as Mul>::mul),
);

bench!(
    vector_mul,
    ARRAY_LEN,
    (unaligned, <Vec3<f32> as Mul<Rot3<f32>>>::mul),
    (aligned, <Vec3A<f32> as Mul<Rot3A<f32>>>::mul),
    (aligned_glam, <glam::Quat as Mul<glam::Vec3A>>::mul),
    (x4_unaligned, <Vec3<f32x4> as Mul<Rot3<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    ARRAY_LEN / BATCH_LEN,
    (
        unaligned,
        |vectors: [Vec3<f32>; BATCH_LEN], q: Rot3<f32>| vectors.map(|v| v * q)
    ),
    (
        aligned,
        |vectors: [Vec3A<f32>; BATCH_LEN], q: Rot3A<f32>| vectors.map(|v| v * q)
    ),
    (
        aligned_glam,
        |vectors: [glam::Vec3A; BATCH_LEN], q: glam::Quat| vectors.map(|v| q * v)
    ),
    (
        x4_unaligned,
        |vectors: [Vec3<f32x4>; BATCH_LEN], q: Rot3<f32x4>| vectors.map(|v| v * q)
    ),
);
