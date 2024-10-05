use std::{
    mem::{transmute, transmute_copy},
    ops::{Index, IndexMut, Range},
    slice::SliceIndex,
};

use super::*;

impl<N: VecNum, T: Element> VecN<N, T> {
    #[inline(always)]
    fn from_array(value: N::Array<T>) -> Self {}
    #[inline(always)]
    fn into_array(self) -> N::Array<T> {
        unsafe { transmute_copy(&self) }
    }
    #[inline(always)]
    fn as_array(&self) -> &N::Array<T> {
        unsafe { transmute(self) }
    }
    #[inline(always)]
    fn as_array_mut(&mut self) -> &mut N::Array<T> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    fn iter(self) -> <N::Array<T> as IntoIterator>::IntoIter {
        self.into_array().into_iter()
    }
    #[inline(always)]
    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.as_array_mut().iter_mut()
    }
    #[inline(always)]
    fn map<T2: Element>(self, f: impl FnMut(T) -> T2) -> VecN<N, T2> {
        <Self::WithT<T2>>::from_array(self.as_array().map(f))
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

impl<N: VecNum, T: Element, Idx: SliceIndex<[T]>> Index<Idx> for Vec2<T> {
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
