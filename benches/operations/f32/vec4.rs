use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use ggmath::{Vec4, Vec4A};
use wide::f32x4;

use crate::{ARRAY_LEN, bench};

bench!(
    acos,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::acos),
    (aligned, Vec4A::<f32>::acos),
    // This function is not in `glam`.
    (x4_unaligned, Vec4::<f32x4>::acos),
);

bench!(
    add,
    ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Add>::add),
    (aligned, <Vec4A::<f32> as Add>::add),
    (aligned_glam, <glam::Vec4 as Add>::add),
    (x4_unaligned, <Vec4::<f32x4> as Add>::add),
);

bench!(
    asin,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::asin),
    (aligned, Vec4A::<f32>::asin),
    // This function is not in `glam`.
    (x4_unaligned, Vec4::<f32x4>::asin),
);

bench!(
    atan,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::atan),
    (aligned, Vec4A::<f32>::atan),
    // This function is not in `glam`.
    (x4_unaligned, Vec4::<f32x4>::atan),
);

bench!(
    ceil,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::ceil),
    (aligned, Vec4A::<f32>::ceil),
    (aligned_glam, glam::Vec4::ceil),
    (x4_unaligned, Vec4::<f32x4>::ceil),
);

bench!(
    clamp_length,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::clamp_length),
    (aligned, Vec4A::<f32>::clamp_length),
    (aligned_glam, glam::Vec4::clamp_length),
    (x4_unaligned, Vec4::<f32x4>::clamp_length),
);

bench!(
    cos,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::cos),
    (aligned, Vec4A::<f32>::cos),
    (aligned_glam, glam::Vec4::cos),
    (x4_unaligned, Vec4::<f32x4>::cos),
);

bench!(
    div,
    ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Div>::div),
    (aligned, <Vec4A::<f32> as Div>::div),
    (aligned_glam, <glam::Vec4 as Div>::div),
    (x4_unaligned, <Vec4::<f32x4> as Div>::div),
);

bench!(
    dot,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::dot),
    (aligned, Vec4A::<f32>::dot),
    (aligned_glam, glam::Vec4::dot),
    (x4_unaligned, Vec4::<f32x4>::dot),
);

bench!(
    floor,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::floor),
    (aligned, Vec4A::<f32>::floor),
    (aligned_glam, glam::Vec4::floor),
    (x4_unaligned, Vec4::<f32x4>::floor),
);

bench!(
    fract,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::fract),
    (aligned, Vec4A::<f32>::fract),
    (aligned_glam, glam::Vec4::fract),
    (x4_unaligned, Vec4::<f32x4>::fract),
);

bench!(
    length,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::length),
    (aligned, Vec4A::<f32>::length),
    (aligned_glam, glam::Vec4::length),
    (x4_unaligned, Vec4::<f32x4>::length),
);

bench!(
    move_towards,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::move_towards),
    (aligned, Vec4A::<f32>::move_towards),
    (aligned_glam, glam::Vec4::move_towards),
    (x4_unaligned, Vec4::<f32x4>::move_towards),
);

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Mul>::mul),
    (aligned, <Vec4A::<f32> as Mul>::mul),
    (aligned_glam, <glam::Vec4 as Mul>::mul),
    (x4_unaligned, <Vec4::<f32x4> as Mul>::mul),
);

bench!(
    neg,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::neg),
    (aligned, Vec4A::<f32>::neg),
    (aligned_glam, glam::Vec4::neg),
    (x4_unaligned, Vec4::<f32x4>::neg),
);

bench!(
    normalize,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::normalize),
    (aligned, Vec4A::<f32>::normalize),
    (aligned_glam, glam::Vec4::normalize),
    (x4_unaligned, Vec4::<f32x4>::normalize),
);

bench!(
    rem,
    ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Rem>::rem),
    (aligned, <Vec4A::<f32> as Rem>::rem),
    (aligned_glam, <glam::Vec4 as Rem>::rem),
    (x4_unaligned, <Vec4::<f32x4> as Rem>::rem),
);

bench!(
    rotate_towards,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::rotate_towards),
    (aligned, Vec4A::<f32>::rotate_towards),
    // This function is not in `glam`.
    (x4_unaligned, Vec4::<f32x4>::rotate_towards),
);

bench!(
    round,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::round),
    (aligned, Vec4A::<f32>::round),
    (aligned_glam, glam::Vec4::round),
    (x4_unaligned, Vec4::<f32x4>::round),
);

bench!(
    sin,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::sin),
    (aligned, Vec4A::<f32>::sin),
    (aligned_glam, glam::Vec4::sin),
    (x4_unaligned, Vec4::<f32x4>::sin),
);

bench!(
    slerp,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::slerp),
    (aligned, Vec4A::<f32>::slerp),
    // This function is not in `glam`.
    (x4_unaligned, Vec4::<f32x4>::slerp),
);

bench!(
    sub,
    ARRAY_LEN,
    (unaligned, <Vec4::<f32> as Sub>::sub),
    (aligned, <Vec4A::<f32> as Sub>::sub),
    (aligned_glam, <glam::Vec4 as Sub>::sub),
    (x4_unaligned, <Vec4::<f32x4> as Sub>::sub),
);

bench!(
    tan,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::tan),
    (aligned, Vec4A::<f32>::tan),
    // This function is not in `glam`.
    (x4_unaligned, Vec4::<f32x4>::tan),
);

bench!(
    trunc,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::trunc),
    (aligned, Vec4A::<f32>::trunc),
    (aligned_glam, glam::Vec4::trunc),
    (x4_unaligned, Vec4::<f32x4>::trunc),
);

bench!(
    with_max_length,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::with_max_length),
    (aligned, Vec4A::<f32>::with_max_length),
    (aligned_glam, glam::Vec4::clamp_length_max),
    (x4_unaligned, Vec4::<f32x4>::with_max_length),
);

bench!(
    with_min_length,
    ARRAY_LEN,
    (unaligned, Vec4::<f32>::with_min_length),
    (aligned, Vec4A::<f32>::with_min_length),
    (aligned_glam, glam::Vec4::clamp_length_min),
    (x4_unaligned, Vec4::<f32x4>::with_min_length),
);
