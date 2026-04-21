use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Alignment, Length, SupportedLength, Vector};

macro_rules! impl_wide_float {
    ($Wide:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// For each lane, returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(self) -> $Wide {
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
                self.map($Wide::is_nan)
            }

            /// For each lane, returns `true` if all elements are neither
            /// infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(self) -> $Wide {
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
                self.map($Wide::is_finite)
            }

            /// Returns the element-wise reciprocal (inverse) of a vector,
            /// `1 / self`.
            #[inline]
            #[must_use]
            pub fn recip(self) -> Self {
                Self::ONE / self
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
                    self.is_nan() == $Wide::splat(0.0) && other.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.max({other:?})"
                );

                Self::from_fn(|i| self[i].fast_max(other[i]))
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
                    self.is_nan() == $Wide::splat(0.0) && other.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.min({other:?})"
                );

                Self::from_fn(|i| self[i].fast_min(other[i]))
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
                    self.is_nan() == $Wide::splat(0.0)
                        && min.is_nan() == $Wide::splat(0.0)
                        && max.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.clamp({min:?}, {max:?})"
                );

                #[cfg(assertions)]
                assert!(
                    min.simd_gt_mask(max).any() == $Wide::splat(0.0),
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
            pub fn max_element(self) -> $Wide {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.max_element()"
                );

                match N {
                    2 => self[0].fast_max(self[1]),
                    3 => self[0].fast_max(self[1]).fast_max(self[2]),
                    4 => self[0]
                        .fast_max(self[1])
                        .fast_max(self[2])
                        .fast_max(self[3]),
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
            pub fn min_element(self) -> $Wide {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.min_element()"
                );

                match N {
                    2 => self[0].fast_min(self[1]),
                    3 => self[0].fast_min(self[1]).fast_min(self[2]),
                    4 => self[0]
                        .fast_min(self[1])
                        .fast_min(self[2])
                        .fast_min(self[3]),
                    _ => unreachable!(),
                }
            }
        }

        // MISSING: sign_positive_mask
        // MISSING: sign_negative_mask
        // MISSING: abs
        // MISSING: signum
        // MISSING: copysign
        // MISSING: floor
        // MISSING: ceil
        // MISSING: round
        // MISSING: trunc
        // MISSING: fract
        // MISSING: mul_add
        // MISSING: div_euclid
        // MISSING: rem_euclid
        // MISSING: powf
        // MISSING: sqrt
        // MISSING: exp
        // MISSING: exp2
        // MISSING: ln
        // MISSING: log2
        // MISSING: sin
        // MISSING: cos
        // MISSING: tan
        // MISSING: asin
        // MISSING: acos
        // MISSING: atan
        // MISSING: sin_cos
        // MISSING: lerp
        // MISSING: midpoint
        // MISSING: move_towards
        // MISSING: length
        // MISSING: distance
        // MISSING: distance_squared
        // MISSING: normalize
        // MISSING: try_normalize
        // MISSING: normalize_or
        // MISSING: normalize_or_zero
        // MISSING: normalize_and_length
        // MISSING: is_normalized
        // MISSING: with_max_length
        // MISSING: with_min_length
        // MISSING: clamp_length
        // MISSING: angle_between
        // MISSING: project_onto
        // MISSING: project_onto_normalized
        // MISSING: reject_from
        // MISSING: reject_from_normalized
        // MISSING: reflect
        // MISSING: refract
        // MISSING: abs_diff_eq
        // MISSING: rotate
        // MISSING: rotate_x
        // MISSING: rotate_y
        // MISSING: rotate_z
        // MISSING: any_orthogonal_vector
        // MISSING: any_orthonormal_vector
        // MISSING: any_orthonormal_pair
    };
}
impl_wide_float!(f32x4);
impl_wide_float!(f32x8);
impl_wide_float!(f32x16);
impl_wide_float!(f64x2);
impl_wide_float!(f64x4);
impl_wide_float!(f64x8);

#[cfg(test)]
mod tests {
    use wide::CmpGt;

    use crate::{
        Vector,
        utils::{assert_assertions_panic, assert_float_eq, for_parameters},
    };

