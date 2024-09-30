use std::{
    mem::{transmute, transmute_copy},
    ops::{Index, IndexMut, Range},
    slice::SliceIndex,
};

use super::*;

pub trait VecNArray<T: Element, const N: usize>:
    Sized
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
    fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<&I::Output> {
        self.as_array().get(index)
    }
    #[inline(always)]
    fn get_mut<I: SliceIndex<[T]>>(&mut self, index: I) -> Option<&mut I::Output> {
        self.as_array_mut().get_mut(index)
    }
    #[inline(always)]
    unsafe fn get_unchecked<I: SliceIndex<[T]>>(&self, index: I) -> &I::Output {
        self.as_array().get_unchecked(index)
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut<I: SliceIndex<[T]>>(&mut self, index: I) -> &mut I::Output {
        self.as_array_mut().get_unchecked_mut(index)
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
