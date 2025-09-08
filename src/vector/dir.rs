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
    pub const ZERO: Self = Self::const_splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A vector of all `1`s.
    pub const ONE: Self = Self::const_splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A vector of all `-1`s.
    pub const NEG_ONE: Self = Self::const_splat(T::NEG_ONE);
}

impl<T: ScalarZero + ScalarOne, A: VecAlignment> Vector<2, T, A> {
    /// A vector that points to the positive x direction.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO]);

    /// A vector that points to the positive y direction.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, A: VecAlignment> Vector<3, T, A> {
    /// A vector that points to the positive x direction.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);

    /// A vector that points to the positive y direction.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);

    /// A vector that points to the positive z direction.
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne, A: VecAlignment> Vector<4, T, A> {
    /// A vector that points to the positive x direction.
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);

    /// A vector that points to the positive y direction.
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);

    /// A vector that points to the positive z direction.
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);

    /// A vector that points to the positive w direction.
    pub const W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Vector<2, T, A> {
    /// A vector that points to the negative x direction.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO]);

    /// A vector that points to the negative y direction.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Vector<3, T, A> {
    /// A vector that points to the negative x direction.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);

    /// A vector that points to the negative y direction.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);

    /// A vector that points to the negative z direction.
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Vector<4, T, A> {
    /// A vector that points to the negative x direction.
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);

    /// A vector that points to the negative y direction.
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);

    /// A vector that points to the negative z direction.
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);

    /// A vector that points to the negative w direction.
    pub const NEG_W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE]);
}
