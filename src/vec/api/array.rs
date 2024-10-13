use std::mem::{transmute, transmute_copy};

use gomath_proc_macros::vec_api;

use super::*;

vec_api!(
    Array:

    fn from_array(array: [T; N]) -> Self;

    #[inline(always)]
    fn into_array(self) -> [T; N] {
        unsafe { transmute_copy(&self) }
    }
    #[inline(always)]
    fn as_array(&self) -> &[T; N] {
        unsafe { transmute(self) }
    }
    #[inline(always)]
    fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute(self) }
    }
);
