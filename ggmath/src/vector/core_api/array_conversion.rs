use std::mem::{transmute, transmute_copy, MaybeUninit};

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    /// Creates a ```Vector<N, T, A>``` from a ```[T; N]``` by copying it.
    #[inline(always)]
    pub fn from_array(array: [T; N]) -> Self {
        Self::from_resolved_alignment_fns(
            || unsafe {
                let mut output = MaybeUninit::uninit().assume_init();

                *transmute::<_, &mut [T; N]>(&mut output) = array;

                output
            },
            || Vector(array),
        )
    }

    /// Creates a ```[T; N]``` from a ```Vector<N, T, A>``` by copying it.
    #[inline(always)]
    pub fn into_array(self) -> [T; N] {
        unsafe { transmute_copy(&self) }
    }

    /// referecnes ```self``` as an array.
    /// - Cost: Nothing.
    #[inline(always)]
    pub fn as_array(&self) -> &[T; N] {
        unsafe { transmute(self) }
    }

    /// mutably referecnes ```self``` as an array.
    /// - Cost: Nothing.
    #[inline(always)]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute(self) }
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
where
    ScalarCount<N>: VecLen,
{
    /// Converts an array reference to a vector reference.
    ///
    /// This is only possible with ```VecPacked``` alignment
    /// because it guarentees the same type-layout as an array,
    /// where as a ```VecAligned``` vector might have a larger size than an array.
    #[inline(always)]
    pub fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute(array) }
    }

    /// Converts a mutable array reference to a mutable vector reference.
    ///
    /// This is only possible with ```VecPacked``` alignment
    /// because it guarentees the same type-layout as an array,
    /// where as a ```VecAligned``` vector might have a larger size than an array.
    #[inline(always)]
    pub fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute(array) }
    }
}
