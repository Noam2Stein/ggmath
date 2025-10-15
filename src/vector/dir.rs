use crate::{Scalar, Simdness, SupportedVecLen, VecLen, Vector};

/// A trait for scalars that have a `0` value.
pub trait ScalarZero: Scalar {
    /// `0`.
    const ZERO: Self;
}

/// A trait for scalars that have a `1` value.
pub trait ScalarOne: Scalar {
    /// `1`.
    const ONE: Self;
}

/// A trait for scalars that have a `-1` value.
pub trait ScalarNegOne: Scalar {
    /// `-1`.
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero, S: Simdness> Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    /// Vector with all elements set to `0`.
    pub const ZERO: Self = Self::const_from_array([T::ZERO; N]);
}

impl<const N: usize, T: ScalarOne, S: Simdness> Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    /// Vector with all elements set to `1`.
    pub const ONE: Self = Self::const_from_array([T::ONE; N]);
}

impl<const N: usize, T: ScalarNegOne, S: Simdness> Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    /// Vector with all elements set to `-1`.
    pub const NEG_ONE: Self = Self::const_from_array([T::NEG_ONE; N]);
}

impl<T: ScalarZero + ScalarOne, S: Simdness> Vector<2, T, S> {
    /// Vector that points in the positive x direction.
    pub const X: Self = Self::const_from_array([T::ONE, T::ZERO]);
    /// Vector that points in the positive y direction.
    pub const Y: Self = Self::const_from_array([T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, S: Simdness> Vector<3, T, S> {
    /// Vector that points in the positive x direction.
    pub const X: Self = Self::const_from_array([T::ONE, T::ZERO, T::ZERO]);
    /// Vector that points in the positive y direction.
    pub const Y: Self = Self::const_from_array([T::ZERO, T::ONE, T::ZERO]);
    /// Vector that points in the positive z direction.
    pub const Z: Self = Self::const_from_array([T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, S: Simdness> Vector<4, T, S> {
    /// Vector that points in the positive x direction.
    pub const X: Self = Self::const_from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
    /// Vector that points in the positive y direction.
    pub const Y: Self = Self::const_from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
    /// Vector that points in the positive z direction.
    pub const Z: Self = Self::const_from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
    /// Vector that points in the positive w direction.
    pub const W: Self = Self::const_from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarNegOne, S: Simdness> Vector<2, T, S> {
    /// Vector that points in the negative x direction.
    pub const NEG_X: Self = Self::const_from_array([T::NEG_ONE, T::ZERO]);
    /// Vector that points in the negative y direction.
    pub const NEG_Y: Self = Self::const_from_array([T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, S: Simdness> Vector<3, T, S> {
    /// Vector that points in the negative x direction.
    pub const NEG_X: Self = Self::const_from_array([T::NEG_ONE, T::ZERO, T::ZERO]);
    /// Vector that points in the negative y direction.
    pub const NEG_Y: Self = Self::const_from_array([T::ZERO, T::NEG_ONE, T::ZERO]);
    /// Vector that points in the negative z direction.
    pub const NEG_Z: Self = Self::const_from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, S: Simdness> Vector<4, T, S> {
    /// Vector that points in the negative x direction.
    pub const NEG_X: Self = Self::const_from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);
    /// Vector that points in the negative y direction.
    pub const NEG_Y: Self = Self::const_from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);
    /// Vector that points in the negative z direction.
    pub const NEG_Z: Self = Self::const_from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);
    /// Vector that points in the negative w direction.
    pub const NEG_W: Self = Self::const_from_array([T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE]);
}
