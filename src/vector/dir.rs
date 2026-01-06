use crate::vector::{Alignment, Length, Scalar, SupportedLength, Vector};

/// A trait for [`Scalar`] types that have a `0` value.
pub trait ScalarZero: Scalar {
    /// `0`.
    const ZERO: Self;
}

/// A trait for [`Scalar`] types that have a `1` value.
pub trait ScalarOne: Scalar {
    /// `1`.
    const ONE: Self;
}

/// A trait for [`Scalar`] types that have a `-1` value.
pub trait ScalarNegOne: Scalar {
    /// `-1`.
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// All `0`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// All `1`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// All `-1`.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: ScalarZero + ScalarOne, A: Alignment> Vector<2, T, A> {
    /// `(1, 0)`.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO]);
    /// `(0, 1)`.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, A: Alignment> Vector<3, T, A> {
    /// `(1, 0, 0)`.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);
    /// `(0, 1, 0)`.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);
    /// `(0, 0, 1)`.
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, A: Alignment> Vector<4, T, A> {
    /// `(1, 0, 0, 0)`.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
    /// `(0, 1, 0, 0)`.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
    /// `(0, 0, 1, 0)`.
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
    /// `(0, 0, 0, 1)`.
    pub const W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarNegOne, A: Alignment> Vector<2, T, A> {
    /// `(-1, 0)`.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO]);
    /// `(0, -1)`.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, A: Alignment> Vector<3, T, A> {
    /// `(-1, 0, 0)`.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);
    /// `(0, -1, 0)`.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);
    /// `(0, 0, -1)`.
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, A: Alignment> Vector<4, T, A> {
    /// `(-1, 0, 0, 0)`.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);
    /// `(0, -1, 0, 0)`.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);
    /// `(0, 0, -1, 0)`.
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);
    /// `(0, 0, 0, -1)`.
    pub const NEG_W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE]);
}
