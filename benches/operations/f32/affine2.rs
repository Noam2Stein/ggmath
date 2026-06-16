use std::ops::Mul;

use ggmath::{Affine2, Affine2A, Vec2, Vec2A};
use wide::f32x4;

use crate::{ARRAY_LEN, BATCH_LEN, bench};

bench!(
    inverse,
    ARRAY_LEN,
    (unaligned, |m: Affine2<f32>| m.inverse()),
    (aligned, |m: Affine2A<f32>| m.inverse()),
    (aligned_glam, |m: glam::Affine2| m.inverse()),
    (x4_unaligned, |m: Affine2<f32x4>| m.inverse()),
);

bench!(
    transform_point,
    ARRAY_LEN,
    (unaligned, |a: Affine2<f32>, p| a.transform_point(p)),
    (aligned, |a: Affine2A<f32>, p| a.transform_point(p)),
    (aligned_glam, |a: glam::Affine2, p| a.transform_point2(p)),
    (x4_unaligned, |a: Affine2<f32x4>, p| a.transform_point(p)),
);

bench!(
    transform_point_batched,
    ARRAY_LEN / BATCH_LEN,
    (
        unaligned,
        |points: [Vec2<f32>; BATCH_LEN], a: Affine2<f32>| points.map(|p| a.transform_point(p))
    ),
    (
        aligned,
        |points: [Vec2A<f32>; BATCH_LEN], a: Affine2A<f32>| points.map(|p| a.transform_point(p))
    ),
    (
        aligned_glam,
        |points: [glam::Vec2; BATCH_LEN], a: glam::Affine2| points.map(|p| a.transform_point2(p))
    ),
    (
        x4_unaligned,
        |points: [Vec2<f32x4>; BATCH_LEN], a: Affine2<f32x4>| points.map(|p| a.transform_point(p))
    ),
);

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Affine2<f32> as Mul>::mul),
    (aligned, <Affine2A<f32> as Mul>::mul),
    (aligned_glam, <glam::Affine2 as Mul>::mul),
    (x4_unaligned, <Affine2<f32x4> as Mul>::mul),
);
