use std::cmp::Ordering;

use super::*;

impl Scalar for Ordering {
    type Vec2Alignment = Align<1>;
    type Vec3Alignment = Align<1>;
    type Vec4Alignment = Align<1>;
}

impl<const N: usize, A: VecAlignment> Vector<N, Ordering, A>
where
    Usize<N>: VecLen,
{
    /// Vector of all `Less` values.
    pub const LESS: Self = Self::from_array([Ordering::Less; N]);
    /// Vector of all `Equal` values.
    pub const EQUAL: Self = Self::from_array([Ordering::Equal; N]);
    /// Vector of all `Greater` values.
    pub const GREATER: Self = Self::from_array([Ordering::Greater; N]);
}
