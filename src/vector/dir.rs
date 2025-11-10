use crate::{Scalar, SupportedVecLen, VecLen, Vector};

pub trait ScalarZero: Scalar {
    const ZERO: Self;
}

pub trait ScalarOne: Scalar {
    const ONE: Self;
}

pub trait ScalarNegOne: Scalar {
    const NEG_ONE: Self;
}

impl<const N: usize, T: ScalarZero> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T: ScalarOne> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T: ScalarNegOne> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: ScalarZero + ScalarOne> Vector<2, T> {
    pub const X: Self = Self::from_array([T::ONE, T::ZERO]);
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne> Vector<3, T> {
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarOne> Vector<4, T> {
    pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
    pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
    pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
    pub const W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: ScalarZero + ScalarNegOne> Vector<2, T> {
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO]);
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne> Vector<3, T> {
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO]);
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO]);
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE]);
}

impl<T: ScalarZero + ScalarNegOne> Vector<4, T> {
    pub const NEG_X: Self = Self::from_array([T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO]);
    pub const NEG_Y: Self = Self::from_array([T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO]);
    pub const NEG_Z: Self = Self::from_array([T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO]);
    pub const NEG_W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE]);
}
