use std::ops::{Add, Div, Mul, Rem, Sub};

use ggmath::{Vec3, Vec3A};
use wide::f32x4;

use crate::{MICROBENCH_ARRAY_LEN, bench};

bench!(
    add,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Add>::add),
    (unaligned_glam, <glam::Vec3 as Add>::add),
    (aligned, <Vec3A::<f32> as Add>::add),
    (aligned_glam, <glam::Vec3A as Add>::add),
    (x4_unaligned, <Vec3::<f32x4> as Add>::add),
);

bench!(
    sub,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Sub>::sub),
    (unaligned_glam, <glam::Vec3 as Sub>::sub),
    (aligned, <Vec3A::<f32> as Sub>::sub),
    (aligned_glam, <glam::Vec3A as Sub>::sub),
    (x4_unaligned, <Vec3::<f32x4> as Sub>::sub),
);

bench!(
    mul,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Mul>::mul),
    (unaligned_glam, <glam::Vec3 as Mul>::mul),
    (aligned, <Vec3A::<f32> as Mul>::mul),
    (aligned_glam, <glam::Vec3A as Mul>::mul),
    (x4_unaligned, <Vec3::<f32x4> as Mul>::mul),
);

bench!(
    div,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Div>::div),
    (unaligned_glam, <glam::Vec3 as Div>::div),
    (aligned, <Vec3A::<f32> as Div>::div),
    (aligned_glam, <glam::Vec3A as Div>::div),
    (x4_unaligned, <Vec3::<f32x4> as Div>::div),
);

bench!(
    rem,
    MICROBENCH_ARRAY_LEN,
    (unaligned, <Vec3::<f32> as Rem>::rem),
    (unaligned_glam, <glam::Vec3 as Rem>::rem),
    (aligned, <Vec3A::<f32> as Rem>::rem),
    (aligned_glam, <glam::Vec3A as Rem>::rem),
    (x4_unaligned, <Vec3::<f32x4> as Rem>::rem),
);

bench!(
    length,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::length),
    (unaligned_glam, glam::Vec3::length),
    (aligned, Vec3A::<f32>::length),
    (aligned_glam, glam::Vec3A::length),
    (x4_unaligned, Vec3::<f32x4>::length),
);

bench!(
    normalize,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::normalize),
    (unaligned_glam, glam::Vec3::normalize),
    (aligned, Vec3A::<f32>::normalize),
    (aligned_glam, glam::Vec3A::normalize),
    (x4_unaligned, Vec3::<f32x4>::normalize),
);

bench!(
    dot,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::dot),
    (unaligned_glam, glam::Vec3::dot),
    (aligned, Vec3A::<f32>::dot),
    (aligned_glam, glam::Vec3A::dot),
    (x4_unaligned, Vec3::<f32x4>::dot),
);

bench!(
    cross,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::cross),
    (unaligned_glam, glam::Vec3::cross),
    (aligned, Vec3A::<f32>::cross),
    (aligned_glam, glam::Vec3A::cross),
    (x4_unaligned, Vec3::<f32x4>::cross),
);

bench!(
    angle_between,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::angle_between),
    (unaligned_glam, glam::Vec3::angle_between),
    (aligned, Vec3A::<f32>::angle_between),
    (aligned_glam, glam::Vec3A::angle_between),
    (x4_unaligned, Vec3::<f32x4>::angle_between),
);

bench!(
    any_orthogonal_vector,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::any_orthogonal_vector),
    (unaligned_glam, glam::Vec3::any_orthogonal_vector),
    (aligned, Vec3A::<f32>::any_orthogonal_vector),
    (aligned_glam, glam::Vec3A::any_orthogonal_vector),
    (x4_unaligned, Vec3::<f32x4>::any_orthogonal_vector),
);

bench!(
    any_orthonormal_vector,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::any_orthonormal_vector),
    (unaligned_glam, glam::Vec3::any_orthonormal_vector),
    (aligned, Vec3A::<f32>::any_orthonormal_vector),
    (aligned_glam, glam::Vec3A::any_orthonormal_vector),
    (x4_unaligned, Vec3::<f32x4>::any_orthonormal_vector),
);

bench!(
    any_orthonormal_pair,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::any_orthonormal_pair),
    (unaligned_glam, glam::Vec3::any_orthonormal_pair),
    (aligned, Vec3A::<f32>::any_orthonormal_pair),
    (aligned_glam, glam::Vec3A::any_orthonormal_pair),
    (x4_unaligned, Vec3::<f32x4>::any_orthonormal_pair),
);

bench!(
    project_onto,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::project_onto),
    (unaligned_glam, glam::Vec3::project_onto),
    (aligned, Vec3A::<f32>::project_onto),
    (aligned_glam, glam::Vec3A::project_onto),
    (x4_unaligned, Vec3::<f32x4>::project_onto),
);

bench!(
    reject_from,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::reject_from),
    (unaligned_glam, glam::Vec3::reject_from),
    (aligned, Vec3A::<f32>::reject_from),
    (aligned_glam, glam::Vec3A::reject_from),
    (x4_unaligned, Vec3::<f32x4>::reject_from),
);

bench!(
    reflect,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::reflect),
    (unaligned_glam, glam::Vec3::reflect),
    (aligned, Vec3A::<f32>::reflect),
    (aligned_glam, glam::Vec3A::reflect),
    (x4_unaligned, Vec3::<f32x4>::reflect),
);

bench!(
    refract,
    MICROBENCH_ARRAY_LEN,
    (unaligned, Vec3::<f32>::refract),
    (unaligned_glam, glam::Vec3::refract),
    (aligned, Vec3A::<f32>::refract),
    (aligned_glam, glam::Vec3A::refract),
    (x4_unaligned, Vec3::<f32x4>::refract),
);
