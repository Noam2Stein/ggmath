use genco::quote;

use crate::{
    backend::{SrcFile, TokensExt},
    iter::PrimitiveFloat,
};

pub fn srcmod() -> SrcFile {
    quote! {
        $("/// A trait to extend the functionality of primitive float types.")
        pub trait FloatExt {
            $("/// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,")
            $("/// or linearly extrapolates if `t` is outside the range `[0.0, 1.0]`.")
            fn lerp(self, other: Self, t: Self) -> Self;

            $("/// Moves `self` towards `target` by at most `max_delta`.")
            fn move_towards(self, target: Self, max_delta: Self) -> Self;
        }

        $(
            for primitive in PrimitiveFloat::iter() =>

            impl FloatExt for $primitive {
                #[inline(always)]
                fn lerp(self, other: Self, t: Self) -> Self {
                    self + (other - self) * t
                }

                #[inline(always)]
                fn move_towards(self, target: Self, max_delta: Self) -> Self {
                    target.clamp(self - max_delta, self + max_delta)
                }
            }

            $['\n']
        )
    }
    .to_srcfile("float_ext")
}
