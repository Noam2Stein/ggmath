use crate::{
    Alignment, Length, PrimitiveSigned, PrimitiveSignedBackend, SupportedLength, Vector,
    utils::specialize,
};

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveSigned,
{
    /// Computes `self + rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_checked_add(
            self, rhs
        ))
    }

    /// Computes `self - rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_checked_sub(
            self, rhs
        ))
    }

    /// Computes `self * rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_checked_mul(
            self, rhs
        ))
    }

    /// Computes `self / rhs`, returning `None` if overflow or division
    /// by zero occured.
    #[inline]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_checked_div(
            self, rhs
        ))
    }

    /// Computes `self % rhs`, returning `None` if overflow or division
    /// by zero occurred.
    #[inline]
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_checked_rem(
            self, rhs
        ))
    }

    /// Computes `self + rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_saturating_add(
            self, rhs
        ))
    }

    /// Computes `self - rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_saturating_sub(
            self, rhs
        ))
    }

    /// Computes `self * rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_saturating_mul(
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
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_saturating_div(
            self, rhs
        ))
    }

    /// Computes `self + rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_wrapping_add(
            self, rhs
        ))
    }

    /// Computes `self - rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_wrapping_sub(
            self, rhs
        ))
    }

    /// Computes `self * rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_wrapping_mul(
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
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_wrapping_div(
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
        specialize!(<T as PrimitiveSignedBackend<N, A>>::vector_wrapping_rem(
            self, rhs
        ))
    }
}

