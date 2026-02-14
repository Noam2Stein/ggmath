use core::ops::{Deref, DerefMut};

use crate::{
    Affine, Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    utils::{transmute_mut, transmute_ref},
};

#[repr(C)]
pub struct AffineDeref<const N: usize, T, A: Alignment>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// The scaling, rotation and shear part of the transform.
    pub matrix: Matrix<N, T, A>,
    /// The translation of the transform.
    pub translation: Vector<N, T, A>,
}

impl<const N: usize, T, A: Alignment> Deref for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Target = AffineDeref<N, T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Affine<N, T, A>` is guaranteed to begin with
        // `Matrix<N, T, A>` followed by `Vector<N, T, A>`, and so begin with
        // `AffineDeref<N, T, A>`.
        unsafe { transmute_ref::<Affine<N, T, A>, AffineDeref<N, T, A>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> DerefMut for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Affine<N, T, A>` is guaranteed to begin with
        // `Matrix<N, T, A>` followed by `Vector<N, T, A>`, and so begin with
        // `AffineDeref<N, T, A>`.
        unsafe { transmute_mut::<Affine<N, T, A>, AffineDeref<N, T, A>>(self) }
    }
}
