use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
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
    MaybeVecLen<N>: VecLen,
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
    MaybeVecLen<N>: VecLen,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array_mut().iter_mut()
    }
}
