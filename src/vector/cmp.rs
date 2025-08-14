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

#[cfg(test)]
mod tests {
    use super::*;

    repetitive! {
        @for
            float in ['f32, 'f64],
            N in 2..=4,
            A in ['VecAligned, 'VecPacked],
            A2 in ['VecAligned, 'VecPacked],
        {
            @let Vec = @{ Vector::<@N, @float, @A> };
            @let Vec2 = @{ Vector::<@N, @float, @A2> };
            @let BVec = @{ Vector::<@N, bool, @A> };
            @let BVec2 = @{ Vector::<@N, bool, @A2> };

            @let f = match float {
                'f32 => 'f,
                'f64 => 'd,
            };

            @let a = match A {
                'VecAligned => 'a,
                'VecPacked => 'p,
            };

            @let a2 = match A2 {
                'VecAligned => 'a,
                'VecPacked => 'p,
            };

            @let vec = @[f 'vec N a];

            @let values_0 = @{[@for val in [1.0, 2.0, 3.0, 4.0][0..N] { @val, }]};
            @let values_1 = @{[@for val in [5.0, 6.0, 7.0, 8.0][0..N] { @val, }]};
            @let values_2 = @{[@for val in [9.0, 10.0, 11.0, 12.0][0..N] { @val, }]};

            #[test]
            fn @['test_ vec '_ a2 '_cmp]() {
                let a = @Vec::from_array(@values_0);
                let b = @Vec2::from_array(@values_1);
                let c = @Vec2::from_array(@values_2);

                assert_eq!(a.with_padding(0.0), a.with_padding(1.0));
                assert_ne!(a.with_padding(2.0), b.with_padding(2.0));
                assert_ne!(a, b.with_y(a.y()));

                assert_eq!(a.eq_mask(a), @BVec::splat(true));
                assert_eq!(a.eq_mask(b), @BVec::splat(false));
                assert_eq!(a.eq_mask(b.with_y(a.y())), @BVec::splat(false).with_y(true));

                assert_eq!(a.ne_mask(a), @BVec::splat(false));
                assert_eq!(a.ne_mask(b), @BVec::splat(true));
                assert_eq!(a.ne_mask(b.with_y(a.y())), @BVec::splat(true).with_y(false));

                assert_eq!(a.lt_mask(a), @BVec::splat(false));
                assert_eq!(a.lt_mask(b), @BVec::splat(true));
                assert_eq!(a.lt_mask(b.with_y(a.y())), @BVec::splat(true).with_y(false));

                assert_eq!(a.gt_mask(a), @BVec::splat(false));
                assert_eq!(b.gt_mask(a), @BVec::splat(true));
                assert_eq!(b.gt_mask(a.with_y(b.y())), @BVec::splat(true).with_y(false));

                assert_eq!(a.le_mask(a), @BVec::splat(true));
                assert_eq!(a.le_mask(b), @BVec::splat(true));
                assert_eq!(b.le_mask(a), @BVec::splat(false));
                assert_eq!(a.le_mask(b.with_y(a.y())), @BVec::splat(true));

                assert_eq!(a.ge_mask(a), @BVec::splat(true));
                assert_eq!(a.ge_mask(b), @BVec::splat(false));
                assert_eq!(b.ge_mask(a), @BVec::splat(true));
                assert_eq!(a.ge_mask(b.with_y(a.y())), @BVec::splat(false).with_y(true));

                assert_eq!(a.min(a), a);
                assert_eq!(a.min(b), a);
                assert_eq!(b.min(a), a);
                assert_eq!(b.min(a.with_y(b.y())), a.with_y(b.y()));

                assert_eq!(a.max(a), a);
                assert_eq!(a.max(b), b);
                assert_eq!(b.max(a), b);
                assert_eq!(a.max(b.with_y(a.y())), b.with_y(a.y()));

                assert_eq!(b.clamp(a, c), b);
                assert_eq!(a.clamp(a, c), a);
                assert_eq!(c.clamp(a, c), c);
                assert_eq!(a.clamp(b, c), b);
                assert_eq!(c.clamp(a, b), b);
                assert_eq!(a.with_y(100.0).clamp(b, c), b.with_y(c.y()));
                assert_eq!(c.with_y(-100.0).clamp(a, b), b.with_y(a.y()));
            }
        }
    }
}
