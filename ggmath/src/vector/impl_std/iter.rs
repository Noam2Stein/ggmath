use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

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

    fn into_iter(self) -> Self::IntoIter {
        self.as_array_mut().iter_mut()
    }
}

impl<'a, const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn iter_ref(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
