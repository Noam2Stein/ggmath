use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use ggmath::{Vec4, Vec4A};
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, bench};

bench!(
    neg,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::neg),
    (aligned, Vec4A::<f32>::neg),
    (aligned_glam, glam::Vec4::neg),
    (x4_unaligned, Vec4::<f32x4>::neg),
);

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

bench!(
    floor,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::floor),
    (aligned, Vec4A::<f32>::floor),
    (aligned_glam, glam::Vec4::floor),
    (x4_unaligned, Vec4::<f32x4>::floor),
);

bench!(
    ceil,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::ceil),
    (aligned, Vec4A::<f32>::ceil),
    (aligned_glam, glam::Vec4::ceil),
    (x4_unaligned, Vec4::<f32x4>::ceil),
);

bench!(
    round,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::round),
    (aligned, Vec4A::<f32>::round),
    (aligned_glam, glam::Vec4::round),
    (x4_unaligned, Vec4::<f32x4>::round),
);

bench!(
    trunc,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::trunc),
    (aligned, Vec4A::<f32>::trunc),
    (aligned_glam, glam::Vec4::trunc),
    (x4_unaligned, Vec4::<f32x4>::trunc),
);

bench!(
    fract,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec4::<f32>::fract),
    (aligned, Vec4A::<f32>::fract),
    (aligned_glam, glam::Vec4::fract),
    (x4_unaligned, Vec4::<f32x4>::fract),
);
