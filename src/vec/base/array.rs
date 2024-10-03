use std::{
    mem::{transmute, transmute_copy},
    ops::{Index, IndexMut, Range},
    slice::SliceIndex,
};

use super::*;

pub trait VecNArray<T: Element, const N: usize>:
    Sized
    + VecNWithT<N>
    + Index<usize, Output = T>
    + IndexMut<usize>
    + Index<Range<usize>, Output = [T]>
    + IndexMut<Range<usize>>
{
    fn from_array(value: [T; N]) -> Self;
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

    #[inline(always)]
    fn iter(self) -> std::array::IntoIter<T, N> {
        self.into_array().into_iter()
    }
    #[inline(always)]
    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.as_array_mut().iter_mut()
    }
    #[inline(always)]
    fn map<OutputT: Element>(self, f: impl FnMut(T) -> OutputT) -> Self::WithT<OutputT> {
        <Self::WithT<OutputT>>::from_array(self.as_array().map(f))
    }
    #[inline(always)]
    fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
        Iterator::count(self.iter().filter(|c| f(*c)))
    }
    #[inline(always)]
    fn any(self, f: impl FnMut(T) -> bool) -> bool {
        Iterator::any(&mut self.iter(), f)
    }
    #[inline(always)]
    fn all(self, f: impl FnMut(T) -> bool) -> bool {
        Iterator::all(&mut self.iter(), f)
    }
}

impl<T: Element> VecNArray<T, 2> for Vec2<T> {
    #[inline(always)]
    fn from_array(value: [T; 2]) -> Self {
        unsafe { transmute_copy(&value) }
    }
}
impl<T: Element> VecNArray<T, 3> for Vec3<T> {
    #[inline(always)]
    fn from_array(value: [T; 3]) -> Self {
        unsafe { transmute_copy(&[value[0], value[1], value[2], T::default()]) }
    }
}
impl<T: Element> VecNArray<T, 4> for Vec4<T> {
    #[inline(always)]
    fn from_array(value: [T; 4]) -> Self {
        unsafe { transmute_copy(&value) }
    }
}

impl<T: Element, Idx: SliceIndex<[T]>> Index<Idx> for Vec2<T> {
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        self.as_array().index(index)
    }
}
impl<T: Element, Idx: SliceIndex<[T]>> IndexMut<Idx> for Vec2<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}
impl<T: Element, Idx: SliceIndex<[T]>> Index<Idx> for Vec3<T> {
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        self.as_array().index(index)
    }
}
impl<T: Element, Idx: SliceIndex<[T]>> IndexMut<Idx> for Vec3<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}
impl<T: Element, Idx: SliceIndex<[T]>> Index<Idx> for Vec4<T> {
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        self.as_array().index(index)
    }
}
impl<T: Element, Idx: SliceIndex<[T]>> IndexMut<Idx> for Vec4<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}
