use ggmath::{Mask2, Vec2};
use wide::f32x4;

use crate::{ARRAY_LEN, bench};

bench!(
    all,
    ARRAY_LEN,
    (unaligned, Mask2::<f32>::all),
    (unaligned_glam, glam::BVec2::all),
    (x4_unaligned, Vec2::<f32x4>::all),
);

bench!(
    any,
    ARRAY_LEN,
    (unaligned, Mask2::<f32>::any),
    (unaligned_glam, glam::BVec2::any),
    (x4_unaligned, Vec2::<f32x4>::any),
);

bench!(
    select,
    ARRAY_LEN,
    (unaligned, Mask2::<f32>::select),
    (unaligned_glam, glam::Vec2::select),
    (x4_unaligned, Vec2::<f32x4>::blend),
);
