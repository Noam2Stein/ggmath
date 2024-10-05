use std::{
    mem::{transmute, transmute_copy},
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use super::*;

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    pub fn from_array(value: [T; N]) -> Self {
        unsafe { transmute_copy(&value) }
    }
    #[inline(always)]
    pub fn into_array(self) -> [T; N] {
        *unsafe { transmute::<&Self, &[T; N]>(&self) }
    }
    #[inline(always)]
    pub fn as_array(&self) -> &[T; N] {
        unsafe { transmute(self) }
    }
    #[inline(always)]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn iter(self) -> std::array::IntoIter<T, N> {
        self.into_array().into_iter()
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.as_array_mut().iter_mut()
    }
    #[inline(always)]
    pub fn map<T2: Element>(self, f: impl FnMut(T) -> T2) -> VecN<N, T2> {
        VecN::from_array(self.as_array().map(f))
    }
    #[inline(always)]
    pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
        Iterator::count(self.iter().filter(|c| f(*c)))
    }
    #[inline(always)]
    pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
        Iterator::any(&mut self.iter(), f)
    }
    #[inline(always)]
    pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
        Iterator::all(&mut self.iter(), f)
    }
}
impl<const N: usize, T: Element, Idx: SliceIndex<[T]>> Index<Idx> for VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    type Output = Idx::Output;
    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        self.as_array().index(index)
    }
}
impl<const N: usize, T: Element, Idx: SliceIndex<[T]>> IndexMut<Idx> for VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}
