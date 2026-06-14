use std::ops::{Add, Div, Mul, Rem, Sub};

use ggmath::{Vec4, Vec4A};
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, bench};

bench!(
    add,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Add>::add),
    (aligned, <Vec4A::<f32> as Add>::add),
    (aligned_glam, <glam::Vec4 as Add>::add),
    (x4_unaligned, <Vec4::<f32x4> as Add>::add),
);

bench!(
    sub,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Sub>::sub),
    (aligned, <Vec4A::<f32> as Sub>::sub),
    (aligned_glam, <glam::Vec4 as Sub>::sub),
    (x4_unaligned, <Vec4::<f32x4> as Sub>::sub),
);

bench!(
    mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Mul>::mul),
    (aligned, <Vec4A::<f32> as Mul>::mul),
    (aligned_glam, <glam::Vec4 as Mul>::mul),
    (x4_unaligned, <Vec4::<f32x4> as Mul>::mul),
);

bench!(
    div,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Div>::div),
    (aligned, <Vec4A::<f32> as Div>::div),
    (aligned_glam, <glam::Vec4 as Div>::div),
    (x4_unaligned, <Vec4::<f32x4> as Div>::div),
);

bench!(
    rem,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Rem>::rem),
    (aligned, <Vec4A::<f32> as Rem>::rem),
    (aligned_glam, <glam::Vec4 as Rem>::rem),
    (x4_unaligned, <Vec4::<f32x4> as Rem>::rem),
);

bench!(
    length,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::length),
    (aligned, Vec4A::<f32>::length),
    (aligned_glam, glam::Vec4::length),
    (x4_unaligned, Vec4::<f32x4>::length),
);

bench!(
    normalize,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::normalize),
    (aligned, Vec4A::<f32>::normalize),
    (aligned_glam, glam::Vec4::normalize),
    (x4_unaligned, Vec4::<f32x4>::normalize),
);

bench!(
    dot,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::dot),
    (aligned, Vec4A::<f32>::dot),
    (aligned_glam, glam::Vec4::dot),
    (x4_unaligned, Vec4::<f32x4>::dot),
);
