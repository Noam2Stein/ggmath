use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Creates a vector, where each element T is the returned value from `cb` using that element’s index.
    #[inline(always)]
    pub fn from_fn(cb: impl FnMut(usize) -> T) -> Self {
        Vector::from_array(std::array::from_fn(cb))
    }

    /// Returns a vector of the same size and alignment as `self`, with function `f` applied to each element in order.
    #[inline(always)]
    pub fn map<TOutput: Scalar>(self, f: impl FnMut(T) -> TOutput) -> Vector<N, TOutput, A> {
        Vector::from_array(self.to_array().map(f))
    }

    /// Returns a vector of the same size and alignment as `self`, with function `f` applied to each element reference in order.
    #[inline(always)]
    pub fn map_ref<TOutput: Scalar>(&self, f: impl FnMut(&T) -> TOutput) -> Vector<N, TOutput, A> {
        Vector::from_array(self.as_array_ref().each_ref().map(f))
    }

    /// Returns a vector of the same size and alignment as `self`, with function `f` applied to each element of `self` and `rhs` in order.
    #[inline(always)]
    pub fn map_rhs<TRhs: Scalar, TOutput: Scalar>(
        self,
        rhs: Vector<N, TRhs, impl VecAlignment>,
        mut f: impl FnMut(T, TRhs) -> TOutput,
    ) -> Vector<N, TOutput, A> {
        Vector::from_fn(|i| f(self[i], rhs[i]))
    }

    /// Returns a vector of the same size and alignment as `self`, with function `f` applied to each element reference of `self` and `rhs` in order.
    #[inline(always)]
    pub fn map_ref_rhs<TRhs: Scalar, TOutput: Scalar>(
        &self,
        rhs: &Vector<N, TRhs, impl VecAlignment>,
        mut f: impl FnMut(&T, &TRhs) -> TOutput,
    ) -> Vector<N, TOutput, A> {
        Vector::from_fn(|i| f(&self[i], &rhs[i]))
    }

    /// Creates an iterator of references to the vector's elements.
    ///
    /// This is named `iter_ref` and not `iter` because scalars are always copy and are usually not used with references.
    #[inline(always)]
    pub fn iter_ref(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    /// Creates an iterator of mutable references to the vector's elements.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    /// Folds the vector into a single value by applying the given function to each element in order.
    #[inline(always)]
    pub fn fold(self, mut f: impl FnMut(T, T) -> T) -> T {
        let mut output = self[0];

        for i in 1..N {
            output = f(output, self[i]);
        }

        output
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.to_array().into_iter()
    }
}

impl<'a, const N: usize, T: Scalar, A: VecAlignment> IntoIterator for &'a Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Item = &'a T;
    type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array_ref().iter()
    }
}

impl<'a, const N: usize, T: Scalar, A: VecAlignment> IntoIterator for &'a mut Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        assert_eq!(Vec2::from_fn(|i| i + 1), vec2!(1, 2));
        assert_eq!(
            vec4!(1, 2, 3, 4).map(|x| (x + 1) as f64),
            vec4!(2.0, 3.0, 4.0, 5.0)
        );
        assert_eq!(
            vec4!(1, 2, 3, 4).map_ref(|x| (*x + 1) as f64),
            vec4!(2.0, 3.0, 4.0, 5.0)
        );
        assert_eq!(
            vec4!(1, 2, 3, 4).map_rhs(vec4!(5, 6, 7, 8), |x, y| x + y),
            vec4!(6, 8, 10, 12)
        );
        assert_eq!(
            vec4!(1, 2, 3, 4).map_ref_rhs(&vec4!(5, 6, 7, 8), |x, y| x + y),
            vec4!(6, 8, 10, 12)
        );

        assert_eq!(vec2!(1, 2).into_iter().collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(vec2!(1, 2).iter_ref().collect::<Vec<_>>(), vec![&1, &2]);
        assert_eq!(
            vec2!(1, 2).iter_mut().collect::<Vec<_>>(),
            vec![&mut 1, &mut 2]
        );
        assert_eq!((&vec2!(1, 2)).into_iter().collect::<Vec<_>>(), vec![&1, &2]);
        assert_eq!(
            (&mut vec2!(1, 2)).into_iter().collect::<Vec<_>>(),
            vec![&mut 1, &mut 2]
        );

        assert_eq!(vec2!(1, 2).fold(|x, y| x + y), 3);
        assert_eq!(vec4!(1, 2, 3, 4).fold(|x, y| (x + y) / 2), 3);
    }
}
