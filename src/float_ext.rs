use crate::utils::PrimitiveFloat;

/// Extends floating-point primitives with extra functionality.
pub trait FloatExt {
    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`.
    ///
    /// When `t` is `0.0`, the result is `self`.  When `t` is `1.0`, the result
    /// is `other`. When `t` is outside of the range `0.0..=1.0`, the result is
    /// linearly extrapolated.
    #[must_use]
    fn lerp(self, other: Self, t: Self) -> Self;

    /// Returns `true` if the absolute difference between `self` and `other` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two values that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[must_use]
    fn abs_diff_eq(self, other: Self, max_abs_diff: Self) -> bool
    where
        Self: Sized,
    {
        // TODO: remove this default implementation and make `FloatExt` sealed
        // for `0.17.0`.
        let _ = (other, max_abs_diff);
        unimplemented!()
    }
}

impl<T: PrimitiveFloat> FloatExt for T {
    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (T::as_from(1.0) - t) + other * t
    }

    #[inline]
    fn abs_diff_eq(self, other: Self, max_abs_diff: Self) -> bool {
        (self - other).abs() <= max_abs_diff
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FloatExt,
        utils::{assert_float_eq, for_parameters},
    };

    #[test]
    fn test_lerp() {
        for_parameters!(|T: PrimitiveFloat, x, y, z| {
            if !T::is_finite(x) || !T::is_finite(y) || !T::is_finite(z) {
                return;
            }

            assert_float_eq!(x.lerp(y, 0.0), x, 0.0 = -0.0);
            assert_float_eq!(x.lerp(y, 0.5), x * 0.5 + y * 0.5, 0.0 = -0.0);
            assert_float_eq!(x.lerp(y, 1.0), y, 0.0 = -0.0);
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat| {
            assert!(T::abs_diff_eq(0.0, 0.0, 0.125));
            assert!(T::abs_diff_eq(0.0, 0.1, 0.125));
            assert!(T::abs_diff_eq(5.0, 4.9, 0.125));
            assert!(!T::abs_diff_eq(0.0, 1.0, 0.125));
            assert!(!T::abs_diff_eq(0.0, 0.9, 0.125));
            assert!(!T::abs_diff_eq(5.0, 3.9, 0.125));
        });
    }
}
