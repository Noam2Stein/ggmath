/// An extension trait for float primitives.
pub trait FloatExt {
    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`.
    ///
    /// When `t` is `0.0`, the result is `self`.  When `t` is `1.0`, the result
    /// is `other`. When `t` is outside of the range `[0.0, 1.0]`, the result is
    /// linearly extrapolated.
    #[must_use]
    fn lerp(self, other: Self, t: Self) -> Self;
}

macro_rules! float_impl {
    ($T:ident) => {
        impl FloatExt for $T {
            #[inline]
            fn lerp(self, other: Self, t: Self) -> Self {
                self * (1.0 - t) + other * t
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
