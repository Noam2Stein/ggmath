use core::mem::MaybeUninit;

use crate::{
    Scalar, Simd, SimdBehaviour, SupportedVecLen, VecLen, Vector, vector::SoundVectorRepr,
};

impl<T: Scalar> Scalar for MaybeUninit<T> {}

impl<const N: usize, T: Scalar> SimdBehaviour<N> for MaybeUninit<T>
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = MaybeUninit<Vector<N, T, Simd>>;
}

// SAFETY: MaybeUninit<Vector<N, T, Simd>> begins with N T's, so it also begins with N MaybeUninit<T>'s
unsafe impl<const N: usize, T: Scalar> SoundVectorRepr<N, MaybeUninit<T>>
    for MaybeUninit<Vector<N, T, Simd>>
where
    VecLen<N>: SupportedVecLen,
{
}
