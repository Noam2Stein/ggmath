use crate::{Scalar, SupportedVecLen, VecLen, Vector};

/// A trait for scalar types that have a `0` value
pub trait ScalarZero: Scalar {
    /// `0`
    const ZERO: Self;
}

/// A trait for scalar types that have a `1` value
pub trait ScalarOne: Scalar {
    /// `1`
    const ONE: Self;
}

/// A trait for scalar types that have a `-1` value
pub trait ScalarNegOne: Scalar {
    /// `-1`
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    /// A vector with all elements set to `0`
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    /// A vector with all elements set to `1`
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    /// A vector with all elements set to `-1`
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: ScalarZero + ScalarOne> Vector<2, T> {
    /// `(1, 0)`
    pub const X: Self = Self::from_array([T::ONE, T::ZERO]);

    /// `(0, 1)`
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne> Vector<3, T> {
    /// `(1, 0, 0)`
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);

    /// `(0, 1, 0)`
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);

    /// `(0, 0, 1)`
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne> Vector<4, T> {
    /// `(1, 0, 0, 0)`
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);

    /// `(0, 1, 0, 0)`
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);

    /// `(0, 0, 1, 0)`
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);

    /// `(0, 0, 0, 1)`
    pub const W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarNegOne> Vector<2, T> {
    /// `(-1, 0)`
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO]);

    /// `(0, -1)`
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne> Vector<3, T> {
    /// `(-1, 0, 0)`
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);

    /// `(0, -1, 0)`
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);

    /// `(0, 0, -1)`
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne> Vector<4, T> {
    /// `(-1, 0, 0, 0)`
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);

    /// `(0, -1, 0, 0)`
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);

    /// `(0, 0, -1, 0)`
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);

    /// `(0, 0, 0, -1)`
    pub const NEG_W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE]);
}
