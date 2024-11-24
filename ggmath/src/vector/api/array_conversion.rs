use std::mem::{transmute, transmute_copy, MaybeUninit};

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

                *transmute::<_, &mut [T; N]>(&mut output) = array;

                output
            },
            || Vector(array),
        )
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; N] {
        unsafe { transmute_copy(&self) }
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
