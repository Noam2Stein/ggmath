use super::*;

use std::ops::{Index, IndexMut};

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn from_fn(cb: impl FnMut(usize) -> T) -> Self {
        Vector::from_array(std::array::from_fn(cb))
    }

    #[inline(always)]
    pub fn map<TOutput: Scalar>(self, f: impl FnMut(T) -> TOutput) -> Vector<N, TOutput, A> {
        Vector::from_array(self.into_array().map(f))
    }

    #[inline(always)]
    pub fn iter(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    #[inline(always)]
    pub fn iter_ref(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Index<usize> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array().index(index)
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> IndexMut<usize> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}
impl<'a, const N: usize, T: Scalar, A: VecAlignment> IntoIterator for &'a Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Item = &'a T;
    type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array().iter()
    }
}
impl<'a, const N: usize, T: Scalar, A: VecAlignment> IntoIterator for &'a mut Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array_mut().iter_mut()
    }
}
