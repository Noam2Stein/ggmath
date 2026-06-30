use ggmath::{Mask4, Mask4A, Vec4};
use wide::f32x4;

use crate::{ARRAY_LEN, bench};

bench!(
    all,
    ARRAY_LEN,
    (unaligned, Mask4::<f32>::all),
    (unaligned_glam, glam::BVec4::all),
    (aligned, Mask4A::<f32>::all),
    (aligned_glam, glam::BVec4A::all),
    (x4_unaligned, Vec4::<f32x4>::all),
);

bench!(
    any,
    ARRAY_LEN,
    (unaligned, Mask4::<f32>::any),
    (unaligned_glam, glam::BVec4::any),
    (aligned, Mask4A::<f32>::any),
    (aligned_glam, glam::BVec4A::any),
    (x4_unaligned, Vec4::<f32x4>::any),
);

bench!(
    select,
    ARRAY_LEN,
    (unaligned, Mask4::<f32>::select),
    (aligned, Mask4A::<f32>::select),
    (aligned_glam, glam::Vec4::select),
    (x4_unaligned, Vec4::<f32x4>::blend),
);
