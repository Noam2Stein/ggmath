use genco::quote;
use strum::IntoEnumIterator;

use crate::{
    code::vector::primitives::{PrimitiveSrcMod, PrimitiveTestMod},
    iter::{Axis, Length, PrimitiveSint, PrimitiveUint},
};

pub fn push_src(primitive: PrimitiveSint, output: &mut PrimitiveSrcMod) {
    let unsigned_primitive = match primitive {
        PrimitiveSint::I8 => PrimitiveUint::U8,
        PrimitiveSint::I16 => PrimitiveUint::U16,
        PrimitiveSint::I32 => PrimitiveUint::U32,
        PrimitiveSint::I64 => PrimitiveUint::U64,
        PrimitiveSint::I128 => PrimitiveUint::U128,
        PrimitiveSint::Isize => PrimitiveUint::Usize,
    };

    output.impl_items.push(quote! {
        $("// The following code is generated for all signed int primitives")

        $("/// Returns `-self` with saturating arithmetic.")
        #[inline(always)]
        pub fn saturating_neg(self) -> Self {
            Vector::from_fn(|i| self.index(i).saturating_neg())
        }

        $("/// Returns a vector containing the signum of each element of `self`.")
        $("/// Signum for each element is:")
        $("/// - `1` if the element is positive")
        $("/// - `-1` if the element is negative")
        $("/// - `0` if the element is zero")
        #[inline(always)]
        pub fn signum(self) -> Self {
            self.map(|x| x.signum())
        }

        $("/// Returns a vector containing the absolute value of each element of `self`.")
        #[inline(always)]
        pub fn abs(self) -> Self {
            self.map(|x| x.abs())
        }

        $("/// Returns a vector containing the absolute difference between each element of `self` and `other`.")
        #[inline(always)]
        pub fn abs_diff(self, other: Self) -> Vector<N, $unsigned_primitive, S> {
            Vector::from_fn(|i| self.index(i).abs_diff(other.index(i)))
        }

        $("/// Returns a vector containing the squared distance between each element of `self` and `other`.")
        #[inline(always)]
        pub fn distance_sq(self, other: Self) -> $unsigned_primitive {
            self.abs_diff(other).mag_sq()
        }
    });

    output.trait_impls.push(quote! {
        impl ScalarNegOne for $primitive {
            const NEG_ONE: Self = -1;

            $(
                for n in Length::iter() join($['\r']) =>

                const VEC$(n)_NEG_ONE: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([-1; $n]);
            )

            $(
                for n in Length::iter() join($['\n']) => $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    const VEC$(n)_NEG_$(Axis(i).uppercase_str()): Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([$(
                        for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { -1 } else { 0 })
                    )]);
                )
            )
        }
    });
}

pub fn push_tests(_primitive: PrimitiveSint, _output: &mut PrimitiveTestMod) {}
