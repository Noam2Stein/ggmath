use wide::{
    i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32,
    u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2, u64x4, u64x8,
};

use crate::{Alignment, Length, SupportedLength, Vector, utils::transmute_generic};

macro_rules! wide_unsigned_impl {
    ($Wide:ident, $SignedWide:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// Returns the bit patterns of `self` reinterpreted as signed
            /// integers of the same size.
            ///
            /// This produces the same result as [`as`] conversions, but ensures
            /// that the bit-width remains the same.
            ///
            /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
            #[inline]
            #[must_use]
            pub const fn cast_signed(self) -> Vector<N, $SignedWide, A> {
                // SAFETY: Both types accept all bit-patterns.
                unsafe { transmute_generic::<Vector<N, $Wide, A>, Vector<N, $SignedWide, A>>(self) }
            }
        }
    };
}
wide_unsigned_impl!(u8x16, i8x16);
wide_unsigned_impl!(u8x32, i8x32);
wide_unsigned_impl!(u16x8, i16x8);
wide_unsigned_impl!(u16x16, i16x16);
wide_unsigned_impl!(u16x32, i16x32);
wide_unsigned_impl!(u32x4, i32x4);
wide_unsigned_impl!(u32x8, i32x8);
wide_unsigned_impl!(u32x16, i32x16);
wide_unsigned_impl!(u64x2, i64x2);
wide_unsigned_impl!(u64x4, i64x4);
wide_unsigned_impl!(u64x8, i64x8);

#[cfg(test)]
mod tests {
    use wide::u32x4;

    use crate::{
        Unaligned, Vector,
        utils::{for_types, random_iter},
    };

    #[test]
    fn test_cast_signed() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, u32x4, Unaligned>>().take(100) {
                assert_eq!(
                    vector.cast_signed(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).cast_signed())
                );
            }
        });
    }
}
