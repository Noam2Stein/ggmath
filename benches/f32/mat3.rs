use std::ops::Mul;

use ggmath::{Mat3, Mat3A, Vec3, Vec3A};
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, TRANSFORM_BATCH, bench};

bench!(
    determinant,
    MICROBENCH_ARRAY_LEN,
    (unaligned, |m: Mat3<f32>| m.determinant()),
    (unaligned_glam, |m: glam::Mat3| m.determinant()),
    (aligned, |m: Mat3A<f32>| m.determinant()),
    (aligned_glam, |m: glam::Mat3A| m.determinant()),
    (x4_unaligned, |m: Mat3<f32x4>| m.determinant()),
);

bench!(
    inverse,
    MICROBENCH_ARRAY_LEN,
    (unaligned, |m: Mat3<f32>| m.inverse()),
    (unaligned_glam, |m: glam::Mat3| m.inverse()),
    (aligned, |m: Mat3A<f32>| m.inverse()),
    (aligned_glam, |m: glam::Mat3A| m.inverse()),
    (x4_unaligned, |m: Mat3<f32x4>| m.inverse()),
);

bench!(
    vector_mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3<f32> as Mul<Mat3<f32>>>::mul),
    (unaligned_glam, <glam::Mat3 as Mul<glam::Vec3>>::mul),
    (aligned, <Vec3A<f32> as Mul<Mat3A<f32>>>::mul),
    (aligned_glam, <glam::Mat3A as Mul<glam::Vec3A>>::mul),
    (x4_unaligned, <Vec3<f32x4> as Mul<Mat3<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    MICROBENCH_ARRAY_LEN / TRANSFORM_BATCH,
    (
        unaligned,
        |vectors: [Vec3<f32>; TRANSFORM_BATCH], m: Mat3<f32>| vectors.map(|v| v * m)
    ),
    (
        unaligned_glam,
        |vectors: [glam::Vec3; TRANSFORM_BATCH], m: glam::Mat3| vectors.map(|v| m * v)
    ),
    (
        aligned,
        |vectors: [Vec3A<f32>; TRANSFORM_BATCH], m: Mat3A<f32>| vectors.map(|v| v * m)
    ),
    (
        aligned_glam,
        |vectors: [glam::Vec3A; TRANSFORM_BATCH], m: glam::Mat3A| vectors.map(|v| m * v)
    ),
    (
        x4_unaligned,
        |vectors: [Vec3<f32x4>; TRANSFORM_BATCH], m: Mat3<f32x4>| vectors.map(|v| v * m)
    ),
);

bench!(
    mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Mat3<f32> as Mul>::mul),
    (unaligned_glam, <glam::Mat3 as Mul>::mul),
    (aligned, <Mat3A<f32> as Mul>::mul),
    (aligned_glam, <glam::Mat3A as Mul>::mul),
    (x4_unaligned, <Mat3<f32x4> as Mul>::mul),
);
