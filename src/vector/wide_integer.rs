use wide::{
    i8x16, i8x32, i8x64, i16x8, i16x16, i16x32, i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16,
    u8x32, u8x64, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2, u64x4, u64x8,
};

use crate::{Alignment, Length, SupportedLength, Vector, utils::specialize};

macro_rules! items {
    ($Wide:ident) => {
        /// A vector with all elements set to [`MIN`].
        ///
        /// [`MIN`]: i32::MIN
        pub const MIN: Self = Self::splat($Wide::MIN);

        /// A vector with all elements set to [`MAX`].
        ///
        /// [`MAX`]: i32::MAX
        pub const MAX: Self = Self::splat($Wide::MAX);

        /// Computes `self + rhs`, saturating at the numeric bounds instead of
        /// overflowing.
        #[inline]
        #[must_use]
        pub fn saturating_add(self, rhs: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::saturating_add_backend(self, rhs))
        }

        /// Computes `self - rhs`, saturating at the numeric bounds instead of
        /// overflowing.
        #[inline]
        #[must_use]
        pub fn saturating_sub(self, rhs: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::saturating_sub_backend(self, rhs))
        }

        /// Computes `self * rhs`, saturating at the numeric bounds instead of
        /// overflowing.
        #[inline]
        #[must_use]
        pub fn saturating_mul(self, rhs: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::saturating_mul_backend(self, rhs))
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
            specialize!(Vector::<N, $Wide, A>::saturating_div_backend(self, rhs))
        }

        /// Returns the maximum elements between `self` and `other`.
        ///
        /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
        #[inline]
        #[must_use]
        pub fn max(self, other: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::max_backend(self, other))
        }

        /// Returns the minimum elements between `self` and `other`.
        ///
        /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
        #[inline]
        #[must_use]
        pub fn min(self, other: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::min_backend(self, other))
        }

        /// Clamps the elements of `self` between the elements of `min` and
        /// `max`.
        ///
        /// Equivalent to
        /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
        ///
        /// If `min > max`, the result is unspecified. Consider manually
        /// checking for that case.
        #[inline]
        #[must_use]
        pub fn clamp(self, min: Self, max: Self) -> Self {
            self.max(min).min(max)
        }

        /// Returns the maximum between the elements of `self`.
        ///
        /// Equivalent to `self.x.max(self.y).max(self.z)...`.
        #[inline]
        #[must_use]
        pub fn max_element(self) -> $Wide {
            specialize!(Vector::<N, $Wide, A>::max_element_backend(self))
        }

        /// Returns the minimum between the elements of `self`.
        ///
        /// Equivalent to `self.x.min(self.y).min(self.z)...`.
        #[inline]
        #[must_use]
        pub fn min_element(self) -> $Wide {
            specialize!(Vector::<N, $Wide, A>::min_element_backend(self))
        }
    };
}

// Since all wide-integer functions have names that conflict with normal integer
// functions, We cannot implement this API using generics. Duplicating the API
// for each supported wide-integer type works, but then documentation shows the
// duplicated API, making it hard to read.
//
// When generating documentation, Rust does not care that these items are
// conflicting. This allows us to cheat by showing these items in a generic
// context in documentation, but making them separate in all other cases.

#[cfg(doc)]
#[doc(hidden)]
pub trait WideInteger: crate::Scalar {}

/// Functionality for [SoA] (Structure of Arrays) integer vectors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all integer types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<const N: usize, Wide, A: Alignment> Vector<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideInteger,
{
    items!(Wide);
}

