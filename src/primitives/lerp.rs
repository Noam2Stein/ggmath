use super::*;

repetitive! {
    @for prim in ['f32, 'f64] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            /// Performs linear interpolation between `self` and `other` based on the value `t`.
            ///
            /// When `t` is `0.0`, the result will be equal to `self`.
            /// When `t` is `1.0`, the result will be equal to `other`.
            /// `t` is clamped to the range `[0, 1]`.
            ///
            /// For unclamped interpolation, use `lerp_unclamped`.
            #[inline(always)]
            pub fn lerp(self, other: Vector<N, @prim, impl VecAlignment>, t: @prim) -> Self {
                self.lerp_unclamped(other, t.clamp(0.0, 1.0))
            }

            /// Performs linear interpolation between `self` and `other` based on the value `t`.
            ///
            /// When `t` is `0.0`, the result will be equal to `self`.
            /// When `t` is `1.0`, the result will be equal to `other`.
            /// When `t` is outside of range `[0, 1]`, the result is linearly extrapolated.
            #[inline(always)]
            pub fn lerp_unclamped(self, other: Vector<N, @prim, impl VecAlignment>, t: @prim) -> Self {
                self * (1.0 - t) + other * t
            }
        }
    }
}
