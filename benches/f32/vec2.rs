use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use ggmath::Vec2;
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, bench};

bench!(
    neg,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::neg),
    (unaligned_glam, glam::Vec2::neg),
    (x4_unaligned, Vec2::<f32x4>::neg),
);

bench!(
    add,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Add>::add),
    (unaligned_glam, <glam::Vec2 as Add>::add),
    (x4_unaligned, <Vec2::<f32x4> as Add>::add),
);

bench!(
    sub,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Sub>::sub),
    (unaligned_glam, <glam::Vec2 as Sub>::sub),
    (x4_unaligned, <Vec2::<f32x4> as Sub>::sub),
);

bench!(
    mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Mul>::mul),
    (unaligned_glam, <glam::Vec2 as Mul>::mul),
    (x4_unaligned, <Vec2::<f32x4> as Mul>::mul),
);

bench!(
    div,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Div>::div),
    (unaligned_glam, <glam::Vec2 as Div>::div),
    (x4_unaligned, <Vec2::<f32x4> as Div>::div),
);

bench!(
    rem,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Rem>::rem),
    (unaligned_glam, <glam::Vec2 as Rem>::rem),
    (x4_unaligned, <Vec2::<f32x4> as Rem>::rem),
);

bench!(
    length,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::length),
    (unaligned_glam, glam::Vec2::length),
    (x4_unaligned, Vec2::<f32x4>::length),
);

bench!(
    normalize,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::normalize),
    (unaligned_glam, glam::Vec2::normalize),
    (x4_unaligned, Vec2::<f32x4>::normalize),
);

bench!(
    dot,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::dot),
    (unaligned_glam, glam::Vec2::dot),
    (x4_unaligned, Vec2::<f32x4>::dot),
);

bench!(
    project_onto,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::project_onto),
    (unaligned_glam, glam::Vec2::project_onto),
    (x4_unaligned, Vec2::<f32x4>::project_onto),
);

bench!(
    reject_from,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::reject_from),
    (unaligned_glam, glam::Vec2::reject_from),
    (x4_unaligned, Vec2::<f32x4>::reject_from),
);

bench!(
    reflect,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::reflect),
    (unaligned_glam, glam::Vec2::reflect),
    (x4_unaligned, Vec2::<f32x4>::reflect),
);

bench!(
    refract,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec2::<f32>::refract),
    (unaligned_glam, glam::Vec2::refract),
    (x4_unaligned, Vec2::<f32x4>::refract),
);
