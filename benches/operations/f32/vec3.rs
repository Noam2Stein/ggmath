use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use ggmath::{Vec3, Vec3A};
use wide::f32x4;

use crate::{ARRAY_LEN, bench};

bench!(
    add,
    ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Add>::add),
    (unaligned_glam, <glam::Vec3 as Add>::add),
    (aligned, <Vec3A::<f32> as Add>::add),
    (aligned_glam, <glam::Vec3A as Add>::add),
    (x4_unaligned, <Vec3::<f32x4> as Add>::add),
);

bench!(
    angle_between,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::angle_between),
    (unaligned_glam, glam::Vec3::angle_between),
    (aligned, Vec3A::<f32>::angle_between),
    (aligned_glam, glam::Vec3A::angle_between),
    (x4_unaligned, Vec3::<f32x4>::angle_between),
);

bench!(
    any_orthogonal_vector,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::any_orthogonal_vector),
    (unaligned_glam, glam::Vec3::any_orthogonal_vector),
    (aligned, Vec3A::<f32>::any_orthogonal_vector),
    (aligned_glam, glam::Vec3A::any_orthogonal_vector),
    (x4_unaligned, Vec3::<f32x4>::any_orthogonal_vector),
);

bench!(
    any_orthonormal_pair,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::any_orthonormal_pair),
    (unaligned_glam, glam::Vec3::any_orthonormal_pair),
    (aligned, Vec3A::<f32>::any_orthonormal_pair),
    (aligned_glam, glam::Vec3A::any_orthonormal_pair),
    (x4_unaligned, Vec3::<f32x4>::any_orthonormal_pair),
);

bench!(
    any_orthonormal_vector,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::any_orthonormal_vector),
    (unaligned_glam, glam::Vec3::any_orthonormal_vector),
    (aligned, Vec3A::<f32>::any_orthonormal_vector),
    (aligned_glam, glam::Vec3A::any_orthonormal_vector),
    (x4_unaligned, Vec3::<f32x4>::any_orthonormal_vector),
);

bench!(
    ceil,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::ceil),
    (unaligned_glam, glam::Vec3::ceil),
    (aligned, Vec3A::<f32>::ceil),
    (aligned_glam, glam::Vec3A::ceil),
    (x4_unaligned, Vec3::<f32x4>::ceil),
);

bench!(
    clamp_length,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::clamp_length),
    (unaligned_glam, glam::Vec3::clamp_length),
    (aligned, Vec3A::<f32>::clamp_length),
    (aligned_glam, glam::Vec3A::clamp_length),
    (x4_unaligned, Vec3::<f32x4>::clamp_length),
);

bench!(
    cross,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::cross),
    (unaligned_glam, glam::Vec3::cross),
    (aligned, Vec3A::<f32>::cross),
    (aligned_glam, glam::Vec3A::cross),
    (x4_unaligned, Vec3::<f32x4>::cross),
);

bench!(
    div,
    ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Div>::div),
    (unaligned_glam, <glam::Vec3 as Div>::div),
    (aligned, <Vec3A::<f32> as Div>::div),
    (aligned_glam, <glam::Vec3A as Div>::div),
    (x4_unaligned, <Vec3::<f32x4> as Div>::div),
);

bench!(
    dot,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::dot),
    (unaligned_glam, glam::Vec3::dot),
    (aligned, Vec3A::<f32>::dot),
    (aligned_glam, glam::Vec3A::dot),
    (x4_unaligned, Vec3::<f32x4>::dot),
);

bench!(
    floor,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::floor),
    (unaligned_glam, glam::Vec3::floor),
    (aligned, Vec3A::<f32>::floor),
    (aligned_glam, glam::Vec3A::floor),
    (x4_unaligned, Vec3::<f32x4>::floor),
);

bench!(
    fract,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::fract),
    (unaligned_glam, glam::Vec3::fract),
    (aligned, Vec3A::<f32>::fract),
    (aligned_glam, glam::Vec3A::fract),
    (x4_unaligned, Vec3::<f32x4>::fract),
);

bench!(
    length,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::length),
    (unaligned_glam, glam::Vec3::length),
    (aligned, Vec3A::<f32>::length),
    (aligned_glam, glam::Vec3A::length),
    (x4_unaligned, Vec3::<f32x4>::length),
);

bench!(
    move_towards,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::move_towards),
    (unaligned_glam, glam::Vec3::move_towards),
    (aligned, Vec3A::<f32>::move_towards),
    (aligned_glam, glam::Vec3A::move_towards),
    (x4_unaligned, Vec3::<f32x4>::move_towards),
);

