use crate::{
    Alignment, Length, PrimitiveInteger, PrimitiveIntegerBackend, SupportedLength, Vector,
    utils::specialize,
};

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveInteger,
{
    /// Computes `self + rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_checked_add(
            self, rhs
        ))
    }

    /// Computes `self - rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_checked_sub(
            self, rhs
        ))
    }

    /// Computes `self * rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_checked_mul(
            self, rhs
        ))
    }

    /// Computes `self / rhs`, returning `None` if overflow or division
    /// by zero occured.
    #[inline]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_checked_div(
            self, rhs
        ))
    }

    /// Computes `self % rhs`, returning `None` if overflow or division
    /// by zero occurred.
    #[inline]
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_checked_rem(
            self, rhs
        ))
    }

    /// Computes `self + rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_saturating_add(
            self, rhs
        ))
    }

    /// Computes `self - rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_saturating_sub(
            self, rhs
        ))
    }

    /// Computes `self * rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_saturating_mul(
            self, rhs
        ))
    }

    /// Computes `self / rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    ///
    /// # Panics
    ///
    /// Panics if any component of `rhs` is `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn saturating_div(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_saturating_div(
            self, rhs
        ))
    }

    /// Computes `self + rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_wrapping_add(
            self, rhs
        ))
    }

    /// Computes `self - rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_wrapping_sub(
            self, rhs
        ))
    }

    /// Computes `self * rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_wrapping_mul(
            self, rhs
        ))
    }

    /// Computes `self / rhs`, wrapping around at the boundary of the type.
    ///
    /// # Panics
    ///
    /// Panics if any component of `rhs` is `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_wrapping_div(
            self, rhs
        ))
    }

    /// Computes `self % rhs`, wrapping around at the boundary of the type.
    ///
    /// # Panics
    ///
    /// Panics if any component of `rhs` is `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveIntegerBackend<N, A>>::vector_wrapping_rem(
            self, rhs
        ))
    }
}

macro_rules! impl_integer {
    ($T:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            /// A vector with all elements set to [`MIN`].
            ///
            /// [`MIN`]: i32::MIN
            pub const MIN: Self = Self::splat($T::MIN);

            /// A vector with all elements set to [`MAX`].
            ///
            /// [`MAX`]: i32::MAX
            pub const MAX: Self = Self::splat($T::MAX);

            /// Returns the maximum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let a = Vec4::<i32>::new(1, 5, 3, 0);
            /// let b = Vec4::<i32>::new(3, 2, 7, -1);
            /// let max = a.max(b);
            ///
            /// assert_eq!(max, Vec4::new(3, 5, 7, 0));
            /// ```
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                specialize!(<$T as PrimitiveIntegerBackend<N, A>>::vector_max(
                    self, other
                ))
            }

            /// Returns the minimum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let a = Vec4::<i32>::new(1, 5, 3, 0);
            /// let b = Vec4::<i32>::new(3, 2, 7, -1);
            /// let min = a.min(b);
            ///
            /// assert_eq!(min, Vec4::new(1, 2, 3, -1));
            /// ```
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                specialize!(<$T as PrimitiveIntegerBackend<N, A>>::vector_min(
                    self, other
                ))
            }

            /// Clamps the elements of `self` between the elements of `min` and
            /// `max`.
            ///
            /// Equivalent to
            /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
            ///
            /// # Panics
            ///
            /// When debug assertions are enabled:
            ///
            /// Panics if any element of `min` is greater than the corresponding
            /// element of `max`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let vector = Vec4::<i32>::new(1, 2, 3, 0);
            /// let min = Vec4::new(0, 5, 1, -2);
            /// let max = Vec4::new(3, 6, 2, -1);
            /// let clamp = vector.clamp(min, max);
            ///
            /// assert_eq!(clamp, Vec4::new(1, 5, 2, -1));
            /// ```
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                debug_assert!((0..N).all(|i| min[i] <= max[i]), "min <= max");

                self.max(min).min(max)
            }

            /// Returns the maximum between the elements of `self`.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vector = Vec3::<i32>::new(-1, 7, 3);
            /// assert_eq!(vector.max_element(), 7);
            /// ```
            #[inline]
            #[must_use]
            pub fn max_element(self) -> $T {
                specialize!(<$T as PrimitiveIntegerBackend<N, A>>::vector_max_element(
                    self
                ))
            }

            /// Returns the minimum between the elements of `self`.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vector = Vec3::<i32>::new(7, -1, 3);
            /// assert_eq!(vector.min_element(), -1);
            /// ```
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $T {
                specialize!(<$T as PrimitiveIntegerBackend<N, A>>::vector_min_element(
                    self
                ))
            }
        }
    };
}
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);

