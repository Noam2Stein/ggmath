use std::ops::Mul;

use ggmath::{Quat, QuatA, Vec3, Vec3A};
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, TRANSFORM_BATCH, bench};

bench!(
    vector_mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3<f32> as Mul<Quat<f32>>>::mul),
    (aligned, <Vec3A<f32> as Mul<QuatA<f32>>>::mul),
    (aligned_glam, <glam::Quat as Mul<glam::Vec3A>>::mul),
    (x4_unaligned, <Vec3<f32x4> as Mul<Quat<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    MICROBENCH_ARRAY_LEN / TRANSFORM_BATCH,
    (
        unaligned,
        |vectors: [Vec3<f32>; TRANSFORM_BATCH], q: Quat<f32>| vectors.map(|v| v * q)
    ),
    (
        aligned,
        |vectors: [Vec3A<f32>; TRANSFORM_BATCH], q: QuatA<f32>| vectors.map(|v| v * q)
    ),
    (
        aligned_glam,
        |vectors: [glam::Vec3A; TRANSFORM_BATCH], q: glam::Quat| vectors.map(|v| q * v)
    ),
    (
        x4_unaligned,
        |vectors: [Vec3<f32x4>; TRANSFORM_BATCH], q: Quat<f32x4>| vectors.map(|v| v * q)
    ),
);

bench!(
    mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Quat<f32> as Mul>::mul),
    (aligned, <QuatA<f32> as Mul>::mul),
    (aligned_glam, <glam::Quat as Mul>::mul),
    (x4_unaligned, <Quat<f32x4> as Mul>::mul),
);
