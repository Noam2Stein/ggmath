use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use ggmath::Vec2;
use wide::f32x4;

use crate::{ARRAY_LEN, bench};

bench!(
    neg,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::neg),
    (unaligned_glam, glam::Vec2::neg),
    (x4_unaligned, Vec2::<f32x4>::neg),
);

bench!(
    add,
    ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Add>::add),
    (unaligned_glam, <glam::Vec2 as Add>::add),
    (x4_unaligned, <Vec2::<f32x4> as Add>::add),
);

bench!(
    sub,
    ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Sub>::sub),
    (unaligned_glam, <glam::Vec2 as Sub>::sub),
    (x4_unaligned, <Vec2::<f32x4> as Sub>::sub),
);

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Mul>::mul),
    (unaligned_glam, <glam::Vec2 as Mul>::mul),
    (x4_unaligned, <Vec2::<f32x4> as Mul>::mul),
);

bench!(
    div,
    ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Div>::div),
    (unaligned_glam, <glam::Vec2 as Div>::div),
    (x4_unaligned, <Vec2::<f32x4> as Div>::div),
);

bench!(
    rem,
    ARRAY_LEN,
    (unaligned, <Vec2::<f32> as Rem>::rem),
    (unaligned_glam, <glam::Vec2 as Rem>::rem),
    (x4_unaligned, <Vec2::<f32x4> as Rem>::rem),
);

bench!(
    length,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::length),
    (unaligned_glam, glam::Vec2::length),
    (x4_unaligned, Vec2::<f32x4>::length),
);

bench!(
    normalize,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::normalize),
    (unaligned_glam, glam::Vec2::normalize),
    (x4_unaligned, Vec2::<f32x4>::normalize),
);

bench!(
    dot,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::dot),
    (unaligned_glam, glam::Vec2::dot),
    (x4_unaligned, Vec2::<f32x4>::dot),
);

bench!(
    project_onto,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::project_onto),
    (unaligned_glam, glam::Vec2::project_onto),
    (x4_unaligned, Vec2::<f32x4>::project_onto),
);

bench!(
    reject_from,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::reject_from),
    (unaligned_glam, glam::Vec2::reject_from),
    (x4_unaligned, Vec2::<f32x4>::reject_from),
);

bench!(
    reflect,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::reflect),
    (unaligned_glam, glam::Vec2::reflect),
    (x4_unaligned, Vec2::<f32x4>::reflect),
);

bench!(
    refract,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::refract),
    (unaligned_glam, glam::Vec2::refract),
    (x4_unaligned, Vec2::<f32x4>::refract),
);

bench!(
    move_towards,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::move_towards),
    (unaligned_glam, glam::Vec2::move_towards),
    (x4_unaligned, Vec2::<f32x4>::move_towards),
);

bench!(
    slerp,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::slerp),
    // This function is not in `glam`.
    (x4_unaligned, Vec2::<f32x4>::slerp),
);

bench!(
    rotate_towards,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::rotate_towards),
    (unaligned_glam, glam::Vec2::rotate_towards),
    (x4_unaligned, Vec2::<f32x4>::rotate_towards),
);

bench!(
    with_max_length,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::with_max_length),
    (unaligned_glam, glam::Vec2::clamp_length_max),
    (x4_unaligned, Vec2::<f32x4>::with_max_length),
);

bench!(
    with_min_length,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::with_min_length),
    (unaligned_glam, glam::Vec2::clamp_length_min),
    (x4_unaligned, Vec2::<f32x4>::with_min_length),
);

bench!(
    clamp_length,
    ARRAY_LEN,
    (unaligned, Vec2::<f32>::clamp_length),
    (unaligned_glam, glam::Vec2::clamp_length),
    (x4_unaligned, Vec2::<f32x4>::clamp_length),
);
