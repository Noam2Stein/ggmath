use std::ops::Mul;

use ggmath::{Mat4, Mat4A, Vec4, Vec4A};
use wide::f32x4;

use crate::{ARRAY_LEN, BATCH_LEN, bench};

bench!(
    determinant,
    ARRAY_LEN,
    (unaligned, |m: Mat4<f32>| m.determinant()),
    (aligned, |m: Mat4A<f32>| m.determinant()),
    (aligned_glam, |m: glam::Mat4| m.determinant()),
    (x4_unaligned, |m: Mat4<f32x4>| m.determinant()),
);

bench!(
    inverse,
    ARRAY_LEN,
    (unaligned, |m: Mat4<f32>| m.inverse()),
    (aligned, |m: Mat4A<f32>| m.inverse()),
    (aligned_glam, |m: glam::Mat4| m.inverse()),
    (x4_unaligned, |m: Mat4<f32x4>| m.inverse()),
);

bench!(
    vector_mul,
    ARRAY_LEN,
    (unaligned, <Vec4<f32> as Mul<Mat4<f32>>>::mul),
    (aligned, <Vec4A<f32> as Mul<Mat4A<f32>>>::mul),
    (aligned_glam, <glam::Mat4 as Mul<glam::Vec4>>::mul),
    (x4_unaligned, <Vec4<f32x4> as Mul<Mat4<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    ARRAY_LEN / BATCH_LEN,
    (
        unaligned,
        |vectors: [Vec4<f32>; BATCH_LEN], m: Mat4<f32>| vectors.map(|v| v * m)
    ),
    (
        aligned,
        |vectors: [Vec4A<f32>; BATCH_LEN], m: Mat4A<f32>| vectors.map(|v| v * m)
    ),
    (
        aligned_glam,
        |vectors: [glam::Vec4; BATCH_LEN], m: glam::Mat4| vectors.map(|v| m * v)
    ),
    (
        x4_unaligned,
        |vectors: [Vec4<f32x4>; BATCH_LEN], m: Mat4<f32x4>| vectors.map(|v| v * m)
    ),
);

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Mat4<f32> as Mul>::mul),
    (aligned, <Mat4A<f32> as Mul>::mul),
    (aligned_glam, <glam::Mat4 as Mul>::mul),
    (x4_unaligned, <Mat4<f32x4> as Mul>::mul),
);
