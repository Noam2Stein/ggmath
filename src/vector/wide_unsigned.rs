use wide::{
    i16x16, i16x32, i16x8, i32x16, i32x4, i32x8, i64x2, i64x4, i64x8, i8x16, i8x32, i8x64, u16x16,
    u16x32, u16x8, u32x16, u32x4, u32x8, u64x2, u64x4, u64x8, u8x16, u8x32, u8x64,
};

use crate::{utils::transmute_generic, Alignment, Length, SupportedLength, Vector};

macro_rules! items {
    ($Wide:ident, $SignedWide:ty) => {
        /// Returns the bit patterns of `self` reinterpreted as signed integers
        /// of the same size.
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
    };
}

// Since all wide-unsigned-integer functions have names that conflict with
// normal unsigned-integer functions, We cannot implement this API using
// generics. Duplicating the API for each supported wide-unsigned-integer type
// works, but then documentation shows the duplicated API, making it hard to
// read.
//
// When generating documentation, Rust does not care that these items are
// conflicting. This allows us to cheat by showing these items in a generic
// context in documentation, but making them separate in all other cases.

#[cfg(doc)]
#[doc(hidden)]
pub trait WideUnsigned: crate::Scalar {
    type Signed: crate::Scalar;
}

/// Functionality for [SoA] (Structure of Arrays) unsigned-integer vectors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all unsigned-integer types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<const N: usize, Wide, A: Alignment> Vector<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideUnsigned,
{
    items!(Wide, <Wide as WideUnsigned>::Signed);
}

macro_rules! impl_items {
    ($Wide:ident, $SignedWide:ident) => {
        #[cfg(not(doc))]
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            items!($Wide, $SignedWide);
        }
    };
}
impl_items!(u8x16, i8x16);
impl_items!(u8x32, i8x32);
impl_items!(u8x64, i8x64);
impl_items!(u16x8, i16x8);
impl_items!(u16x16, i16x16);
impl_items!(u16x32, i16x32);
impl_items!(u32x4, i32x4);
impl_items!(u32x8, i32x8);
impl_items!(u32x16, i32x16);
impl_items!(u64x2, i64x2);
impl_items!(u64x4, i64x4);
impl_items!(u64x8, i64x8);

#[cfg(test)]
mod tests {
    use wide::u32x4;

    use crate::{
        test_utils::{for_types, random_iter},
        Unaligned, Vector,
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
