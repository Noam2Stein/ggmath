use std::ops::Mul;

use ggmath::{Affine3, Affine3A, Vec3, Vec3A};
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, TRANSFORM_BATCH, bench};

bench!(
    inverse,
    MICROBENCH_ARRAY_LEN,
    (unaligned, |m: Affine3<f32>| m.inverse()),
    (unaligned_glam, |m: glam::Affine3| m.inverse()),
    (aligned, |m: Affine3A<f32>| m.inverse()),
    (aligned_glam, |m: glam::Affine3A| m.inverse()),
    (x4_unaligned, |m: Affine3<f32x4>| m.inverse()),
);

bench!(
    transform_point,
    MICROBENCH_ARRAY_LEN,
    (unaligned, |a: Affine3<f32>, p| a.transform_point(p)),
    (unaligned_glam, |a: glam::Affine3, p| a.transform_point3(p)),
    (aligned, |a: Affine3A<f32>, p| a.transform_point(p)),
    (aligned_glam, |a: glam::Affine3A, p| a.transform_point3(p)),
    (x4_unaligned, |a: Affine3<f32x4>, p| a.transform_point(p)),
);

bench!(
    transform_point_batched,
    MICROBENCH_ARRAY_LEN / TRANSFORM_BATCH,
    (
        unaligned,
        |points: [Vec3<f32>; TRANSFORM_BATCH], a: Affine3<f32>| points
            .map(|p| a.transform_point(p))
    ),
    (
        unaligned_glam,
        |points: [glam::Vec3; TRANSFORM_BATCH], a: glam::Affine3| points
            .map(|p| a.transform_point3(p))
    ),
    (
        aligned,
        |points: [Vec3A<f32>; TRANSFORM_BATCH], a: Affine3A<f32>| points
            .map(|p| a.transform_point(p))
    ),
    (
        aligned_glam,
        |points: [glam::Vec3A; TRANSFORM_BATCH], a: glam::Affine3A| points
            .map(|p| a.transform_point3a(p))
    ),
    (
        x4_unaligned,
        |points: [Vec3<f32x4>; TRANSFORM_BATCH], a: Affine3<f32x4>| points
            .map(|p| a.transform_point(p))
    ),
);

bench!(
    mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Affine3<f32> as Mul>::mul),
    (unaligned_glam, <glam::Affine3 as Mul>::mul),
    (aligned, <Affine3A<f32> as Mul>::mul),
    (aligned_glam, <glam::Affine3A as Mul>::mul),
    (x4_unaligned, <Affine3<f32x4> as Mul>::mul),
);
