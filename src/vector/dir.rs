use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

/// A trait for [`Scalar`] types that have `0` value.
pub trait ScalarZero: Scalar {
    /// `0`.
    const ZERO: Self;
}

/// A trait for [`Scalar`] types that have `1` value.
pub trait ScalarOne: Scalar {
    /// `1`.
    const ONE: Self;
}

/// A trait for [`Scalar`] types that have `-1` value.
pub trait ScalarNegOne: Scalar {
    /// `-1`.
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero, S: Alignment> Vector<N, T, S>
where
    Length<N>: SupportedLength,
{
    /// A [`Vector`] with all components set to `0`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne, S: Alignment> Vector<N, T, S>
where
    Length<N>: SupportedLength,
{
    /// A [`Vector`] with all components set to `1`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne, S: Alignment> Vector<N, T, S>
where
    Length<N>: SupportedLength,
{
    /// A [`Vector`] with all components set to `-1`.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: ScalarZero + ScalarOne, S: Alignment> Vector<2, T, S> {
    /// `(1, 0)`.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO]);
    /// `(0, 1)`.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, S: Alignment> Vector<3, T, S> {
    /// `(1, 0, 0)`.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);
    /// `(0, 1, 0)`.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);
    /// `(0, 0, 1)`.
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, S: Alignment> Vector<4, T, S> {
    /// `(1, 0, 0, 0)`.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
    /// `(0, 1, 0, 0)`.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
    /// `(0, 0, 1, 0)`.
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
    /// `(0, 0, 0, 1)`.
    pub const W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarNegOne, S: Alignment> Vector<2, T, S> {
    /// `(-1, 0)`.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO]);
    /// `(0, -1)`.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, S: Alignment> Vector<3, T, S> {
    /// `(-1, 0, 0)`.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);
    /// `(0, -1, 0)`.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);
    /// `(0, 0, -1)`.
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, S: Alignment> Vector<4, T, S> {
    /// `(-1, 0, 0, 0)`.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);
    /// `(0, -1, 0, 0)`.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);
    /// `(0, 0, -1, 0)`.
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);
    /// `(0, 0, 0, -1)`.
    pub const NEG_W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE]);
}
