use std::mem::{transmute, transmute_copy};

use super::*;

pub trait ScalarVecArrayImpl<const N: usize, S: VecStorage>: ScalarInnerVecs
where
    ScalarCount<N>: VecLen,
{
    fn from_array(array: [Self; N]) -> InnerVector<N, S, Self>;

    fn into_array(vec: InnerVector<N, S, Self>) -> [Self; N];
    fn as_array(vec: &InnerVector<N, S, Self>) -> &[Self; N];
    fn as_array_mut(vec: &mut InnerVector<N, S, Self>) -> &mut [Self; N];
}

impl<const N: usize, S: VecStorage, T: Scalar> Vector<N, S, T>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn from_array(array: [T; N]) -> Self {
        Self {
            inner: T::from_array(array),
        }
    }
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute(array) }
    }
    #[inline(always)]
    pub fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute(array) }
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; N] {
        self.0
    }
    #[inline(always)]
    pub const fn as_array(&self) -> &[T; N] {
        unsafe { transmute(self) }
    }
    #[inline(always)]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute(self) }
    }
}

pub(super) trait VecLenArray<const N: usize>
where
    ScalarCount<N>: VecLen<N>,
{
    fn from_array<S: VecStorage, T: Scalar>(array: [T; N]) -> InnerVector<N, S, T>;

    fn into_array<S: VecStorage, T: Scalar>(vec: InnerVector<N, S, T>) -> [T; N];
    fn as_array<S: VecStorage, T: Scalar>(vec: &InnerVector<N, S, T>) -> &[T; N];
    fn as_array_mut<S: VecStorage, T: Scalar>(vec: &mut InnerVector<N, S, T>) -> &mut [T; N];
}

pub(super) trait VecStorageArray<const N: usize>: VecStorageInnerVecs
where
    ScalarCount<N>: VecLen<N>,
{
    fn from_array<T: Scalar>(array: [T; N]) -> InnerVector<N, Self, T>;

    fn into_array<T: Scalar>(vec: InnerVector<N, Self, T>) -> [T; N];
    fn as_array<T: Scalar>(vec: &InnerVector<N, Self, T>) -> &[T; N];
    fn as_array_mut<T: Scalar>(vec: &mut InnerVector<N, Self, T>) -> &mut [T; N];
}
