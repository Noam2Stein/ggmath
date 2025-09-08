use indoc::formatdoc;

use crate::module::Mod;

pub fn write_mod(module: Mod) {
    let mut impls = Vec::new();

    for primitive in ["f32", "f64"] {
        impls.push(formatdoc! {r#"
            impl FloatExt for {primitive} {{
                #[inline(always)]
                fn lerp(self, other: Self, t: Self) -> Self {{
                    self.lerp_unclamped(other, t.clamp(0.0, 1.0))
                }}

                #[inline(always)]
                fn lerp_weighted(self, other: Self, t: Self) -> Self {{
                    self.lerp_unclamped_weighted(other, t.clamp(0.0, 1.0))
                }}

                #[inline(always)]
                fn lerp_unclamped(self, other: Self, t: Self) -> Self {{
                    self + (other - self) * t
                }}

                #[inline(always)]
                fn lerp_unclamped_weighted(self, other: Self, t: Self) -> Self {{
                    self * (1.0 - t) + other * t
                }}

                #[inline(always)]
                fn move_towards(self, target: Self, max_delta: Self) -> Self {{
                    let delta = target - self;
                    let step = delta.clamp(-max_delta, max_delta);
                    self + step
                }}
            }}
        "#});
    }

    let impls = impls.join("\n");

    module.finish(formatdoc! {r#"
        /// A trait that adds additional functions to float types.
        pub trait FloatExt {{
            /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,
            /// which is clamped to the range `[0.0, 1.0]`.
            /// 
            /// This function uses the "delta lerp" formula which is:
            /// `a + (b - a) * t`
            /// 
            /// This formula is more numerically stable and is usually faster than the "weighted lerp" formula:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// The other formula can be used by calling `lerp_weighted`.
            /// It is useful when interpolating large values that are very far away from each other.
            fn lerp(self, other: Self, t: Self) -> Self;

            /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,
            /// which is clamped to the range `[0.0, 1.0]`.
            /// 
            /// This function uses the "weighted lerp" formula which is:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// This formula is usually worse than the "delta lerp" formula:
            /// `a + (b - a) * t`
            /// 
            /// This "weighted" formula is useful when interpolating large values that are very far away from each other.
            fn lerp_weighted(self, other: Self, t: Self) -> Self;

            /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`.
            /// If `t` is outside the range `[0.0, 1.0]`, the result is linearly extrapolated.
            /// 
            /// This function uses the "delta lerp" formula which is:
            /// `a + (b - a) * t`
            /// 
            /// This formula is more numerically stable and is usually faster than the "weighted lerp" formula:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// The other formula can be used by calling `lerp_weighted`.
            /// It is useful when interpolating large values that are very far away from each other.
            fn lerp_unclamped(self, other: Self, t: Self) -> Self;

            /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`.
            /// If `t` is outside the range `[0.0, 1.0]`, the result is linearly extrapolated.
            /// 
            /// This function uses the "weighted lerp" formula which is:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// This formula is usually worse than the "delta lerp" formula:
            /// `a + (b - a) * t`
            /// 
            /// This "weighted" formula is useful when interpolating large values that are very far away from each other.
            fn lerp_unclamped_weighted(self, other: Self, t: Self) -> Self;

            /// Moves `self` towards `target` by at most `max_delta`.
            fn move_towards(self, other: Self, max_delta: Self) -> Self;
        }}

        {impls}
    "#});
}
