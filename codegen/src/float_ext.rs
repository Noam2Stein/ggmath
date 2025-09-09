use indoc::formatdoc;

use crate::{FLOAT_PRIMITIVES, module::Mod};

pub fn write_mod(module: Mod) {
    let mut free_functions = Vec::new();
    let mut impls = Vec::new();

    for &primitive in FLOAT_PRIMITIVES {
        free_functions.push(formatdoc! {r#"
            /// Interpolates between `a` and `b` based on the interpolation factor `t`,
            /// which is clamped to the range `[0.0, 1.0]`.
            /// 
            /// This function uses the "delta lerp" formula which is:
            /// `a + (b - a) * t`
            /// 
            /// This formula is more numerically stable and is usually faster than the "weighted lerp" formula:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// The other formula can be used by calling `const_{primitive}_lerp_weighted`.
            /// It is useful when interpolating large values that are very far away from each other.
            #[inline(always)]
            pub const fn const_{primitive}_lerp(a: {primitive}, b: {primitive}, t: {primitive}) -> {primitive} {{
                const_{primitive}_lerp_unclamped(a, b, t.clamp(0.0, 1.0))
            }}

            /// Interpolates between `a` and `b` based on the interpolation factor `t`,
            /// which is clamped to the range `[0.0, 1.0]`.
            /// 
            /// This function uses the "weighted lerp" formula which is:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// This formula is usually worse than the "delta lerp" formula:
            /// `a + (b - a) * t`
            /// 
            /// This "weighted" formula is useful when interpolating large values that are very far away from each other.
            #[inline(always)]
            pub const fn const_{primitive}_lerp_weighted(a: {primitive}, b: {primitive}, t: {primitive}) -> {primitive} {{
                const_{primitive}_lerp_unclamped_weighted(a, b, t.clamp(0.0, 1.0))
            }}

            /// Interpolates between `a` and `b` based on the interpolation factor `t`,
            /// or extrapolates if `t` is outside the range `[0.0, 1.0]`.
            /// 
            /// This function uses the "delta lerp" formula which is:
            /// `a + (b - a) * t`
            /// 
            /// This formula is more numerically stable and is usually faster than the "weighted lerp" formula:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// The other formula can be used by calling `const_{primitive}_lerp_unclamped_weighted`.
            /// It is useful when interpolating large values that are very far away from each other.
            #[inline(always)]
            pub const fn const_{primitive}_lerp_unclamped(a: {primitive}, b: {primitive}, t: {primitive}) -> {primitive} {{
                a + (b - a) * t
            }}

            /// Interpolates between `a` and `b` based on the interpolation factor `t`,
            /// or extrapolates if `t` is outside the range `[0.0, 1.0]`.
            /// 
            /// This function uses the "weighted lerp" formula which is:
            /// `a * (1.0 - t) + b * t`
            /// 
            /// This formula is usually worse than the "delta lerp" formula:
            /// `a + (b - a) * t`
            /// 
            /// This "weighted" formula is useful when interpolating large values that are very far away from each other.
            #[inline(always)]
            pub const fn const_{primitive}_lerp_unclamped_weighted(a: {primitive}, b: {primitive}, t: {primitive}) -> {primitive} {{
                a * (1.0 - t) + b * t
            }}
        "#});

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

    let free_functions = free_functions.join("\n");
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

        {free_functions}

        {impls}
    "#});
}
