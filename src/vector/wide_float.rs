use wide::{CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Alignment, Length, SupportedLength, Vector};

macro_rules! impl_wide_float {
    ($Simd:ident, $fast_max:ident, $fast_min:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $Simd, A>
        where
            Length<N>: SupportedLength,
        {
            /// For each lane, returns `true` if all elements of `self` are
            /// `true`.
            #[inline]
            #[must_use]
            pub fn all(self) -> $Simd {
                match N {
                    2 => self[0] & self[1],
                    3 => self[0] & self[1] & self[2],
                    4 => self[0] & self[1] & self[2] & self[3],
                    _ => unreachable!(),
                }
            }

            /// For each lane, returns `true` if any element of `self` is
            /// `true`.
            #[inline]
            #[must_use]
            pub fn any(self) -> $Simd {
                match N {
                    2 => self[0] | self[1],
                    3 => self[0] | self[1] | self[2],
                    4 => self[0] | self[1] | self[2] | self[3],
                    _ => unreachable!(),
                }
            }

            /// For each lane, selects between the elements of `if_true` and
            /// `if_false` based on the boolean elements of `self`.
            #[inline]
            #[must_use]
            pub fn blend(self, if_true: Self, if_false: Self) -> Self {
                Vector::from_fn(|i| self[i].blend(if_true[i], if_false[i]))
            }

            /// For each lane, returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(self) -> $Simd {
                self.nan_mask().any()
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is NaN.
            ///
            /// Equivalent to `(self.x.is_nan(), self.y.is_nan(), ...)` for each
            /// lane.
            #[inline]
            #[must_use]
            pub fn nan_mask(self) -> Self {
                self.map($Simd::is_nan)
            }

            /// For each lane, returns `true` if all elements are neither
            /// infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(self) -> $Simd {
                self.finite_mask().all()
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is neither
            /// infinite nor NaN.
            ///
            /// Equivalent to `(self.x.is_finite(), self.y.is_finite(), ...)`
            /// for each lane.
            #[inline]
            #[must_use]
            pub fn finite_mask(self) -> Self {
                self.map($Simd::is_finite)
            }

            /// Returns the element-wise reciprocal (inverse) of a vector,
            /// `1 / self`.
            #[inline]
            #[must_use]
            pub fn recip(self) -> Self {
                Self::ONE / self
            }

            /// For each lane, returns `true` if `self` is equal to `other`.
            ///
            /// Equivalent to
            /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
            #[inline]
            #[must_use]
            pub fn simd_eq(self, other: Self) -> $Simd {
                match N {
                    2 => self[0].simd_eq(other[0]) & self[1].simd_eq(other[1]),
                    3 => {
                        self[0].simd_eq(other[0])
                            & self[1].simd_eq(other[1])
                            & self[2].simd_eq(other[2])
                    }
                    4 => {
                        self[0].simd_eq(other[0])
                            & self[1].simd_eq(other[1])
                            & self[2].simd_eq(other[2])
                            & self[3].simd_eq(other[3])
                    }
                    _ => unreachable!(),
                }
            }

            /// For each lane, returns `true` if `self` is not equal to `other`.
            ///
            /// Equivalent to
            /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
            #[inline]
            #[must_use]
            pub fn simd_ne(self, other: Self) -> $Simd {
                match N {
                    2 => self[0].simd_ne(other[0]) | self[1].simd_ne(other[1]),
                    3 => {
                        self[0].simd_ne(other[0])
                            | self[1].simd_ne(other[1])
                            | self[2].simd_ne(other[2])
                    }
                    4 => {
                        self[0].simd_ne(other[0])
                            | self[1].simd_ne(other[1])
                            | self[2].simd_ne(other[2])
                            | self[3].simd_ne(other[3])
                    }
                    _ => unreachable!(),
                }
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding elements of `self` and `other` are
            /// equal.
            ///
            /// Equivalent to `(self.x == other.x, self.y == other.y, ...)` for
            /// each lane.
            #[inline]
            #[must_use]
            pub fn simd_eq_mask(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].simd_eq(other[i]))
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding elements of `self` and `other` are
            /// not equal.
            ///
            /// Equivalent to `(self.x != other.x, self.y != other.y, ...)` for
            /// each lane.
            #[inline]
            #[must_use]
            pub fn simd_ne_mask(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].simd_ne(other[i]))
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is less than the
            /// corresponding element of `other`.
            ///
            /// Equivalent to `(self.x < other.x, self.y < other.y, ...)` for
            /// each lane.
            #[inline]
            #[must_use]
            pub fn simd_lt_mask(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].simd_lt(other[i]))
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is greater than
            /// the corresponding element of `other`.
            ///
            /// Equivalent to `(self.x > other.x, self.y > other.y, ...)` for
            /// each lane.
            #[inline]
            #[must_use]
            pub fn simd_gt_mask(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].simd_gt(other[i]))
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is less than or
            /// equal to the corresponding element of `other`.
            ///
            /// Equivalent to `(self.x <= other.x, self.y <= other.y, ...)` for
            /// each lane.
            #[inline]
            #[must_use]
            pub fn simd_le_mask(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].simd_le(other[i]))
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is greater than or
            /// equal to the corresponding element of `other`.
            ///
            /// Equivalent to `(self.x >= other.x, self.y >= other.y, ...)` for
            /// each lane.
            #[inline]
            #[must_use]
            pub fn simd_ge_mask(self, other: Self) -> Self {
                Self::from_fn(|i| self[i].simd_ge(other[i]))
            }

            /// Returns the maximum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn max(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Simd::splat(0.0) && other.is_nan() == $Simd::splat(0.0),
                    "NaN: {self:?}.max({other:?})"
                );

                Self::from_fn(|i| self[i].$fast_max(other[i]))
            }

            /// Returns the minimum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn min(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Simd::splat(0.0) && other.is_nan() == $Simd::splat(0.0),
                    "NaN: {self:?}.min({other:?})"
                );

                Self::from_fn(|i| self[i].$fast_min(other[i]))
            }

            /// For each lane, clamps the elements of `self` between the
            /// elements of `min` and `max`.
            ///
            /// Equivalent to
            /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN, or if any element of `min` is
            /// greater than the corresponding element of `max`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Simd::splat(0.0)
                        && min.is_nan() == $Simd::splat(0.0)
                        && max.is_nan() == $Simd::splat(0.0),
                    "NaN: {self:?}.clamp({min:?}, {max:?})"
                );

                #[cfg(assertions)]
                assert!(
                    min.simd_gt_mask(max).any() == $Simd::splat(0.0),
                    "min > max: {self:?}.clamp({min:?}, {max:?})"
                );

                self.max(min).min(max)
            }

            /// For each lane, returns the maximum between the elements of
            /// `self`.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...` for each lane.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn max_element(self) -> $Simd {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Simd::splat(0.0),
                    "NaN: {self:?}.max_element()"
                );

                match N {
                    2 => self[0].max(self[1]),
                    3 => self[0].max(self[1]).max(self[2]),
                    4 => self[0].max(self[1]).max(self[2]).max(self[3]),
                    _ => unreachable!(),
                }
            }

            /// For each lane, returns the minimum between the elements of
            /// `self`.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...` for each lane.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn min_element(self) -> $Simd {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Simd::splat(0.0),
                    "NaN: {self:?}.min_element()"
                );

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
impl_wide_float!(f32x4, fast_max, fast_min);
impl_wide_float!(f32x8, fast_max, fast_min);
// `f32x16` is missing `fast_max` and `fast_min`.
impl_wide_float!(f32x16, max, min);
impl_wide_float!(f64x2, fast_max, fast_min);
impl_wide_float!(f64x4, fast_max, fast_min);
impl_wide_float!(f64x8, fast_max, fast_min);

