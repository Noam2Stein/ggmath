use std::mem::transmute;

use super::*;

pub trait ScalarVecArrayImpl<const N: usize, S: VecStorageInnerVecs>: ScalarInnerVecs
where
    ScalarCount<N>: VecLenInnerVec,
{
    fn from_array(array: [Self; N]) -> InnerVector<N, S, Self>;

    fn into_array(vec: InnerVector<N, S, Self>) -> [Self; N];
    fn as_array(vec: &InnerVector<N, S, Self>) -> &[Self; N];
    fn as_array_mut(vec: &mut InnerVector<N, S, Self>) -> &mut [Self; N];
}

impl<const N: usize, S: VecStorage, T: Scalar> Vector<N, S, T>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    pub fn from_array(array: [T; N]) -> Self {
        Self {
            inner: ScalarCount::<N>::from_array(array),
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
    ScalarCount<N>: VecLenInnerVec,
{
    fn from_array<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        array: [T; N],
    ) -> InnerVector<N, S, T>;

    fn into_array<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        vec: InnerVector<N, S, T>,
    ) -> [T; N];
    fn as_array<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        vec: &InnerVector<N, S, T>,
    ) -> &[T; N];
    fn as_array_mut<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        vec: &mut InnerVector<N, S, T>,
    ) -> &mut [T; N];
}
impl<const N: usize> VecLenArray<N> for ScalarCount<N>
where
    ScalarCount<N>: VecLenInnerVec,
{
    fn from_array<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        array: [T; N],
    ) -> InnerVector<N, S, T> {
        S::from_array(array)
    }

    fn into_array<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        vec: InnerVector<N, S, T>,
    ) -> [T; N] {
        S::into_array(vec)
    }
    fn as_array<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        vec: &InnerVector<N, S, T>,
    ) -> &[T; N] {
        S::as_array(vec)
    }
    fn as_array_mut<S: VecStorageArray<N>, T: ScalarVecArrayImpl<N, S>>(
        vec: &mut InnerVector<N, S, T>,
    ) -> &mut [T; N] {
        S::as_array_mut(vec)
    }
}

pub(super) trait VecStorageArray<const N: usize>: VecStorageInnerVecs
where
    ScalarCount<N>: VecLenInnerVec,
{
    fn from_array<T: Scalar>(array: [T; N]) -> InnerVector<N, Self, T>;

    fn into_array<T: Scalar>(vec: InnerVector<N, Self, T>) -> [T; N];
    fn as_array<T: Scalar>(vec: &InnerVector<N, Self, T>) -> &[T; N];
    fn as_array_mut<T: Scalar>(vec: &mut InnerVector<N, Self, T>) -> &mut [T; N];
}
impl VecStorageArray<2> for VecPacked {
    fn from_array<T: ScalarVecArrayImpl<2, Self>>(array: [T; 2]) -> InnerVector<2, Self, T> {
        T::from_array(array)
    }

    fn into_array<T: ScalarVecArrayImpl<2, Self>>(vec: InnerVector<2, Self, T>) -> [T; 2] {
        T::into_array(vec)
    }
    fn as_array<T: ScalarVecArrayImpl<2, Self>>(vec: &InnerVector<2, Self, T>) -> &[T; 2] {
        T::as_array(vec)
    }
    fn as_array_mut<T: ScalarVecArrayImpl<2, Self>>(
        vec: &mut InnerVector<2, Self, T>,
    ) -> &mut [T; 2] {
        T::as_array_mut(vec)
    }
}