bench!(
    mul,
    ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Mul>::mul),
    (unaligned_glam, <glam::Vec3 as Mul>::mul),
    (aligned, <Vec3A::<f32> as Mul>::mul),
    (aligned_glam, <glam::Vec3A as Mul>::mul),
    (x4_unaligned, <Vec3::<f32x4> as Mul>::mul),
);

bench!(
    neg,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::neg),
    (unaligned_glam, glam::Vec3::neg),
    (aligned, Vec3A::<f32>::neg),
    (aligned_glam, glam::Vec3A::neg),
    (x4_unaligned, Vec3::<f32x4>::neg),
);

bench!(
    normalize,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::normalize),
    (unaligned_glam, glam::Vec3::normalize),
    (aligned, Vec3A::<f32>::normalize),
    (aligned_glam, glam::Vec3A::normalize),
    (x4_unaligned, Vec3::<f32x4>::normalize),
);

bench!(
    project_onto,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::project_onto),
    (unaligned_glam, glam::Vec3::project_onto),
    (aligned, Vec3A::<f32>::project_onto),
    (aligned_glam, glam::Vec3A::project_onto),
    (x4_unaligned, Vec3::<f32x4>::project_onto),
);

bench!(
    reflect,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::reflect),
    (unaligned_glam, glam::Vec3::reflect),
    (aligned, Vec3A::<f32>::reflect),
    (aligned_glam, glam::Vec3A::reflect),
    (x4_unaligned, Vec3::<f32x4>::reflect),
);

bench!(
    refract,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::refract),
    (unaligned_glam, glam::Vec3::refract),
    (aligned, Vec3A::<f32>::refract),
    (aligned_glam, glam::Vec3A::refract),
    (x4_unaligned, Vec3::<f32x4>::refract),
);

bench!(
    reject_from,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::reject_from),
    (unaligned_glam, glam::Vec3::reject_from),
    (aligned, Vec3A::<f32>::reject_from),
    (aligned_glam, glam::Vec3A::reject_from),
    (x4_unaligned, Vec3::<f32x4>::reject_from),
);

bench!(
    rem,
    ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Rem>::rem),
    (unaligned_glam, <glam::Vec3 as Rem>::rem),
    (aligned, <Vec3A::<f32> as Rem>::rem),
    (aligned_glam, <glam::Vec3A as Rem>::rem),
    (x4_unaligned, <Vec3::<f32x4> as Rem>::rem),
);

bench!(
    rotate_towards,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::rotate_towards),
    (unaligned_glam, glam::Vec3::rotate_towards),
    (aligned, Vec3A::<f32>::rotate_towards),
    (aligned_glam, glam::Vec3A::rotate_towards),
    (x4_unaligned, Vec3::<f32x4>::rotate_towards),
);

bench!(
    round,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::round),
    (unaligned_glam, glam::Vec3::round),
    (aligned, Vec3A::<f32>::round),
    (aligned_glam, glam::Vec3A::round),
    (x4_unaligned, Vec3::<f32x4>::round),
);

bench!(
    slerp,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::slerp),
    (unaligned_glam, glam::Vec3::slerp),
    (aligned, Vec3A::<f32>::slerp),
    (aligned_glam, glam::Vec3A::slerp),
    (x4_unaligned, Vec3::<f32x4>::slerp),
);

bench!(
    sub,
    ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Sub>::sub),
    (unaligned_glam, <glam::Vec3 as Sub>::sub),
    (aligned, <Vec3A::<f32> as Sub>::sub),
    (aligned_glam, <glam::Vec3A as Sub>::sub),
    (x4_unaligned, <Vec3::<f32x4> as Sub>::sub),
);

bench!(
    trunc,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::trunc),
    (unaligned_glam, glam::Vec3::trunc),
    (aligned, Vec3A::<f32>::trunc),
    (aligned_glam, glam::Vec3A::trunc),
    (x4_unaligned, Vec3::<f32x4>::trunc),
);

bench!(
    with_max_length,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::with_max_length),
    (unaligned_glam, glam::Vec3::clamp_length_max),
    (aligned, Vec3A::<f32>::with_max_length),
    (aligned_glam, glam::Vec3A::clamp_length_max),
    (x4_unaligned, Vec3::<f32x4>::with_max_length),
);

bench!(
    with_min_length,
    ARRAY_LEN,
    (unaligned, Vec3::<f32>::with_min_length),
    (unaligned_glam, glam::Vec3::clamp_length_min),
    (aligned, Vec3A::<f32>::with_min_length),
    (aligned_glam, glam::Vec3A::clamp_length_min),
    (x4_unaligned, Vec3::<f32x4>::with_min_length),
);
