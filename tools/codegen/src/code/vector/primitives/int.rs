use genco::quote;
use strum::IntoEnumIterator;

use crate::{
    code::vector::primitives::{PrimitiveSrcMod, PrimitiveTestMod},
    iter::{Length, PrimitiveFloat, PrimitiveInt},
};

pub fn push_src(primitive: PrimitiveInt, output: &mut PrimitiveSrcMod) {
    output.impl_items.push(quote! {
        $("// The following code is generated for all int primitives")

        $("/// A vector of all minimum values.")
        pub const MIN: Self = Self::const_splat($primitive::MIN);
        $("/// A vector of all maximum values.")
        pub const MAX: Self = Self::const_splat($primitive::MAX);

        $(
            for primitive2 in PrimitiveFloat::iter() join($['\n']) =>

            $(format!("/// Converts `self` to a vector of `{primitive2}` elements."))
            #[inline(always)]
            pub fn as_$(primitive2)(self) -> Vector<N, $primitive2, S> {
                self.map(|x| x as $primitive2)
            }
        )

        $(
            for primitive2 in PrimitiveInt::iter().filter(|&primitive2| primitive2 != primitive) join($['\n']) =>

            $(format!("/// Converts `self` to a vector of `{primitive2}` elements."))
            #[inline(always)]
            pub fn as_$(primitive2)(self) -> Vector<N, $primitive2, S> {
                self.map(|x| x as $primitive2)
            }
        )

        $("/// Returns `-self` or `None` if there is an overflow.")
        #[inline(always)]
        pub fn checked_neg(self) -> Option<Self> {
            self.map(|x| x.checked_neg()).flatten()
        }

        $("/// Returns `self + other` or `None` if there is an overflow.")
        #[inline(always)]
        pub fn checked_add(self, other: Self) -> Option<Self> {
            Vector::from_fn(|i| self.index(i).checked_add(other.index(i))).flatten()
        }

        $("/// Returns `self - other` or `None` if there is an overflow.")
        #[inline(always)]
        pub fn checked_sub(self, other: Self) -> Option<Self> {
            Vector::from_fn(|i| self.index(i).checked_sub(other.index(i))).flatten()
        }

        $("/// Returns `self * other` or `None` if there is an overflow.")
        #[inline(always)]
        pub fn checked_mul(self, other: Self) -> Option<Self> {
            Vector::from_fn(|i| self.index(i).checked_mul(other.index(i))).flatten()
        }

        $("/// Returns `self / other` or `None` if there is an overflow.")
        #[inline(always)]
        pub fn checked_div(self, other: Self) -> Option<Self> {
            Vector::from_fn(|i| self.index(i).checked_div(other.index(i))).flatten()
        }

        $("/// Returns `self % other` or `None` if there is an overflow.")
        #[inline(always)]
        pub fn checked_rem(self, other: Self) -> Option<Self> {
            Vector::from_fn(|i| self.index(i).checked_rem(other.index(i))).flatten()
        }

        $("/// Returns `-self` with wrapping arithmetic.")
        #[inline(always)]
        pub fn wrapping_neg(self) -> Self {
            self.map(|x| x.wrapping_neg())
        }

        $("/// Returns `self + other` with wrapping arithmetic.")
        #[inline(always)]
        pub fn wrapping_add(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).wrapping_add(other.index(i)))
        }

        $("/// Returns `self - other` with wrapping arithmetic.")
        #[inline(always)]
        pub fn wrapping_sub(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).wrapping_sub(other.index(i)))
        }

        $("/// Returns `self * other` with wrapping arithmetic.")
        #[inline(always)]
        pub fn wrapping_mul(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).wrapping_mul(other.index(i)))
        }

        $("/// Returns `self / other` with wrapping arithmetic.")
        #[inline(always)]
        pub fn wrapping_div(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).wrapping_div(other.index(i)))
        }

        $("/// Returns `self % other` with wrapping arithmetic.")
        #[inline(always)]
        pub fn wrapping_rem(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).wrapping_rem(other.index(i)))
        }

        $("/// Returns `self + other` with saturating arithmetic.")
        #[inline(always)]
        pub fn saturating_add(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).saturating_add(other.index(i)))
        }

        $("/// Returns `self - other` with saturating arithmetic.")
        #[inline(always)]
        pub fn saturating_sub(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).saturating_sub(other.index(i)))
        }

        $("/// Returns `self * other` with saturating arithmetic.")
        #[inline(always)]
        pub fn saturating_mul(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).saturating_mul(other.index(i)))
        }

        $("/// Returns `self / other` with saturating arithmetic.")
        #[inline(always)]
        pub fn saturating_div(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).saturating_div(other.index(i)))
        }

        $("/// Returns a vector of the minimum of each element between `self` and `other`.")
        #[inline(always)]
        pub fn min(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).min(other.index(i)))
        }

        $("/// Returns a vector of the maximum of each element between `self` and `other`.")
        #[inline(always)]
        pub fn max(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).max(other.index(i)))
        }

        $("/// Returns a vector with each element clamped between `min` and `max`.")
        #[inline(always)]
        pub fn clamp(self, min: Self, max: Self) -> Self {
            self.max(min).min(max)
        }

        $("/// Returns the minimum element in the vector.")
        #[inline(always)]
        pub fn min_element(self) -> $primitive {
            self.reduce(|a, b| a.min(b))
        }

        $("/// Returns the maximum element in the vector.")
        #[inline(always)]
        pub fn max_element(self) -> $primitive {
            self.reduce(|a, b| a.max(b))
        }
    });

    output.trait_impls.push(quote! {
        impl ScalarZero for $primitive {
            const ZERO: Self = 0;

            $(
                for n in Length::iter() join($['\r']) =>

                const VEC$(n)_ZERO: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([0; $n]);
            )
        }

        impl ScalarOne for $primitive {
            const ONE: Self = 1;

            $(
                for n in Length::iter() join($['\r']) =>

                const VEC$(n)_ONE: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([1; $n]);
            )

            $(
                for n in Length::iter() join($['\n']) => $(
                    for axis in n.axes() join($['\r']) =>

                    const VEC$(n)_$(axis.uppercase_str()): Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([$(
                        for axis2 in n.axes() join(, ) => $(if axis2 == axis { 1 } else { 0 })
                    )]);
                )
            )
        }
    });
}

pub fn push_tests(_primitive: PrimitiveInt, _output: &mut PrimitiveTestMod) {}
