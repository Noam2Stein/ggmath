use wide::{
    i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32,
    u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2, u64x4, u64x8,
};

use crate::{Alignment, Length, SupportedLength, Vector};

macro_rules! wide_integer_impl {
    ($Wide:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// A vector with all elements set to [`MIN`].
            ///
            /// [`MIN`]: i32::MIN
            pub const MIN: Self = Self::splat($Wide::MIN);

            /// A vector with all elements set to [`MAX`].
            ///
            /// [`MAX`]: i32::MAX
            pub const MAX: Self = Self::splat($Wide::MAX);

            /// Computes `self + rhs`, saturating at the numeric bounds instead
            /// of overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_add(self, rhs: Self) -> Self {
                Self::from_fn(|i| self[i].saturating_add(rhs[i]))
            }

            /// Computes `self - rhs`, saturating at the numeric bounds instead
            /// of overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self::from_fn(|i| self[i].saturating_sub(rhs[i]))
            }

            /// Computes `self * rhs`, saturating at the numeric bounds instead
            /// of overflowing.
            #[inline]
            #[must_use]
            pub fn saturating_mul(self, rhs: Self) -> Self {
                Self::from_fn(|i| self[i].saturating_mul(rhs[i]))
            }

            /// Computes `self / rhs`, saturating at the numeric bounds instead
            /// of overflowing.
            ///
            /// # Panics
            ///
            /// Panics if any component of `rhs` is `0`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn saturating_div(self, rhs: Self) -> Self {
                Self::from_fn(|i| self[i].saturating_div(rhs[i]))
            }

            /// Returns the maximum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].max(other[i]))
            }

            /// Returns the minimum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].min(other[i]))
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
                match N {
                    2 => self[0].max(self[1]),
                    3 => self[0].max(self[1]).max(self[2]),
                    4 => self[0].max(self[1]).max(self[2]).max(self[3]),
                    _ => unreachable!(),
                }
            }

            /// Returns the minimum between the elements of `self`.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...`.
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $Wide {
                match N {
                    2 => self[0].min(self[1]),
                    3 => self[0].min(self[1]).min(self[2]),
                    4 => self[0].min(self[1]).min(self[2]).min(self[3]),
                    _ => unreachable!(),
                }
            }
        }
    };
}
wide_integer_impl!(i8x16);
wide_integer_impl!(i8x32);
wide_integer_impl!(i16x8);
wide_integer_impl!(i16x16);
wide_integer_impl!(i16x32);
wide_integer_impl!(i32x4);
wide_integer_impl!(i32x8);
wide_integer_impl!(i32x16);
wide_integer_impl!(i64x2);
wide_integer_impl!(i64x4);
wide_integer_impl!(i64x8);
wide_integer_impl!(u8x16);
wide_integer_impl!(u8x32);
wide_integer_impl!(u16x8);
wide_integer_impl!(u16x16);
wide_integer_impl!(u16x32);
wide_integer_impl!(u32x4);
wide_integer_impl!(u32x8);
wide_integer_impl!(u32x16);
wide_integer_impl!(u64x2);
wide_integer_impl!(u64x4);
wide_integer_impl!(u64x8);

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::i32x4;

    use crate::{
        Unaligned, Vector,
        utils::{assert_panic_test_eq, assert_test_eq_or_panic, for_types, random_iter},
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
