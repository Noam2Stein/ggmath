use std::ops::{Add, Mul};

use newnum::Root;

use super::*;

impl<const N: usize, T: Scalar + Add + Mul + Root, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    /// Creates a vector where all elements have the same given value.
    #[inline(always)]
    pub fn length(self) -> T {
        T::vector_length(self)
    }
}

impl<const N: usize, T: Scalar + Mul, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
    T::Output: Scalar + Add<Output = T::Output>,
{
    /// Creates a vector where all elements have the same given value.
    #[inline(always)]
    pub fn sq_length(self) -> T::Output {
        T::vector_sq_length(self)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_length:

    #[inline(always)]
    fn vector_length<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Self
    where
        ScalarCount<N>: VecLen,
        Self: Mul<Output: Scalar + Add<Output = <Self as Mul>::Output> + Root<Output = Self>>,
    {
        vec.sq_length().sqrt()
    }

    #[inline(always)]
    fn vector_sq_length<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> <Self as Mul>::Output
    where
        ScalarCount<N>: VecLen,
        Self: Mul<Output: Scalar + Add<Output = <Self as Mul>::Output>>,
    {
        vec.map(|c| c * c).csum()
    }
}
