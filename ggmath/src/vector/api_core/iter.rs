use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    /// Creates a vector, where each element T is the returned value from cb using that element’s index.
    #[inline(always)]
    pub fn from_fn(cb: impl FnMut(usize) -> T) -> Self {
        Vector::from_array(std::array::from_fn(cb))
    }

    /// Returns a vector of the same size and alignment as self, with function f applied to each element in order.
    #[inline(always)]
    pub fn map<TOutput: Scalar>(self, f: impl FnMut(T) -> TOutput) -> Vector<N, TOutput, A> {
        Vector::from_array(self.into_array().map(f))
    }
    /// Returns a vector of the same size and alignment as self, with function f applied to each element reference in order.
    #[inline(always)]
    pub fn map_ref<TOutput: Scalar>(&self, f: impl FnMut(&T) -> TOutput) -> Vector<N, TOutput, A> {
        Vector::from_array(self.as_array().each_ref().map(f))
    }

    /// Returns a vector of the same size and alignment as self, with function f applied to each element of `self` and `rhs` in order.
    pub fn map_rhs<TRhs: Scalar, TOutput: Scalar>(
        self,
        rhs: Vector<N, TRhs, impl VecAlignment>,
        mut f: impl FnMut(T, TRhs) -> TOutput,
    ) -> Vector<N, TOutput, A> {
        Vector::from_fn(|i| f(self[i], rhs[i]))
    }
    pub fn map_ref_rhs<TRhs: Scalar, TOutput: Scalar>(
        &self,
        rhs: &Vector<N, TRhs, impl VecAlignment>,
        mut f: impl FnMut(&T, &TRhs) -> TOutput,
    ) -> Vector<N, TOutput, A> {
        Vector::from_fn(|i| f(&self[i], &rhs[i]))
    }

    /// Creates an iterator of references to the vector's elements.
    #[inline(always)]
    pub fn iter_ref(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    /// Creates an iterator of mutable references to the vector's elements.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn fold(self, mut f: impl FnMut(T, T) -> T) -> T {
        let mut output = self[0];

        for i in 1..N {
            output = f(output, self[i]);
        }

        output
    }
}
