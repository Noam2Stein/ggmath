use crate::num_primitive::PrimitiveFloat;

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
}

impl<T: PrimitiveFloat> FloatExt for T {
    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (T::as_from(1.0) - t) + other * t
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FloatExt,
        test_utils::{assert_float_eq, for_parameters},
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
}