macro_rules! impl_items {
    ($Wide:ident) => {
        #[cfg(not(doc))]
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            items!($Wide);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Vector<2, $Wide, A> {
            #[inline(always)]
            fn saturating_add_backend(self, rhs: Self) -> Self {
                Self::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
            }

            #[inline(always)]
            fn saturating_sub_backend(self, rhs: Self) -> Self {
                Self::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
            }

            #[inline(always)]
            fn saturating_mul_backend(self, rhs: Self) -> Self {
                Self::new(self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y))
            }

            #[inline(always)]
            fn saturating_div_backend(self, rhs: Self) -> Self {
                Self::new(self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y))
            }

            #[inline(always)]
            fn max_backend(self, other: Self) -> Self {
                Self::new(self.x.max(other.x), self.y.max(other.y))
            }

            #[inline(always)]
            fn min_backend(self, other: Self) -> Self {
                Self::new(self.x.min(other.x), self.y.min(other.y))
            }

            #[inline(always)]
            fn max_element_backend(self) -> $Wide {
                self.x.max(self.y)
            }

            #[inline(always)]
            fn min_element_backend(self) -> $Wide {
                self.x.min(self.y)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Vector<3, $Wide, A> {
            #[inline(always)]
            fn saturating_add_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_add(rhs.x),
                    self.y.saturating_add(rhs.y),
                    self.z.saturating_add(rhs.z),
                )
            }

            #[inline(always)]
            fn saturating_sub_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_sub(rhs.x),
                    self.y.saturating_sub(rhs.y),
                    self.z.saturating_sub(rhs.z),
                )
            }

            #[inline(always)]
            fn saturating_mul_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_mul(rhs.x),
                    self.y.saturating_mul(rhs.y),
                    self.z.saturating_mul(rhs.z),
                )
            }

            #[inline(always)]
            fn saturating_div_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_div(rhs.x),
                    self.y.saturating_div(rhs.y),
                    self.z.saturating_div(rhs.z),
                )
            }

            #[inline(always)]
            fn max_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.max(other.x),
                    self.y.max(other.y),
                    self.z.max(other.z),
                )
            }

            #[inline(always)]
            fn min_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.min(other.x),
                    self.y.min(other.y),
                    self.z.min(other.z),
                )
            }

            #[inline(always)]
            fn max_element_backend(self) -> $Wide {
                self.x.max(self.y).max(self.z)
            }

            #[inline(always)]
            fn min_element_backend(self) -> $Wide {
                self.x.min(self.y).min(self.z)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Vector<4, $Wide, A> {
            #[inline(always)]
            fn saturating_add_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_add(rhs.x),
                    self.y.saturating_add(rhs.y),
                    self.z.saturating_add(rhs.z),
                    self.w.saturating_add(rhs.w),
                )
            }

            #[inline(always)]
            fn saturating_sub_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_sub(rhs.x),
                    self.y.saturating_sub(rhs.y),
                    self.z.saturating_sub(rhs.z),
                    self.w.saturating_sub(rhs.w),
                )
            }

            #[inline(always)]
            fn saturating_mul_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_mul(rhs.x),
                    self.y.saturating_mul(rhs.y),
                    self.z.saturating_mul(rhs.z),
                    self.w.saturating_mul(rhs.w),
                )
            }

            #[inline(always)]
            fn saturating_div_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.saturating_div(rhs.x),
                    self.y.saturating_div(rhs.y),
                    self.z.saturating_div(rhs.z),
                    self.w.saturating_div(rhs.w),
                )
            }

            #[inline(always)]
            fn max_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.max(other.x),
                    self.y.max(other.y),
                    self.z.max(other.z),
                    self.w.max(other.w),
                )
            }

            #[inline(always)]
            fn min_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.min(other.x),
                    self.y.min(other.y),
                    self.z.min(other.z),
                    self.w.min(other.w),
                )
            }

            #[inline(always)]
            fn max_element_backend(self) -> $Wide {
                self.x.max(self.y).max(self.z).max(self.w)
            }

            #[inline(always)]
            fn min_element_backend(self) -> $Wide {
                self.x.min(self.y).min(self.z).min(self.w)
            }
        }
    };
}
impl_items!(i8x16);
impl_items!(i8x32);
impl_items!(i8x64);
impl_items!(i16x8);
impl_items!(i16x16);
impl_items!(i16x32);
impl_items!(i32x4);
impl_items!(i32x8);
impl_items!(i32x16);
impl_items!(i64x2);
impl_items!(i64x4);
impl_items!(i64x8);
impl_items!(u8x16);
impl_items!(u8x32);
impl_items!(u8x64);
impl_items!(u16x8);
impl_items!(u16x16);
impl_items!(u16x32);
impl_items!(u32x4);
impl_items!(u32x8);
impl_items!(u32x16);
impl_items!(u64x2);
impl_items!(u64x4);
impl_items!(u64x8);

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::i32x4;

    use crate::{
        Unaligned, Vector,
        test_utils::{assert_panic_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, Wide: WideInteger| {
            assert_eq!(Vector::<N, Wide, Unaligned>::MIN, Vector::splat(Wide::MIN));
            assert_eq!(Vector::<N, Wide, Unaligned>::MAX, Vector::splat(Wide::MAX));
        });
    }

    #[test]
    fn test_saturating_add() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, i32x4, Unaligned>; 2]>().take(100) {
                assert_eq!(
                    a.saturating_add(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).saturating_add(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_saturating_sub() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, i32x4, Unaligned>; 2]>().take(100) {
                assert_eq!(
                    a.saturating_sub(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).saturating_sub(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_saturating_mul() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, i32x4, Unaligned>; 2]>().take(100) {
                assert_eq!(
                    a.saturating_mul(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).saturating_mul(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_saturating_div() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, i32x4, Unaligned>; 2]>().take(100) {
                assert_panic_test_eq!(
                    a.saturating_div(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).saturating_div(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_max() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, i32x4, Unaligned>; 2]>().take(100) {
                assert_eq!(
                    a.max(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).max(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_min() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, i32x4, Unaligned>; 2]>().take(100) {
                assert_eq!(
                    a.min(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).min(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_types!(|N| {
            for [vector, min, max] in random_iter::<[Vector<N, i32x4, Unaligned>; 3]>().take(100) {
                assert_test_eq_or_panic!(
                    vector.clamp(min, max),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .clamp(min.lane(lane), max.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>().take(100) {
                assert_eq!(
                    vector.max_element(),
                    i32x4::new(std::array::from_fn(|lane| vector.lane(lane).max_element()))
                );
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, i32x4, Unaligned>>().take(100) {
                assert_eq!(
                    vector.min_element(),
                    i32x4::new(std::array::from_fn(|lane| vector.lane(lane).min_element()))
                );
            }
        });
    }
}
