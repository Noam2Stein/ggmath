use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
    T: PartialEq<T2>,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_eq(*self, *other)
    }

    #[inline(always)]
    fn ne(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_ne(*self, *other)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns a vector of booleans where each element is true if the corresponding elements in the two vectors are equal.
    #[inline(always)]
    pub fn eq_mask<T2: Scalar, A2: VecAlignment>(
        self,
        other: Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_eq_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding elements in the two vectors are not equal.
    #[inline(always)]
    pub fn ne_mask<T2: Scalar, A2: VecAlignment>(
        self,
        other: Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_ne_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is less than the other vector.
    #[inline(always)]
    pub fn lt_mask<T2: Scalar, A2: VecAlignment>(
        self,
        other: Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_lt_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is greater than the other vector.
    #[inline(always)]
    pub fn gt_mask<T2: Scalar, A2: VecAlignment>(
        self,
        other: Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_gt_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is less than or equal to the other vector.
    #[inline(always)]
    pub fn le_mask<T2: Scalar, A2: VecAlignment>(
        self,
        other: Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_le_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is greater than or equal to the other vector.
    #[inline(always)]
    pub fn ge_mask<T2: Scalar, A2: VecAlignment>(
        self,
        other: Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_ge_mask(self, other)
    }

    /// Returns a vector of the minimum of each two corresponding elements from the input vectors.
    ///
    /// Basically `[self.x.min(other.x), self.y.min(other.y), ...]`.
    #[inline(always)]
    pub fn min(self, other: Vector<N, T, impl VecAlignment>) -> Vector<N, T, A>
    where
        T: PartialOrd,
    {
        T::vec_min(self, other)
    }

    /// Returns a vector of the maximum of each two corresponding elements from the input vectors.
    ///
    /// Basically `[self.x.max(other.x), self.y.max(other.y), ...]`.
    #[inline(always)]
    pub fn max(self, other: Vector<N, T, impl VecAlignment>) -> Vector<N, T, A>
    where
        T: PartialOrd,
    {
        T::vec_max(self, other)
    }

    /// Returns a vector of the elements clamped to the range `[min, max]`.
    ///
    /// Basically `[self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...]`.
    #[inline(always)]
    pub fn clamp(
        self,
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Vector<N, T, A>
    where
        T: PartialOrd,
    {
        T::vec_clamp(self, min, max)
    }
}
