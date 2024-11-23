use std::mem::{transmute, MaybeUninit};

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn from_array(array: [T; N]) -> Self {
        Self::from_resolved_alignment_fns(
            || unsafe {
                let mut output = MaybeUninit::uninit().assume_init();

                *transmute(&mut output) = array;

                output
            },
            || Self(array),
        )
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; N] {
        unsafe { *transmute(&self) }
    }

    #[inline(always)]
    pub fn as_array(&self) -> &[T; N] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute(self) }
    }
}
