use crate::{
    Usize,
    vector::{Scalar, VecAlignment, VecLen, Vector},
};

/// A trait for scalar types that have a `0` value.
///
/// This trait along with `ScalarOne` and `ScalarNegOne`
/// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
pub trait ScalarZero: Scalar {
    /// The zero value of the scalar type.
    const ZERO: Self;
}

/// A trait for scalar types that have a `1` value.
///
/// This trait along with `ScalarZero` and `ScalarNegOne`
/// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
pub trait ScalarOne: Scalar {
    /// The one value of the scalar type.
    const ONE: Self;
}

/// A trait for scalar types that have a `-1` value.
///
/// This trait along with `ScalarZero` and `ScalarOne`
/// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
pub trait ScalarNegOne: Scalar {
    /// The negative one value of the scalar type.
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A vector of all `0`s.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A vector of all `1`s.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A vector of all `-1`s.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}