macro_rules! impl_signed {
    ($T:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
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
                specialize!(<$T as PrimitiveSignedBackend<N, A>>::vector_max(
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
                specialize!(<$T as PrimitiveSignedBackend<N, A>>::vector_min(
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
                assert!((0..N).all(|i| min[i] <= max[i]), "min <= max");

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
                specialize!(<$T as PrimitiveSignedBackend<N, A>>::vector_max_element(
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
                specialize!(<$T as PrimitiveSignedBackend<N, A>>::vector_min_element(
                    self
                ))
            }

            /// Returns the absolute values of the elements of `self`.
            ///
            /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation) or
            /// overflow checks are enabled:
            ///
            /// Panics if any component is [`T::MIN`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec3;
            /// #
            /// let vector = Vec3::<i32>::new(7, -1, -3);
            /// assert_eq!(vector.abs(), Vec3::new(7, 1, 3));
            /// ```
            ///
            /// [`T::MIN`]: crate::constants::Min::MIN
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn abs(self) -> Self {
                specialize!(<$T as PrimitiveSignedBackend<N, A>>::vector_abs(self))
            }

            /// Returns the signum of the elements of `self`.
            ///
            /// Equivalent to `(self.x.signum(), self.y.signum(), ...)`.
            ///
            /// For each element:
            ///
            /// - `0` if the element is zero
            /// - `1` if the element is positive
            /// - `-1` if the element is negative
            ///
            /// # Examples
            ///
            /// ```
            /// # use ggmath::Vec4;
            /// #
            /// let vector = Vec4::<i32>::new(7, -1, -3, 0);
            /// assert_eq!(vector.signum(), Vec4::new(1, -1, -1, 0));
            /// ```
            #[inline]
            #[must_use]
            pub fn signum(self) -> Self {
                specialize!(<$T as PrimitiveSignedBackend<N, A>>::vector_signum(self))
            }
        }
    };
}
impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);

#[cfg(test)]
mod tests {
    use crate::{
        Vector,
        utils::{assert_panic_eq, for_parameters},
    };

    #[test]
    fn test_max() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).max(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.max(z), y.max(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).max(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.max(z), y.max(w), z.max(y))
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).max(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(x.max(z), y.max(w), z.max(y), w.max(x))
            );
        });
    }

    #[test]
    fn test_min() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).min(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.min(z), y.min(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).min(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.min(z), y.min(w), z.min(y))
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).min(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(x.min(z), y.min(w), z.min(y), w.min(x))
            );
        });
    }

    #[test]
    fn test_clamp() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y)
                    .clamp(Vector::<2, T, A>::new(z, w), Vector::<2, T, A>::new(y, z)),
                Vector::<2, T, A>::new(x.clamp(z, y), y.clamp(w, z))
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).clamp(
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(y, z, x)
                ),
                Vector::<3, T, A>::new(x.clamp(z, y), y.clamp(w, z), z.clamp(y, x))
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).clamp(
                    Vector::<4, T, A>::new(z, w, y, z),
                    Vector::<4, T, A>::new(y, z, x, x)
                ),
                Vector::<4, T, A>::new(x.clamp(z, y), y.clamp(w, z), z.clamp(y, x), w.clamp(z, x))
            );
        });
    }

    #[test]
    fn test_max_element() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(Vector::<2, T, A>::new(x, y).max_element(), x.max(y));
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).max_element(),
                x.max(y).max(z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).max_element(),
                x.max(y).max(z).max(w)
            );
        });
    }

    #[test]
    fn test_min_element() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(Vector::<2, T, A>::new(x, y).min_element(), x.min(y));
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).min_element(),
                x.min(y).min(z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).min_element(),
                x.min(y).min(z).min(w)
            );
        });
    }

    #[test]
    fn test_abs() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y).abs(),
                Vector::<2, T, A>::new(x.abs(), y.abs())
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).abs(),
                Vector::<3, T, A>::new(x.abs(), y.abs(), z.abs())
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).abs(),
                Vector::<4, T, A>::new(x.abs(), y.abs(), z.abs(), w.abs())
            );
        });
    }

    #[test]
    fn test_signum() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).signum(),
                Vector::<2, T, A>::new(x.signum(), y.signum())
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).signum(),
                Vector::<3, T, A>::new(x.signum(), y.signum(), z.signum())
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).signum(),
                Vector::<4, T, A>::new(x.signum(), y.signum(), z.signum(), w.signum())
            );
        });
    }

    #[test]
    fn test_checked_add() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).checked_add(Vector::<2, T, A>::new(z, w)),
                (|| Some(Vector::<2, T, A>::new(x.checked_add(z)?, y.checked_add(w)?)))()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).checked_add(Vector::<3, T, A>::new(z, w, y)),
                (|| Some(Vector::<3, T, A>::new(
                    x.checked_add(z)?,
                    y.checked_add(w)?,
                    z.checked_add(y)?
                )))()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).checked_add(Vector::<4, T, A>::new(z, w, y, x)),
                (|| Some(Vector::<4, T, A>::new(
                    x.checked_add(z)?,
                    y.checked_add(w)?,
                    z.checked_add(y)?,
                    w.checked_add(x)?
                )))()
            );
        });
    }

    #[test]
    fn test_checked_sub() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).checked_sub(Vector::<2, T, A>::new(z, w)),
                (|| Some(Vector::<2, T, A>::new(x.checked_sub(z)?, y.checked_sub(w)?)))()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).checked_sub(Vector::<3, T, A>::new(z, w, y)),
                (|| Some(Vector::<3, T, A>::new(
                    x.checked_sub(z)?,
                    y.checked_sub(w)?,
                    z.checked_sub(y)?
                )))()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).checked_sub(Vector::<4, T, A>::new(z, w, y, x)),
                (|| Some(Vector::<4, T, A>::new(
                    x.checked_sub(z)?,
                    y.checked_sub(w)?,
                    z.checked_sub(y)?,
                    w.checked_sub(x)?
                )))()
            );
        });
    }

    #[test]
    fn test_checked_mul() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).checked_mul(Vector::<2, T, A>::new(z, w)),
                (|| Some(Vector::<2, T, A>::new(x.checked_mul(z)?, y.checked_mul(w)?)))()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).checked_mul(Vector::<3, T, A>::new(z, w, y)),
                (|| Some(Vector::<3, T, A>::new(
                    x.checked_mul(z)?,
                    y.checked_mul(w)?,
                    z.checked_mul(y)?
                )))()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).checked_mul(Vector::<4, T, A>::new(z, w, y, x)),
                (|| Some(Vector::<4, T, A>::new(
                    x.checked_mul(z)?,
                    y.checked_mul(w)?,
                    z.checked_mul(y)?,
                    w.checked_mul(x)?
                )))()
            );
        });
    }

    #[test]
    fn test_checked_div() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).checked_div(Vector::<2, T, A>::new(z, w)),
                (|| Some(Vector::<2, T, A>::new(x.checked_div(z)?, y.checked_div(w)?)))()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).checked_div(Vector::<3, T, A>::new(z, w, y)),
                (|| Some(Vector::<3, T, A>::new(
                    x.checked_div(z)?,
                    y.checked_div(w)?,
                    z.checked_div(y)?
                )))()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).checked_div(Vector::<4, T, A>::new(z, w, y, x)),
                (|| Some(Vector::<4, T, A>::new(
                    x.checked_div(z)?,
                    y.checked_div(w)?,
                    z.checked_div(y)?,
                    w.checked_div(x)?
                )))()
            );
        });
    }

    #[test]
    fn test_checked_rem() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).checked_rem(Vector::<2, T, A>::new(z, w)),
                (|| Some(Vector::<2, T, A>::new(x.checked_rem(z)?, y.checked_rem(w)?)))()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).checked_rem(Vector::<3, T, A>::new(z, w, y)),
                (|| Some(Vector::<3, T, A>::new(
                    x.checked_rem(z)?,
                    y.checked_rem(w)?,
                    z.checked_rem(y)?
                )))()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).checked_rem(Vector::<4, T, A>::new(z, w, y, x)),
                (|| Some(Vector::<4, T, A>::new(
                    x.checked_rem(z)?,
                    y.checked_rem(w)?,
                    z.checked_rem(y)?,
                    w.checked_rem(x)?
                )))()
            );
        });
    }

    #[test]
    fn test_saturating_add() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).saturating_add(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.saturating_add(z), y.saturating_add(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).saturating_add(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(
                    x.saturating_add(z),
                    y.saturating_add(w),
                    z.saturating_add(y)
                )
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w)
                    .saturating_add(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.saturating_add(z),
                    y.saturating_add(w),
                    z.saturating_add(y),
                    w.saturating_add(x)
                )
            );
        });
    }

    #[test]
    fn test_saturating_sub() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).saturating_sub(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.saturating_sub(z), y.saturating_sub(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).saturating_sub(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(
                    x.saturating_sub(z),
                    y.saturating_sub(w),
                    z.saturating_sub(y)
                )
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w)
                    .saturating_sub(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.saturating_sub(z),
                    y.saturating_sub(w),
                    z.saturating_sub(y),
                    w.saturating_sub(x)
                )
            );
        });
    }

    #[test]
    fn test_saturating_mul() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).saturating_mul(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.saturating_mul(z), y.saturating_mul(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).saturating_mul(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(
                    x.saturating_mul(z),
                    y.saturating_mul(w),
                    z.saturating_mul(y)
                )
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w)
                    .saturating_mul(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.saturating_mul(z),
                    y.saturating_mul(w),
                    z.saturating_mul(y),
                    w.saturating_mul(x)
                )
            );
        });
    }

    #[test]
    fn test_saturating_div() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y).saturating_div(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.saturating_div(z), y.saturating_div(w))
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).saturating_div(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(
                    x.saturating_div(z),
                    y.saturating_div(w),
                    z.saturating_div(y)
                )
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w)
                    .saturating_div(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.saturating_div(z),
                    y.saturating_div(w),
                    z.saturating_div(y),
                    w.saturating_div(x)
                )
            );
        });
    }

    #[test]
    fn test_wrapping_add() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).wrapping_add(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.wrapping_add(z), y.wrapping_add(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).wrapping_add(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.wrapping_add(z), y.wrapping_add(w), z.wrapping_add(y))
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).wrapping_add(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.wrapping_add(z),
                    y.wrapping_add(w),
                    z.wrapping_add(y),
                    w.wrapping_add(x)
                )
            );
        });
    }

    #[test]
    fn test_wrapping_sub() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).wrapping_sub(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.wrapping_sub(z), y.wrapping_sub(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).wrapping_sub(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.wrapping_sub(z), y.wrapping_sub(w), z.wrapping_sub(y))
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).wrapping_sub(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.wrapping_sub(z),
                    y.wrapping_sub(w),
                    z.wrapping_sub(y),
                    w.wrapping_sub(x)
                )
            );
        });
    }

    #[test]
    fn test_wrapping_mul() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).wrapping_mul(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.wrapping_mul(z), y.wrapping_mul(w))
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).wrapping_mul(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.wrapping_mul(z), y.wrapping_mul(w), z.wrapping_mul(y))
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).wrapping_mul(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.wrapping_mul(z),
                    y.wrapping_mul(w),
                    z.wrapping_mul(y),
                    w.wrapping_mul(x)
                )
            );
        });
    }

    #[test]
    fn test_wrapping_div() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y).wrapping_div(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.wrapping_div(z), y.wrapping_div(w))
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).wrapping_div(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.wrapping_div(z), y.wrapping_div(w), z.wrapping_div(y))
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).wrapping_div(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.wrapping_div(z),
                    y.wrapping_div(w),
                    z.wrapping_div(y),
                    w.wrapping_div(x)
                )
            );
        });
    }

    #[test]
    fn test_wrapping_rem() {
        for_parameters!(|T: PrimitiveSigned, A, x, y, z| {
            let w = T::max(x, y);

            assert_panic_eq!(
                Vector::<2, T, A>::new(x, y).wrapping_rem(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.wrapping_rem(z), y.wrapping_rem(w))
            );
            assert_panic_eq!(
                Vector::<3, T, A>::new(x, y, z).wrapping_rem(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.wrapping_rem(z), y.wrapping_rem(w), z.wrapping_rem(y))
            );
            assert_panic_eq!(
                Vector::<4, T, A>::new(x, y, z, w).wrapping_rem(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.wrapping_rem(z),
                    y.wrapping_rem(w),
                    z.wrapping_rem(y),
                    w.wrapping_rem(x)
                )
            );
        });
    }
}
