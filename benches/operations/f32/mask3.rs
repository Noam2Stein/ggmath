use ggmath::{Mask3, Mask3A, Vec3};
use wide::f32x4;

use crate::{ARRAY_LEN, bench};

bench!(
    all,
    ARRAY_LEN,
    (unaligned, Mask3::<f32>::all),
    (unaligned_glam, glam::BVec3::all),
    (aligned, Mask3A::<f32>::all),
    (aligned_glam, glam::BVec3A::all),
    (x4_unaligned, Vec3::<f32x4>::all),
);

bench!(
    any,
    ARRAY_LEN,
    (unaligned, Mask3::<f32>::any),
    (unaligned_glam, glam::BVec3::any),
    (aligned, Mask3A::<f32>::any),
    (aligned_glam, glam::BVec3A::any),
    (x4_unaligned, Vec3::<f32x4>::any),
);

bench!(
    select,
    ARRAY_LEN,
    (unaligned, Mask3::<f32>::select),
    (unaligned_glam, glam::Vec3::select),
    (aligned, Mask3A::<f32>::select),
    (aligned_glam, glam::Vec3A::select),
    (x4_unaligned, Vec3::<f32x4>::select::<f32x4>),
);
