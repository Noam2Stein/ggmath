use wide::{
    i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32,
    u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2, u64x4, u64x8,
};

use crate::{
    Alignment, Length, SupportedLength, Vector,
    utils::{specialize, transmute_generic},
};

macro_rules! wide_signed_impl {
    ($Wide:ident, $UnsignedWide:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// Returns a vector mask where each element is `true` if the
            /// corresponding element of `self` is positive, and `false` if it
            /// is zero or negative.
            ///
            /// Equivalent to
            /// `(self.x.is_positive(), self.y.is_positive(), ...)`.
            #[inline]
            #[must_use]
            pub fn positive_mask(self) -> Self {
                specialize!(Vector::<N, $Wide, A>::positive_mask_backend(self))
            }

            /// Returns a vector mask where each element is `true` if the
            /// corresponding element of `self` is negative, and `false` if it
            /// is zero or positive.
            ///
            /// Equivalent to
            /// `(self.x.is_negative(), self.y.is_negative(), ...)`.
            #[inline]
            #[must_use]
            pub fn negative_mask(self) -> Self {
                specialize!(Vector::<N, $Wide, A>::negative_mask_backend(self))
            }

            /// Returns the bit patterns of `self` reinterpreted as unsigned
            /// integers of the same size.
            ///
            /// This produces the same result as [`as`] conversions, but ensures
            /// that the bit-width remains the same.
            ///
            /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
            #[inline]
            #[must_use]
            pub const fn cast_unsigned(self) -> Vector<N, $UnsignedWide, A> {
                // SAFETY: Both types accept all bit-patterns.
                unsafe {
                    transmute_generic::<Vector<N, $Wide, A>, Vector<N, $UnsignedWide, A>>(self)
                }
            }

            /// Returns the absolute values of the elements of `self`.
            ///
            /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
            #[inline]
            #[must_use]
            pub fn abs(self) -> Self {
                specialize!(Vector::<N, $Wide, A>::abs_backend(self))
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
            #[inline]
            #[must_use]
            pub fn signum(self) -> Self {
                specialize!(Vector::<N, $Wide, A>::signum_backend(self))
            }
        }

        impl<A: Alignment> Vector<2, $Wide, A> {
            #[inline(always)]
            fn positive_mask_backend(self) -> Self {
                Self::new(self.x.is_positive(), self.y.is_positive())
            }

            #[inline(always)]
            fn negative_mask_backend(self) -> Self {
                Self::new(self.x.is_negative(), self.y.is_negative())
            }

            #[inline(always)]
            fn abs_backend(self) -> Self {
                Self::new(self.x.abs(), self.y.abs())
            }

            #[inline(always)]
            fn signum_backend(self) -> Self {
                Self::new(self.x.signum(), self.y.signum())
            }
        }

        impl<A: Alignment> Vector<3, $Wide, A> {
            #[inline(always)]
            fn positive_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_positive(),
                    self.y.is_positive(),
                    self.z.is_positive(),
                )
            }

            #[inline(always)]
            fn negative_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_negative(),
                    self.y.is_negative(),
                    self.z.is_negative(),
                )
            }

            #[inline(always)]
            fn abs_backend(self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs())
            }

            #[inline(always)]
            fn signum_backend(self) -> Self {
                Self::new(self.x.signum(), self.y.signum(), self.z.signum())
            }
        }

        impl<A: Alignment> Vector<4, $Wide, A> {
            #[inline(always)]
            fn positive_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_positive(),
                    self.y.is_positive(),
                    self.z.is_positive(),
                    self.w.is_positive(),
                )
            }

            #[inline(always)]
            fn negative_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_negative(),
                    self.y.is_negative(),
                    self.z.is_negative(),
                    self.w.is_negative(),
                )
            }

            #[inline(always)]
            fn abs_backend(self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
            }

            #[inline(always)]
            fn signum_backend(self) -> Self {
                Self::new(
                    self.x.signum(),
                    self.y.signum(),
                    self.z.signum(),
                    self.w.signum(),
                )
            }
        }
    };
}
wide_signed_impl!(i8x16, u8x16);
wide_signed_impl!(i8x32, u8x32);
wide_signed_impl!(i16x8, u16x8);
wide_signed_impl!(i16x16, u16x16);
wide_signed_impl!(i16x32, u16x32);
wide_signed_impl!(i32x4, u32x4);
wide_signed_impl!(i32x8, u32x8);
wide_signed_impl!(i32x16, u32x16);
wide_signed_impl!(i64x2, u64x2);
wide_signed_impl!(i64x4, u64x4);
wide_signed_impl!(i64x8, u64x8);

#[cfg(test)]
mod tests {
    use wide::i32x4;

    use crate::{
        Unaligned, Vector,
        test_utils::{for_types, random_iter},
    };

    #[test]
    fn test_positive_mask() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>() {
                assert_eq!(vector.positive_mask(), vector.map(i32x4::is_positive));
            }
        });
    }

    #[test]
    fn test_negative_mask() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>() {
                assert_eq!(vector.negative_mask(), vector.map(i32x4::is_negative));
            }
        });
    }

    #[test]
    fn test_cast_unsigned() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>().take(100) {
                assert_eq!(
                    vector.cast_unsigned(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).cast_unsigned())
                );
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>().take(100) {
                assert_eq!(
                    vector.abs(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).abs())
                );
            }
        });
    }

    #[test]
    fn test_signum() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>().take(100) {
                assert_eq!(
                    vector.signum(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).signum())
                );
            }
        });
    }
}
