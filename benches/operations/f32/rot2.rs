use std::ops::Mul;

use ggmath::{Rot2, Rot2A, Vec2, Vec2A};
use wide::f32x4;

use crate::{ARRAY_LEN, BATCH_LEN, bench};

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Rot2<f32> as Mul>::mul),
    (aligned, <Rot2A<f32> as Mul>::mul),
    (x4_unaligned, <Rot2<f32x4> as Mul>::mul),
);

bench!(
    vector_mul,
    ARRAY_LEN,
    (unaligned, <Vec2<f32> as Mul<Rot2<f32>>>::mul),
    (aligned, <Vec2A<f32> as Mul<Rot2A<f32>>>::mul),
    (x4_unaligned, <Vec2<f32x4> as Mul<Rot2<f32x4>>>::mul),
);

bench!(
    vector_mul_batched,
    ARRAY_LEN / BATCH_LEN,
    (
        unaligned,
        |vectors: [Vec2<f32>; BATCH_LEN], r: Rot2<f32>| vectors.map(|v| v * r)
    ),
    (
        aligned,
        |vectors: [Vec2A<f32>; BATCH_LEN], r: Rot2A<f32>| vectors.map(|v| v * r)
    ),
    (
        x4_unaligned,
        |vectors: [Vec2<f32x4>; BATCH_LEN], r: Rot2<f32x4>| vectors.map(|v| v * r)
    ),
);
