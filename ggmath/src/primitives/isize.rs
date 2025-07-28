use super::*;

primitive_aliases! { pub Isize => isize }

#[cfg(target_pointer_width = "32")]
impl Scalar for isize {
    type Vec2Alignment = <i32 as Scalar>::Vec2Alignment;
    type Vec3Alignment = <i32 as Scalar>::Vec3Alignment;
    type Vec4Alignment = <i32 as Scalar>::Vec4Alignment;
}

#[cfg(target_pointer_width = "64")]
impl Scalar for isize {
    type Vec2Alignment = <i64 as Scalar>::Vec2Alignment;
    type Vec3Alignment = <i64 as Scalar>::Vec3Alignment;
    type Vec4Alignment = <i64 as Scalar>::Vec4Alignment;
}

// isize methods are defined using `macro_loop` in i8.rs
