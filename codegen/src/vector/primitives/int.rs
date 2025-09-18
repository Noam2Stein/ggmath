use std::collections::HashMap;

use genco::{lang::rust::Tokens, quote};

use crate::constants::{COMPONENTS, LENGTHS};

pub fn push_fns(
    primitive: &str,
    _use_crate_items: &mut Vec<Tokens>,
    functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    trait_impls: &mut Vec<Tokens>,
) {
    functions.push(quote! {
        $("// The following code is for all int primitives")

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
        pub fn min(self, other: Vector<N, $primitive, impl VecAlignment>) -> Self {
            Vector::from_fn(|i| self.index(i).min(other.index(i)))
        }

        $("/// Returns a vector of the maximum of each element between `self` and `other`.")
        #[inline(always)]
        pub fn max(self, other: Vector<N, $primitive, impl VecAlignment>) -> Self {
            Vector::from_fn(|i| self.index(i).max(other.index(i)))
        }

        $("/// Returns a vector with each element clamped between `min` and `max`.")
        #[inline(always)]
        pub fn clamp(self, min: Vector<N, $primitive, impl VecAlignment>, max: Vector<N, $primitive, impl VecAlignment>) -> Self {
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

    trait_impls.push(quote! {
        impl ScalarZero for $primitive {
            const ZERO: Self = 0;

            $(
                for &n in LENGTHS join($['\r']) =>

                const VEC$(n)_ZERO: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([0; $n]);
            )
        }

        impl ScalarOne for $primitive {
            const ONE: Self = 1;

            $(
                for &n in LENGTHS join($['\r']) =>

                const VEC$(n)_ONE: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([1; $n]);
            )

            $(
                for &n in LENGTHS join($['\n']) => $(
                    for i in 0..n join($['\r']) =>

                    $(let component = COMPONENTS[i].to_uppercase())

                    const VEC$(n)_$(component): Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([$(
                        for i2 in 0..n join(, ) => $(if i2 == i { 1 } else { 0 })
                    )]);
                )
            )
        }
    });
}
