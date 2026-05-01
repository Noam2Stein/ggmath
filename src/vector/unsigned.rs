use crate::{
    Alignment, Length, PrimitiveUnsignedBackend, SupportedLength, Vector, utils::specialize,
};

macro_rules! impl_unsigned {
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
            /// let a = Vec4::<u32>::new(1, 5, 3, 0);
            /// let b = Vec4::<u32>::new(3, 2, 7, 1);
            /// let max = a.max(b);
            ///
            /// assert_eq!(max, Vec4::new(3, 5, 7, 1));
            /// ```
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_max(
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
            /// let a = Vec4::<u32>::new(1, 5, 3, 0);
            /// let b = Vec4::<u32>::new(3, 2, 7, 1);
            /// let min = a.min(b);
            ///
            /// assert_eq!(min, Vec4::new(1, 2, 3, 0));
            /// ```
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_min(
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
            /// let vector = Vec4::<u32>::new(1, 2, 3, 9);
            /// let min = Vec4::new(0, 5, 1, 2);
            /// let max = Vec4::new(3, 6, 2, 3);
            /// let clamp = vector.clamp(min, max);
            ///
            /// assert_eq!(clamp, Vec4::new(1, 5, 2, 3));
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
            /// let vector = Vec3::<u32>::new(1, 7, 3);
            /// assert_eq!(vector.max_element(), 7);
            /// ```
            #[inline]
            #[must_use]
            pub fn max_element(self) -> $T {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_max_element(
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
            /// let vector = Vec3::<u32>::new(7, 0, 3);
            /// assert_eq!(vector.min_element(), 0);
            /// ```
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $T {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_min_element(
                    self
                ))
            }

            /// Computes `self + rhs`, returning `None` if overflow occured.
            #[inline]
            #[must_use]
            pub fn checked_add(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_checked_add(
                    self, rhs
                ))
            }

            /// Computes `self - rhs`, returning `None` if overflow occured.
            #[inline]
            #[must_use]
            pub fn checked_sub(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_checked_sub(
                    self, rhs
                ))
            }

            /// Computes `self * rhs`, returning `None` if overflow occured.
            #[inline]
            #[must_use]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_checked_mul(
                    self, rhs
                ))
            }

            /// Computes `self / rhs`, returning `None` if division by zero occured.
            #[inline]
            #[must_use]
            pub fn checked_div(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_checked_div(
                    self, rhs
                ))
            }

            /// Computes `self % rhs`, returning `None` if division by zero occurred.
            #[inline]
            #[must_use]
            pub fn checked_rem(self, rhs: Self) -> Option<Self> {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_checked_rem(
                    self, rhs
                ))
            }

            /// Computes `self + rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_add(self, rhs: Self) -> Self {
                specialize!(
                    <$T as PrimitiveUnsignedBackend<N, A>>::vector_saturating_add(self, rhs)
                )
            }

            /// Computes `self - rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_sub(self, rhs: Self) -> Self {
                specialize!(
                    <$T as PrimitiveUnsignedBackend<N, A>>::vector_saturating_sub(self, rhs)
                )
            }

            /// Computes `self * rhs`, saturating at the numeric bounds instead of
            /// overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_mul(self, rhs: Self) -> Self {
                specialize!(
                    <$T as PrimitiveUnsignedBackend<N, A>>::vector_saturating_mul(self, rhs)
                )
            }

            /// Computes `self + rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub fn wrapping_add(self, rhs: Self) -> Self {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_wrapping_add(
                    self, rhs
                ))
            }

            /// Computes `self - rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub fn wrapping_sub(self, rhs: Self) -> Self {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_wrapping_sub(
                    self, rhs
                ))
            }

            /// Computes `self * rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub fn wrapping_mul(self, rhs: Self) -> Self {
                specialize!(<$T as PrimitiveUnsignedBackend<N, A>>::vector_wrapping_mul(
                    self, rhs
                ))
            }
        }
    };
}
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
impl_unsigned!(usize);

#[cfg(test)]
mod tests {
    use crate::{
        Vector,
        utils::{assert_panic_eq, for_parameters},
    };

    #[test]
    fn test_max() {
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
    fn test_checked_add() {
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
    fn test_wrapping_add() {
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
        for_parameters!(|T: PrimitiveUnsigned, A, x, y, z| {
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
}