    #[test]
    fn test_is_nan() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).is_nan(),
                x.is_nan() | y.is_nan()
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan()
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
            );
        });
    }

    #[test]
    fn test_nan_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).nan_mask(),
                Vector::<2, Wide, A>::new(x.is_nan(), y.is_nan())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).nan_mask(),
                Vector::<3, Wide, A>::new(x.is_nan(), y.is_nan(), z.is_nan())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).nan_mask(),
                Vector::<4, Wide, A>::new(x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan())
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).is_finite(),
                x.is_finite() & y.is_finite()
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite()
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
            );
        });
    }

    #[test]
    fn test_finite_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).finite_mask(),
                Vector::<2, Wide, A>::new(x.is_finite(), y.is_finite())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).finite_mask(),
                Vector::<3, Wide, A>::new(x.is_finite(), y.is_finite(), z.is_finite())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).finite_mask(),
                Vector::<4, Wide, A>::new(
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
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).recip(),
                Vector::<2, Wide, A>::new(1.0 / x, 1.0 / y)
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).recip(),
                Vector::<3, Wide, A>::new(1.0 / x, 1.0 / y, 1.0 / z)
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).recip(),
                Vector::<4, Wide, A>::new(1.0 / x, 1.0 / y, 1.0 / z, 1.0 / w)
            );
        });
    }

    #[test]
    fn test_max() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            if x.is_nan() == Wide::splat(0.0)
                && y.is_nan() == Wide::splat(0.0)
                && z.is_nan() == Wide::splat(0.0)
                && w.is_nan() == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Wide, A>::new(x, y).max(Vector::<2, Wide, A>::new(z, w)),
                    Vector::<2, Wide, A>::new(x.max(z), y.max(w))
                );
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, a).max(Vector::<3, Wide, A>::new(z, w, c)),
                    Vector::<3, Wide, A>::new(x.max(z), y.max(w), a.max(c))
                );
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, a, b)
                        .max(Vector::<4, Wide, A>::new(z, w, c, d)),
                    Vector::<4, Wide, A>::new(x.max(z), y.max(w), a.max(c), b.max(d))
                );
            } else {
                assert_assertions_panic!(
                    Vector::<2, Wide, A>::new(x, y).max(Vector::<2, Wide, A>::new(z, w))
                );
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, a).max(Vector::<
                    3,
                    Wide,
                    A,
                >::new(
                    z, w, c
                )));
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, a, b).max(Vector::<
                    4,
                    Wide,
                    A,
                >::new(
                    z, w, c, d
                )));
            }
        });
    }

    #[test]
    fn test_min() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            if x.is_nan() == Wide::splat(0.0)
                && y.is_nan() == Wide::splat(0.0)
                && z.is_nan() == Wide::splat(0.0)
                && w.is_nan() == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Wide, A>::new(x, y).min(Vector::<2, Wide, A>::new(z, w)),
                    Vector::<2, Wide, A>::new(x.min(z), y.min(w))
                );
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, a).min(Vector::<3, Wide, A>::new(z, w, c)),
                    Vector::<3, Wide, A>::new(x.min(z), y.min(w), a.min(c))
                );
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, a, b)
                        .min(Vector::<4, Wide, A>::new(z, w, c, d)),
                    Vector::<4, Wide, A>::new(x.min(z), y.min(w), a.min(c), b.min(d))
                );
            } else {
                assert_assertions_panic!(
                    Vector::<2, Wide, A>::new(x, y).min(Vector::<2, Wide, A>::new(z, w))
                );
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, a).min(Vector::<
                    3,
                    Wide,
                    A,
                >::new(
                    z, w, c
                )));
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, a, b).min(Vector::<
                    4,
                    Wide,
                    A,
                >::new(
                    z, w, c, d
                )));
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [x ^ y, x ^ z, y ^ z];
            let [c, d, e] = [x + y, x + z, y + z];
            let [f, g, h] = [c + d, c + e, d + e];

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (z.simd_gt(a) | w.simd_gt(b))
                == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Wide, A>::new(x, y).clamp(
                        Vector::<2, Wide, A>::new(z, w),
                        Vector::<2, Wide, A>::new(a, b)
                    ),
                    Vector::<2, Wide, A>::new(x.max(z).min(a), y.max(w).min(b))
                );
            } else {
                assert_assertions_panic!(Vector::<2, Wide, A>::new(x, y).clamp(
                    Vector::<2, Wide, A>::new(z, w),
                    Vector::<2, Wide, A>::new(a, b)
                ));
            }

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (c.is_nan() | d.is_nan() | e.is_nan() | w.simd_gt(c) | a.simd_gt(d))
                | b.simd_gt(e)
                == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, z).clamp(
                        Vector::<3, Wide, A>::new(w, a, b),
                        Vector::<3, Wide, A>::new(c, d, e)
                    ),
                    Vector::<3, Wide, A>::new(x.max(w).min(c), y.max(a).min(d), z.max(b).min(e))
                );
            } else {
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, z).clamp(
                    Vector::<3, Wide, A>::new(w, a, b),
                    Vector::<3, Wide, A>::new(c, d, e)
                ));
            }

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (c.is_nan() | d.is_nan() | e.is_nan() | f.is_nan() | g.is_nan() | h.is_nan())
                | (a.simd_gt(e) | b.simd_gt(f) | c.simd_gt(g) | d.simd_gt(h))
                == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, z, w).clamp(
                        Vector::<4, Wide, A>::new(a, b, c, d),
                        Vector::<4, Wide, A>::new(e, f, g, h)
                    ),
                    Vector::<4, Wide, A>::new(
                        x.max(a).min(e),
                        y.max(b).min(f),
                        z.max(c).min(g),
                        w.max(d).min(h)
                    )
                );
            } else {
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, z, w).clamp(
                    Vector::<4, Wide, A>::new(a, b, c, d),
                    Vector::<4, Wide, A>::new(e, f, g, h)
                ));
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            if x.is_nan() | y.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(Vector::<2, Wide, A>::new(x, y).max_element(), x.max(y));
            } else {
                assert_assertions_panic!(Vector::<2, Wide, A>::new(x, y).max_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, z).max_element(),
                    x.max(y).max(z)
                );
            } else {
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, z).max_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, z, w).max_element(),
                    x.max(y).max(z).max(w)
                );
            } else {
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, z, w).max_element());
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            if x.is_nan() | y.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(Vector::<2, Wide, A>::new(x, y).min_element(), x.min(y));
            } else {
                assert_assertions_panic!(Vector::<2, Wide, A>::new(x, y).min_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, z).min_element(),
                    x.min(y).min(z)
                );
            } else {
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, z).min_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, z, w).min_element(),
                    x.min(y).min(z).min(w)
                );
            } else {
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, z, w).min_element());
            }
        });
    }
}