#[cfg(test)]
mod tests {
    use wide::{CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

    use crate::{
        Vector,
        utils::{assert_assertions_panic, assert_float_eq, for_parameters},
    };

    #[test]
    fn test_all() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(Vector::<2, Simd, A>::new(x, y).all(), x & y);
            assert_float_eq!(Vector::<3, Simd, A>::new(x, y, z).all(), x & y & z);
            assert_float_eq!(Vector::<4, Simd, A>::new(x, y, z, w).all(), x & y & z & w);
        });
    }

    #[test]
    fn test_any() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(Vector::<2, Simd, A>::new(x, y).any(), x | y);
            assert_float_eq!(Vector::<3, Simd, A>::new(x, y, z).any(), x | y | z);
            assert_float_eq!(Vector::<4, Simd, A>::new(x, y, z, w).any(), x | y | z | w);
        });
    }

    #[test]
    fn test_blend() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).blend(
                    Vector::<2, Simd, A>::new(y, z),
                    Vector::<2, Simd, A>::new(z, w)
                ),
                Vector::<2, Simd, A>::new(x.blend(y, z), y.blend(z, w))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).blend(
                    Vector::<3, Simd, A>::new(y, z, w),
                    Vector::<3, Simd, A>::new(z, w, x)
                ),
                Vector::<3, Simd, A>::new(x.blend(y, z), y.blend(z, w), z.blend(w, x))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w).blend(
                    Vector::<4, Simd, A>::new(y, z, w, x),
                    Vector::<4, Simd, A>::new(z, w, x, y)
                ),
                Vector::<4, Simd, A>::new(
                    x.blend(y, z),
                    y.blend(z, w),
                    z.blend(w, x),
                    w.blend(x, y)
                )
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).is_nan(),
                x.is_nan() | y.is_nan()
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan()
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
            );
        });
    }

    #[test]
    fn test_nan_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).nan_mask(),
                Vector::<2, Simd, A>::new(x.is_nan(), y.is_nan())
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).nan_mask(),
                Vector::<3, Simd, A>::new(x.is_nan(), y.is_nan(), z.is_nan())
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w).nan_mask(),
                Vector::<4, Simd, A>::new(x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan())
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).is_finite(),
                x.is_finite() & y.is_finite()
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite()
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
            );
        });
    }

    #[test]
    fn test_finite_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).finite_mask(),
                Vector::<2, Simd, A>::new(x.is_finite(), y.is_finite())
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).finite_mask(),
                Vector::<3, Simd, A>::new(x.is_finite(), y.is_finite(), z.is_finite())
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w).finite_mask(),
                Vector::<4, Simd, A>::new(
                    x.is_finite(),
                    y.is_finite(),
                    z.is_finite(),
                    w.is_finite()
                )
            );
        });
    }

    #[test]
    fn test_recip() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).recip(),
                Vector::<2, Simd, A>::new(1.0 / x, 1.0 / y)
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).recip(),
                Vector::<3, Simd, A>::new(1.0 / x, 1.0 / y, 1.0 / z)
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w).recip(),
                Vector::<4, Simd, A>::new(1.0 / x, 1.0 / y, 1.0 / z, 1.0 / w)
            );
        });
    }

    #[test]
    fn test_simd_eq() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_eq(Vector::<2, Simd, A>::new(a, b)),
                x.simd_eq(a) & y.simd_eq(b)
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_eq(Vector::<3, Simd, A>::new(a, b, c)),
                x.simd_eq(a) & y.simd_eq(b) & z.simd_eq(c)
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_eq(Vector::<4, Simd, A>::new(a, b, c, d)),
                x.simd_eq(a) & y.simd_eq(b) & z.simd_eq(c) & w.simd_eq(d)
            );
        });
    }

    #[test]
    fn test_simd_ne() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_ne(Vector::<2, Simd, A>::new(a, b)),
                x.simd_ne(a) | y.simd_ne(b)
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_ne(Vector::<3, Simd, A>::new(a, b, c)),
                x.simd_ne(a) | y.simd_ne(b) | z.simd_ne(c)
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_ne(Vector::<4, Simd, A>::new(a, b, c, d)),
                x.simd_ne(a) | y.simd_ne(b) | z.simd_ne(c) | w.simd_ne(d)
            );
        });
    }

    #[test]
    fn test_simd_eq_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_eq_mask(Vector::<2, Simd, A>::new(a, b)),
                Vector::<2, Simd, A>::new(x.simd_eq(a), y.simd_eq(b))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_eq_mask(Vector::<3, Simd, A>::new(a, b, c)),
                Vector::<3, Simd, A>::new(x.simd_eq(a), y.simd_eq(b), z.simd_eq(c))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_eq_mask(Vector::<4, Simd, A>::new(a, b, c, d)),
                Vector::<4, Simd, A>::new(x.simd_eq(a), y.simd_eq(b), z.simd_eq(c), w.simd_eq(d))
            );
        });
    }

    #[test]
    fn test_simd_ne_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_ne_mask(Vector::<2, Simd, A>::new(a, b)),
                Vector::<2, Simd, A>::new(x.simd_ne(a), y.simd_ne(b))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_ne_mask(Vector::<3, Simd, A>::new(a, b, c)),
                Vector::<3, Simd, A>::new(x.simd_ne(a), y.simd_ne(b), z.simd_ne(c))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_ne_mask(Vector::<4, Simd, A>::new(a, b, c, d)),
                Vector::<4, Simd, A>::new(x.simd_ne(a), y.simd_ne(b), z.simd_ne(c), w.simd_ne(d))
            );
        });
    }

    #[test]
    fn test_simd_lt_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_lt_mask(Vector::<2, Simd, A>::new(a, b)),
                Vector::<2, Simd, A>::new(x.simd_lt(a), y.simd_lt(b))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_lt_mask(Vector::<3, Simd, A>::new(a, b, c)),
                Vector::<3, Simd, A>::new(x.simd_lt(a), y.simd_lt(b), z.simd_lt(c))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_lt_mask(Vector::<4, Simd, A>::new(a, b, c, d)),
                Vector::<4, Simd, A>::new(x.simd_lt(a), y.simd_lt(b), z.simd_lt(c), w.simd_lt(d))
            );
        });
    }

    #[test]
    fn test_simd_gt_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_gt_mask(Vector::<2, Simd, A>::new(a, b)),
                Vector::<2, Simd, A>::new(x.simd_gt(a), y.simd_gt(b))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_gt_mask(Vector::<3, Simd, A>::new(a, b, c)),
                Vector::<3, Simd, A>::new(x.simd_gt(a), y.simd_gt(b), z.simd_gt(c))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_gt_mask(Vector::<4, Simd, A>::new(a, b, c, d)),
                Vector::<4, Simd, A>::new(x.simd_gt(a), y.simd_gt(b), z.simd_gt(c), w.simd_gt(d))
            );
        });
    }

    #[test]
    fn test_simd_le_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_le_mask(Vector::<2, Simd, A>::new(a, b)),
                Vector::<2, Simd, A>::new(x.simd_le(a), y.simd_le(b))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_le_mask(Vector::<3, Simd, A>::new(a, b, c)),
                Vector::<3, Simd, A>::new(x.simd_le(a), y.simd_le(b), z.simd_le(c))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_le_mask(Vector::<4, Simd, A>::new(a, b, c, d)),
                Vector::<4, Simd, A>::new(x.simd_le(a), y.simd_le(b), z.simd_le(c), w.simd_le(d))
            );
        });
    }

    #[test]
    fn test_simd_ge_mask() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Simd, A>::new(x, y).simd_ge_mask(Vector::<2, Simd, A>::new(a, b)),
                Vector::<2, Simd, A>::new(x.simd_ge(a), y.simd_ge(b))
            );
            assert_float_eq!(
                Vector::<3, Simd, A>::new(x, y, z).simd_ge_mask(Vector::<3, Simd, A>::new(a, b, c)),
                Vector::<3, Simd, A>::new(x.simd_ge(a), y.simd_ge(b), z.simd_ge(c))
            );
            assert_float_eq!(
                Vector::<4, Simd, A>::new(x, y, z, w)
                    .simd_ge_mask(Vector::<4, Simd, A>::new(a, b, c, d)),
                Vector::<4, Simd, A>::new(x.simd_ge(a), y.simd_ge(b), z.simd_ge(c), w.simd_ge(d))
            );
        });
    }

    #[test]
    fn test_max() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            if x.is_nan() == Simd::splat(0.0)
                && y.is_nan() == Simd::splat(0.0)
                && z.is_nan() == Simd::splat(0.0)
                && w.is_nan() == Simd::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Simd, A>::new(x, y).max(Vector::<2, Simd, A>::new(z, w)),
                    Vector::<2, Simd, A>::new(x.max(z), y.max(w))
                );
                assert_float_eq!(
                    Vector::<3, Simd, A>::new(x, y, a).max(Vector::<3, Simd, A>::new(z, w, c)),
                    Vector::<3, Simd, A>::new(x.max(z), y.max(w), a.max(c))
                );
                assert_float_eq!(
                    Vector::<4, Simd, A>::new(x, y, a, b)
                        .max(Vector::<4, Simd, A>::new(z, w, c, d)),
                    Vector::<4, Simd, A>::new(x.max(z), y.max(w), a.max(c), b.max(d))
                );
            } else {
                assert_assertions_panic!(
                    Vector::<2, Simd, A>::new(x, y).max(Vector::<2, Simd, A>::new(z, w))
                );
                assert_assertions_panic!(Vector::<3, Simd, A>::new(x, y, a).max(Vector::<
                    3,
                    Simd,
                    A,
                >::new(
                    z, w, c
                )));
                assert_assertions_panic!(Vector::<4, Simd, A>::new(x, y, a, b).max(Vector::<
                    4,
                    Simd,
                    A,
                >::new(
                    z, w, c, d
                )));
            }
        });
    }

    #[test]
    fn test_min() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            if x.is_nan() == Simd::splat(0.0)
                && y.is_nan() == Simd::splat(0.0)
                && z.is_nan() == Simd::splat(0.0)
                && w.is_nan() == Simd::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Simd, A>::new(x, y).min(Vector::<2, Simd, A>::new(z, w)),
                    Vector::<2, Simd, A>::new(x.min(z), y.min(w))
                );
                assert_float_eq!(
                    Vector::<3, Simd, A>::new(x, y, a).min(Vector::<3, Simd, A>::new(z, w, c)),
                    Vector::<3, Simd, A>::new(x.min(z), y.min(w), a.min(c))
                );
                assert_float_eq!(
                    Vector::<4, Simd, A>::new(x, y, a, b)
                        .min(Vector::<4, Simd, A>::new(z, w, c, d)),
                    Vector::<4, Simd, A>::new(x.min(z), y.min(w), a.min(c), b.min(d))
                );
            } else {
                assert_assertions_panic!(
                    Vector::<2, Simd, A>::new(x, y).min(Vector::<2, Simd, A>::new(z, w))
                );
                assert_assertions_panic!(Vector::<3, Simd, A>::new(x, y, a).min(Vector::<
                    3,
                    Simd,
                    A,
                >::new(
                    z, w, c
                )));
                assert_assertions_panic!(Vector::<4, Simd, A>::new(x, y, a, b).min(Vector::<
                    4,
                    Simd,
                    A,
                >::new(
                    z, w, c, d
                )));
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let [w, a, b] = [x ^ y, x ^ z, y ^ z];
            let [c, d, e] = [x + y, x + z, y + z];
            let [f, g, h] = [c + d, c + e, d + e];

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (z.simd_gt(a) | w.simd_gt(b))
                == Simd::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Simd, A>::new(x, y).clamp(
                        Vector::<2, Simd, A>::new(z, w),
                        Vector::<2, Simd, A>::new(a, b)
                    ),
                    Vector::<2, Simd, A>::new(x.max(z).min(a), y.max(w).min(b))
                );
            } else {
                assert_assertions_panic!(Vector::<2, Simd, A>::new(x, y).clamp(
                    Vector::<2, Simd, A>::new(z, w),
                    Vector::<2, Simd, A>::new(a, b)
                ));
            }

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (c.is_nan() | d.is_nan() | e.is_nan() | w.simd_gt(c) | a.simd_gt(d))
                | b.simd_gt(e)
                == Simd::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<3, Simd, A>::new(x, y, z).clamp(
                        Vector::<3, Simd, A>::new(w, a, b),
                        Vector::<3, Simd, A>::new(c, d, e)
                    ),
                    Vector::<3, Simd, A>::new(x.max(w).min(c), y.max(a).min(d), z.max(b).min(e))
                );
            } else {
                assert_assertions_panic!(Vector::<3, Simd, A>::new(x, y, z).clamp(
                    Vector::<3, Simd, A>::new(w, a, b),
                    Vector::<3, Simd, A>::new(c, d, e)
                ));
            }

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (c.is_nan() | d.is_nan() | e.is_nan() | f.is_nan() | g.is_nan() | h.is_nan())
                | (a.simd_gt(e) | b.simd_gt(f) | c.simd_gt(g) | d.simd_gt(h))
                == Simd::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<4, Simd, A>::new(x, y, z, w).clamp(
                        Vector::<4, Simd, A>::new(a, b, c, d),
                        Vector::<4, Simd, A>::new(e, f, g, h)
                    ),
                    Vector::<4, Simd, A>::new(
                        x.max(a).min(e),
                        y.max(b).min(f),
                        z.max(c).min(g),
                        w.max(d).min(h)
                    )
                );
            } else {
                assert_assertions_panic!(Vector::<4, Simd, A>::new(x, y, z, w).clamp(
                    Vector::<4, Simd, A>::new(a, b, c, d),
                    Vector::<4, Simd, A>::new(e, f, g, h)
                ));
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;

            if x.is_nan() | y.is_nan() == Simd::splat(0.0) {
                assert_float_eq!(Vector::<2, Simd, A>::new(x, y).max_element(), x.max(y));
            } else {
                assert_assertions_panic!(Vector::<2, Simd, A>::new(x, y).max_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() == Simd::splat(0.0) {
                assert_float_eq!(
                    Vector::<3, Simd, A>::new(x, y, z).max_element(),
                    x.max(y).max(z)
                );
            } else {
                assert_assertions_panic!(Vector::<3, Simd, A>::new(x, y, z).max_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() == Simd::splat(0.0) {
                assert_float_eq!(
                    Vector::<4, Simd, A>::new(x, y, z, w).max_element(),
                    x.max(y).max(z).max(w)
                );
            } else {
                assert_assertions_panic!(Vector::<4, Simd, A>::new(x, y, z, w).max_element());
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_parameters!(|Simd: WideFloat, A, x, y, z| {
            let _: [Simd; 3] = [x, y, z];
            let w = x ^ y;

            if x.is_nan() | y.is_nan() == Simd::splat(0.0) {
                assert_float_eq!(Vector::<2, Simd, A>::new(x, y).min_element(), x.min(y));
            } else {
                assert_assertions_panic!(Vector::<2, Simd, A>::new(x, y).min_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() == Simd::splat(0.0) {
                assert_float_eq!(
                    Vector::<3, Simd, A>::new(x, y, z).min_element(),
                    x.min(y).min(z)
                );
            } else {
                assert_assertions_panic!(Vector::<3, Simd, A>::new(x, y, z).min_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() == Simd::splat(0.0) {
                assert_float_eq!(
                    Vector::<4, Simd, A>::new(x, y, z, w).min_element(),
                    x.min(y).min(z).min(w)
                );
            } else {
                assert_assertions_panic!(Vector::<4, Simd, A>::new(x, y, z, w).min_element());
            }
        });
    }
}
