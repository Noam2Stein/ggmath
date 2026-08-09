use crate::PrimitiveFloat;

/// Extends floating-point primitives with extra functionality.
#[expect(private_bounds)]
pub trait FloatExt: Sealed {
    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`.
    ///
    /// When `t` is `0.0`, the result is `self`.  When `t` is `1.0`, the result
    /// is `other`. When `t` is outside of the range `0.0..=1.0`, the result is
    /// linearly extrapolated.
    #[must_use]
    fn lerp(self, other: Self, t: Self) -> Self;

    /// Moves `self` towards `other` by at most `max_delta`.
    ///
    /// When `max_delta` is `0`, the result is `self`. When `max_delta` is equal
    /// to or greater than `(self - target).abs()`, the result is `target`.
    ///
    /// ```
    /// # use ggmath::FloatExt;
    /// #
    /// assert_eq!(2.0.move_towards(5.0, 1.0), 3.0);
    /// assert_eq!(2.0.move_towards(-5.0, 1.0), 1.0);
    /// ```
    #[must_use]
    fn move_towards(self, target: Self, max_delta: Self) -> Self;

    /// Returns `true` if the absolute difference between `self` and `other` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two values that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[must_use]
    fn abs_diff_eq(self, other: Self, max_abs_diff: Self) -> bool;
}

trait Sealed {}

impl<T: PrimitiveFloat> FloatExt for T {
    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (T::ONE - t) + other * t
    }

    #[inline]
    fn move_towards(self, target: Self, max_delta: Self) -> Self {
        let delta = target - self;
        let delta_abs = delta.abs();

        if delta_abs <= max_delta || delta_abs <= Self::as_from(1e-4) {
            target
        } else {
            self + max_delta * delta.signum()
        }
    }

    #[inline]
    fn abs_diff_eq(self, other: Self, max_abs_diff: Self) -> bool {
        (self - other).abs() <= max_abs_diff
    }
}

impl<T: PrimitiveFloat> Sealed for T {}

#[cfg(feature = "wide")]
mod wide {
    use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

    use crate::{FloatExt, float_ext::Sealed};

    macro_rules! impl_wide_float {
        ($Wide:ident) => {
            impl FloatExt for $Wide {
                #[inline]
                fn lerp(self, other: Self, t: Self) -> Self {
                    self * (Self::ONE - t) + other * t
                }

                #[inline]
                fn move_towards(self, target: Self, max_delta: Self) -> Self {
                    let delta = target - self;
                    let delta_abs = delta.abs();

                    (delta_abs.simd_le(max_delta) | delta_abs.simd_le($Wide::splat(1e-4)))
                        .blend(target, self + max_delta * delta.signum())
                }

                #[inline]
                fn abs_diff_eq(self, other: Self, max_abs_diff: Self) -> bool {
                    (self - other).abs().simd_le(max_abs_diff).all()
                }
            }

            impl Sealed for $Wide {}
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
        use wide::f32x4;

        use crate::{
            FloatExt,
            test_utils::{assert_test_eq, random_iter},
        };

        #[test]
        fn test_lerp() {
            for [value, other, t] in random_iter::<[f32x4; 3]>() {
                assert_test_eq!(
                    value.lerp(other, t),
                    f32x4::new(core::array::from_fn(
                        |i| value.to_array()[i].lerp(other.to_array()[i], t.to_array()[i])
                    ))
                );
            }
        }

        #[test]
        fn test_move_towards() {
            for [value, target, max_delta] in random_iter::<[f32x4; 3]>() {
                assert_test_eq!(
                    value.move_towards(target, max_delta),
                    f32x4::new(core::array::from_fn(|i| value.to_array()[i]
                        .move_towards(target.to_array()[i], max_delta.to_array()[i])))
                );
            }
        }

        #[test]
        fn test_abs_diff_eq() {
            for [value, other, max_abs_diff] in random_iter::<[f32x4; 3]>() {
                assert_eq!(
                    value.abs_diff_eq(other, max_abs_diff),
                    (0..4).all(|i| value.to_array()[i]
                        .abs_diff_eq(other.to_array()[i], max_abs_diff.to_array()[i]))
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FloatExt,
        test_utils::{assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_lerp() {
        for_types!(|T: PrimitiveFloat| {
            for [x, y] in
                random_iter::<[T; 2]>().filter(|values| values.iter().all(|x| x.is_finite()))
            {
                assert_test_eq!(x.lerp(y, 0.0), x, 0.0 = -0.0);
                assert_test_eq!(x.lerp(y, 0.5), x * 0.5 + y * 0.5, 0.0 = -0.0);
                assert_test_eq!(x.lerp(y, 1.0), y, 0.0 = -0.0);
            }
        });
    }

    #[test]
    fn test_move_towards() {
        for_types!(|T: PrimitiveFloat| {
            assert!(T::abs_diff_eq(5.0.move_towards(10.0, 2.0), 7.0, 1e-5));
            assert!(T::abs_diff_eq(10.0.move_towards(5.0, 2.0), 8.0, 1e-5));
            assert!(T::abs_diff_eq((-5.0).move_towards(10.0, 2.0), -3.0, 1e-5));
            assert!(T::abs_diff_eq(10.0.move_towards(-5.0, 2.0), 8.0, 1e-5));
            assert!(T::abs_diff_eq(5.0.move_towards(10.0, 20.0), 10.0, 1e-5));
            assert!(T::abs_diff_eq(10.0.move_towards(5.0, 20.0), 5.0, 1e-5));
            assert!(T::abs_diff_eq((-5.0).move_towards(10.0, 20.0), 10.0, 1e-5));
            assert!(T::abs_diff_eq(10.0.move_towards(-5.0, 20.0), -5.0, 1e-5));
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat| {
            assert!(T::abs_diff_eq(0.0, 0.0, 0.125));
            assert!(T::abs_diff_eq(0.0, 0.1, 0.125));
            assert!(T::abs_diff_eq(5.0, 4.9, 0.125));
            assert!(!T::abs_diff_eq(0.0, 1.0, 0.125));
            assert!(!T::abs_diff_eq(0.0, 0.9, 0.125));
            assert!(!T::abs_diff_eq(5.0, 3.9, 0.125));
        });
    }
}