#[cfg(test)]
mod tests {
    use crate::{
        Vector,
        utils::{assert_panic_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, T: PrimitiveInteger, A| {
            assert_eq!(Vector::<N, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<N, T, A>::MAX, Vector::splat(T::MAX));
        });
    }

    #[test]
    fn test_max() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_eq!(
                    vector_a.max(vector_b),
                    Vector::from_fn(|i| vector_a[i].max(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_min() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_eq!(
                    vector_a.min(vector_b),
                    Vector::from_fn(|i| vector_a[i].min(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector, min, max] in random_iter::<[Vector<N, T, A>; 3]>() {
                if cfg!(debug_assertions) {
                    assert_panic_test_eq!(
                        vector.clamp(min, max),
                        Vector::from_fn(|i| vector[i].clamp(min[i], max[i]))
                    );
                } else {
                    assert_test_eq_or_panic!(
                        vector.clamp(min, max),
                        Vector::from_fn(|i| vector[i].clamp(min[i], max[i]))
                    );
                }
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(vector.max_element(), vector.iter().max().unwrap());
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(vector.min_element(), vector.iter().min().unwrap());
            }
        });
    }

    #[test]
    fn test_checked_add() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[2, 3], [T::MAX - 1, 3], [T::MAX - 1, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a.checked_add(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_add(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MIN + 1, -3], [T::MIN + 1, -1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.checked_add(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_add(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_checked_sub() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MIN + 1, 3], [T::MIN + 1, 1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a.checked_sub(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_sub(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -3], [T::MAX - 1, -1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.checked_sub(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_sub(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_checked_mul() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MAX - 1, 2], [T::MAX, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a.checked_mul(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_mul(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -2], [T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.checked_mul(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_mul(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_checked_div() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a.checked_div(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_div(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.checked_div(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_div(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_checked_rem() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a.checked_rem(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_rem(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.checked_rem(vector_b).unwrap(),
                    Vector::from_fn(|i| vector_a[i].strict_rem(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_saturating_add() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[2, 3], [T::MAX - 1, 3], [T::MAX - 1, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_eq!(
                    vector_a.saturating_add(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_add(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MIN + 1, -3], [T::MIN + 1, -1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_eq!(
                    vector_a.saturating_add(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_add(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_saturating_sub() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MIN + 1, 3], [T::MIN + 1, 1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_eq!(
                    vector_a.saturating_sub(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_sub(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -3], [T::MAX - 1, -1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_eq!(
                    vector_a.saturating_sub(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_sub(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_saturating_mul() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MAX - 1, 2], [T::MAX, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_eq!(
                    vector_a.saturating_mul(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_mul(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -2], [T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_eq!(
                    vector_a.saturating_mul(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_mul(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_saturating_div() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a.saturating_div(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_div(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.saturating_div(vector_b),
                    Vector::from_fn(|i| vector_a[i].saturating_div(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_wrapping_add() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[2, 3], [T::MAX - 1, 3], [T::MAX - 1, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_eq!(
                    vector_a.wrapping_add(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_add(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MIN + 1, -3], [T::MIN + 1, -1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_eq!(
                    vector_a.wrapping_add(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_add(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_wrapping_sub() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MIN + 1, 3], [T::MIN + 1, 1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_eq!(
                    vector_a.wrapping_sub(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_sub(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -3], [T::MAX - 1, -1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_eq!(
                    vector_a.wrapping_sub(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_sub(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_wrapping_mul() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MAX - 1, 2], [T::MAX, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_eq!(
                    vector_a.wrapping_mul(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_mul(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -2], [T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_eq!(
                    vector_a.wrapping_mul(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_mul(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_wrapping_div() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a.wrapping_div(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_div(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.wrapping_div(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_div(vector_b[i]))
                );
            }
        });
    }

    #[test]
    fn test_wrapping_rem() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a.wrapping_rem(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_rem(vector_b[i]))
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a.wrapping_rem(vector_b),
                    Vector::from_fn(|i| vector_a[i].wrapping_rem(vector_b[i]))
                );
            }
        });
    }
}
