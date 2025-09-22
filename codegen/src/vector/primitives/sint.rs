use std::collections::HashMap;

use genco::{lang::rust::Tokens, quote};

use crate::constants::{COMPONENTS, LENGTHS};

pub fn push_src(
    primitive: &str,
    use_crate_items: &mut Vec<Tokens>,
    functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    trait_impls: &mut Vec<Tokens>,
) {
    let unsigned_primitive = &"u"
        .chars()
        .chain(primitive.chars().skip(1))
        .collect::<String>();

    use_crate_items.push(quote! { ScalarNegOne });

    functions.push(quote! {
        $("// The following code is for all signed int primitives")

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

    trait_impls.push(quote! {
        impl ScalarNegOne for $primitive {
            const NEG_ONE: Self = -1;

            $(
                for &n in LENGTHS join($['\r']) =>

                const VEC$(n)_NEG_ONE: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([-1; $n]);
            )

            $(
                for &n in LENGTHS join($['\n']) => $(
                    for i in 0..n join($['\r']) =>

                    $(let component = COMPONENTS[i].to_uppercase())

                    const VEC$(n)_NEG_$(component): Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([$(
                        for i2 in 0..n join(, ) => $(if i2 == i { -1 } else { 0 })
                    )]);
                )
            )
        }
    });
}

pub fn push_tests(_n: usize, _primitive: &str, _is_simd: bool, _tests: &mut Vec<Tokens>) {}
