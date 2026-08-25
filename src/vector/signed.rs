use crate::{
    Alignment, Length, Mask, PrimitiveSigned, SupportedLength, Vector,
    backend::SignedVectorBackend,
    utils::{specialize, transmute_generic},
};

type Unsigned<T> = <T as PrimitiveSigned>::Unsigned;

/// Contains signed-integer vector items that cannot be part of the generic
/// implementation, because their names conflict with float vector items.
///
/// This will be solved once the type system allows disjoint trait
/// implementations.
macro_rules! conflicting_items {
    ($T:ident) => {
        /// Returns the absolute values of the elements of `self`.
        ///
        /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
        ///
        /// # Panics
        ///
        /// When debug assertions or overflow checks are enabled:
        ///
        /// Panics if any component is [`MIN`].
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
        /// [`MIN`]: i32::MIN
        #[inline]
        #[must_use]
        #[track_caller]
        pub fn abs(self) -> Self {
            debug_assert!(
                self.ne_mask(Self::MIN).all(),
                "cannot negate MIN: {self:?}.abs()"
            );

            specialize!(<$T as SignedVectorBackend<N, A>>::vector_wrapping_abs(self))
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
            specialize!(<$T as SignedVectorBackend<N, A>>::vector_signum(self))
        }
    };
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveSigned,
{
    // When generating documentation, Rust does not care that these items are
    // conflicting. This allows us to cheat by showing these items in a generic
    // context in documentation, but making them separate in all other cases.
    //
    // This makes documentation way easier to read, so its worth it even if its
    // slightly lying (the fact that these functions are not available in
    // generic contexts is mentioned in the docs for `PrimitiveSigned`).
    #[cfg(doc)]
    conflicting_items!(T);

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is positive, and `false` if it is zero or negative.
    ///
    /// Equivalent to `(self.x.is_positive(), self.y.is_positive(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask4, Vec4};
    /// #
    /// let vector = Vec4::new(1, -2, -3, 4);
    /// let mask = vector.positive_mask();
    ///
    /// assert_eq!(mask, Mask4::new(true, false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn positive_mask(self) -> Mask<N, T, A> {
        specialize!(<T as SignedVectorBackend<N, A>>::vector_positive_mask(self))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is negative, and `false` if it is zero or positive.
    ///
    /// Equivalent to `(self.x.is_negative(), self.y.is_negative(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask4, Vec4};
    /// #
    /// let vector = Vec4::new(1, -2, -3, 4);
    /// let mask = vector.negative_mask();
    ///
    /// assert_eq!(mask, Mask4::new(false, true, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn negative_mask(self) -> Mask<N, T, A> {
        specialize!(<T as SignedVectorBackend<N, A>>::vector_negative_mask(self))
    }

    /// Returns the bit patterns of `self` reinterpreted as unsigned integers of
    /// the same size.
    ///
    /// This produces the same result as [`as`] conversions, but ensures that
    /// the bit-width remains the same.
    ///
    /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
    #[inline]
    #[must_use]
    pub const fn cast_unsigned(self) -> Vector<N, Unsigned<T>, A> {
        if const { size_of::<Vector<N, T, A>>() == size_of::<Vector<N, Unsigned<T>, A>>() } {
            // SAFETY: Both types accept all bit-patterns.
            unsafe { transmute_generic::<Vector<N, T, A>, Vector<N, Unsigned<T>, A>>(self) }
        } else {
            // SAFETY: Both types accept all bit-patterns.
            Vector::from_array(unsafe {
                transmute_generic::<[T; N], [Unsigned<T>; N]>(self.to_array())
            })
        }
    }
}

macro_rules! impl_conflicting_items {
    ($T:ident) => {
        // See the `cfg(doc)` usage of `conflicting_items`.
        #[cfg(not(doc))]
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            conflicting_items!($T);
        }
    };
}
impl_conflicting_items!(i8);
impl_conflicting_items!(i16);
impl_conflicting_items!(i32);
impl_conflicting_items!(i64);
impl_conflicting_items!(i128);
impl_conflicting_items!(isize);

#[cfg(test)]
mod tests {
    use crate::{
        Mask, Vec3A, Vector,
        test_utils::{assert_panic_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_positive_mask() {
        for_types!(|N, T: PrimitiveSigned, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(
                    vector.positive_mask(),
                    Mask::from_fn(|i| vector[i].is_positive())
                );
            }
        });
    }

    #[test]
    fn test_negative_mask() {
        for_types!(|N, T: PrimitiveSigned, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(
                    vector.negative_mask(),
                    Mask::from_fn(|i| vector[i].is_negative())
                );
            }
        });
    }

    #[test]
    fn test_cast_unsigned() {
        for_types!(|T: PrimitiveSigned| {
            let vector = Vec3A::<T>::new(1, -1, T::MAX);
            assert_eq!(vector.cast_unsigned(), vector.map(T::cast_unsigned));
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N, T: PrimitiveSigned, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_panic_test_eq!(vector.abs(), vector.map(T::abs));
            }
        });
    }

    #[test]
    fn test_signum() {
        for_types!(|N, T: PrimitiveSigned, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_panic_test_eq!(vector.signum(), vector.map(T::signum));
            }
        });
    }
}
