use crate::{Construct, Scalar, Usize};

/// A trait that marks a `Usize<N>` type as a valid vector length.
pub trait VecLen {
    /// The length value as an enum.
    const ENUM: VecLenEnum;

    /// The inner aligned vector type which will be:
    /// - `T::InnerAlignedVec2` for `Usize<2>`
    /// - `T::InnerAlignedVec3` for `Usize<3>`
    /// - `T::InnerAlignedVec4` for `Usize<4>`
    type InnerAlignedVector<T: Scalar>: Construct;
}

/// An enum with all currently supported vector lengths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecLenEnum {
    /// `2`
    Two,
    /// `3`
    Three,
    /// `4`
    Four,
}

impl VecLen for Usize<2> {
    const ENUM: VecLenEnum = VecLenEnum::Two;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec2;
}
impl VecLen for Usize<3> {
    const ENUM: VecLenEnum = VecLenEnum::Three;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec3;
}
impl VecLen for Usize<4> {
    const ENUM: VecLenEnum = VecLenEnum::Four;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec4;
}
