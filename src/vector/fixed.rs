use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
    types::extra::{LeEqU8, LeEqU16, LeEqU32, LeEqU64, LeEqU128},
};

use crate::{Alignment, Length, SupportedLength, Vector, utils::transmute_generic};

macro_rules! impl_fixed {
    ($T:ident, $Fixed:ident, $LeEq:ident) => {
        impl<const N: usize, Frac, A: Alignment> Vector<N, $Fixed<Frac>, A>
        where
            Length<N>: SupportedLength,
        {
            /// Creates a fixed-point vector from its bitwise representation (an
            /// integer vector).
            #[inline]
            #[must_use]
            pub const fn from_bits(bits: Vector<N, $T, A>) -> Self {
                // SAFETY: Fixed-point types accept all bit-patterns.
                unsafe { transmute_generic::<Vector<N, $T, A>, Vector<N, $Fixed<Frac>, A>>(bits) }
            }

            /// Returns the bitwise representation of `self` (an integer
            /// vector).
            #[inline]
            #[must_use]
            pub const fn to_bits(self) -> Vector<N, $T, A> {
                // SAFETY: Integers accept all bit-patterns.
                unsafe { transmute_generic::<Vector<N, $Fixed<Frac>, A>, Vector<N, $T, A>>(self) }
            }

            /// Returns the element-wise reciprocal (inverse) of a vector,
            /// `1 / self`.
            #[inline]
            #[must_use]
            pub fn recip(self) -> Self
            where
                Frac: $LeEq,
            {
                self.map($Fixed::recip)
            }

            /// Returns the maximum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                Self::from_bits(self.to_bits().max(other.to_bits()))
            }

            /// Returns the minimum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                Self::from_bits(self.to_bits().min(other.to_bits()))
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
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self::from_bits(self.to_bits().clamp(min.to_bits(), max.to_bits()))
            }

            /// Returns the maximum between the elements of `self`.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...`.
            #[inline]
            #[must_use]
            pub fn max_element(self) -> $Fixed<Frac> {
                $Fixed::from_bits(self.to_bits().max_element())
            }

            /// Returns the minimum between the elements of `self`.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...`.
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $Fixed<Frac> {
                $Fixed::from_bits(self.to_bits().max_element())
            }

            /// Returns the largest integers less than or equal to the elements
            /// of `self`.
            #[inline]
            #[must_use]
            pub fn floor(self) -> Self
            where
                Frac: $LeEq,
            {
                self.map($Fixed::floor)
            }

            /// Returns the smallest integers greater than or equal to the
            /// elements of `self`.
            #[inline]
            #[must_use]
            pub fn ceil(self) -> Self
            where
                Frac: $LeEq,
            {
                self.map($Fixed::ceil)
            }

            /// Returns the nearest integers to the elements of `self`.
            ///
            /// If a value is half-way between two integers, round away from 0.
            #[inline]
            #[must_use]
            pub fn round(self) -> Self
            where
                Frac: $LeEq,
            {
                self.map($Fixed::round)
            }

            /// Returns the integer part of the elements of `self`. This means
            /// that non-integer numbers are always truncated towards 0.
            #[inline]
            #[must_use]
            pub fn trunc(self) -> Self
            where
                Frac: $LeEq,
            {
                self.map($Fixed::round_to_zero)
            }

            /// Returns the fractional part of `self`.
            ///
            /// Equivalent to `self - self.trunc()`.
            #[inline]
            #[must_use]
            pub fn fract(self) -> Self
            where
                Frac: $LeEq,
            {
                self - self.trunc()
            }

            /// Calculates Euclidean division for the elements of `self`.
            ///
            /// Equivalent to
            /// `(self.x.div_euclid(rhs.x), self.y.div_euclid(rhs.y), ...)`.
            #[inline]
            #[must_use]
            pub fn div_euclid(self, rhs: Self) -> Self
            where
                Frac: $LeEq,
            {
                Self::from_fn(|i| self[i].div_euclid(rhs[i]))
            }

            /// Calculates Euclidean remainder for the elements of `self`.
            ///
            /// Equivalent to
            /// `(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y), ...)`.
            #[inline]
            #[must_use]
            pub fn rem_euclid(self, rhs: Self) -> Self
            where
                Frac: $LeEq,
            {
                Self::from_fn(|i| self[i].rem_euclid(rhs[i]))
            }

            /// Returns the square root of the elements of `self`.
            ///
            /// Equivalent to `(self.x.sqrt(), self.y.sqrt(), ...)`.
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self
            where
                Frac: $LeEq,
            {
                self.map($Fixed::sqrt)
            }

            /// Computes the linear interpolation between `self` and `other`
            /// based on the value `t`.
            ///
            /// When `t` is 0, the result is `self`. When `t` is 1, the result
            /// is `rhs`. When `t` is outside of the range `0..=1`, the result
            /// is linearly extrapolated.
            #[inline]
            #[must_use]
            pub fn lerp(self, other: Self, t: $Fixed<Frac>) -> Self
            where
                Frac: $LeEq,
            {
                self + (other - self) * t
            }

            /// Moves `self` towards `other` by at most `max_delta`.
            ///
            /// When `max_delta` is `0`, the result is `self`. When `max_delta`
            /// is equal to or greater than `self.distance(other)`, the result
            /// is `other`.
            #[inline]
            #[must_use]
            pub fn move_towards(self, target: Self, max_delta: $Fixed<Frac>) -> Self
            where
                Frac: $LeEq,
            {
                let delta = target - self;
                let delta_length = delta.length();

                if delta_length <= max_delta || delta_length <= $Fixed::<Frac>::from_num(1e-4) {
                    target
                } else {
                    self + delta / delta_length * max_delta
                }
            }

            /// Returns the length/magnitude of `self`.
            #[inline]
            #[must_use]
            pub fn length(self) -> $Fixed<Frac>
            where
                Frac: $LeEq,
            {
                self.length_squared().sqrt()
            }
        }
    };
}
impl_fixed!(i8, FixedI8, LeEqU8);
impl_fixed!(i16, FixedI16, LeEqU16);
impl_fixed!(i32, FixedI32, LeEqU32);
impl_fixed!(i64, FixedI64, LeEqU64);
impl_fixed!(i128, FixedI128, LeEqU128);
impl_fixed!(u8, FixedU8, LeEqU8);
impl_fixed!(u16, FixedU16, LeEqU16);
impl_fixed!(u32, FixedU32, LeEqU32);
impl_fixed!(u64, FixedU64, LeEqU64);
impl_fixed!(u128, FixedU128, LeEqU128);
