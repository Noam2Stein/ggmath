use std::mem::MaybeUninit;

use super::*;

#[cfg(feature = "vector")]
impl<T: Scalar> Scalar for MaybeUninit<T> {
    type Vec2Alignment = T::Vec2Alignment;
    type Vec3Alignment = T::Vec3Alignment;
    type Vec4Alignment = T::Vec4Alignment;
}
