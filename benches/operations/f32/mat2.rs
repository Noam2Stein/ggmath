use std::ops::Mul;

use ggmath::{Mat2, Mat2A, Vec2, Vec2A};
use wide::f32x4;

use crate::{ARRAY_LEN, BATCH_LEN, bench};

bench!(
    determinant,
    ARRAY_LEN,
    (unaligned, |m: Mat2<f32>| m.determinant()),
    (aligned, |m: Mat2A<f32>| m.determinant()),
    (aligned_glam, |m: glam::Mat2| m.determinant()),
    (x4_unaligned, |m: Mat2<f32x4>| m.determinant()),
);

bench!(
    inverse,
    ARRAY_LEN,
    (unaligned, |m: Mat2<f32>| m.inverse()),
    (aligned, |m: Mat2A<f32>| m.inverse()),
    (aligned_glam, |m: glam::Mat2| m.inverse()),
    (x4_unaligned, |m: Mat2<f32x4>| m.inverse()),
);

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Mat2<f32> as Mul>::mul),
    (aligned, <Mat2A<f32> as Mul>::mul),
    (aligned_glam, <glam::Mat2 as Mul>::mul),
    (x4_unaligned, <Mat2<f32x4> as Mul>::mul),
);

bench!(
    vector_mul,
    ARRAY_LEN,
    (unaligned, <Vec2<f32> as Mul<Mat2<f32>>>::mul),
    (aligned, <Vec2A<f32> as Mul<Mat2A<f32>>>::mul),
    (aligned_glam, <glam::Mat2 as Mul<glam::Vec2>>::mul),
    (x4_unaligned, <Vec2<f32x4> as Mul<Mat2<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    ARRAY_LEN / BATCH_LEN,
    (
        unaligned,
        |vectors: [Vec2<f32>; BATCH_LEN], m: Mat2<f32>| vectors.map(|v| v * m)
    ),
    (
        aligned,
        |vectors: [Vec2A<f32>; BATCH_LEN], m: Mat2A<f32>| vectors.map(|v| v * m)
    ),
    (
        aligned_glam,
        |vectors: [glam::Vec2; BATCH_LEN], m: glam::Mat2| vectors.map(|v| m * v)
    ),
    (
        x4_unaligned,
        |vectors: [Vec2<f32x4>; BATCH_LEN], m: Mat2<f32x4>| vectors.map(|v| v * m)
    ),
);
