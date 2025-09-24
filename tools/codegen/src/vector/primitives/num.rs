use std::collections::HashMap;

use genco::{lang::rust::Tokens, quote};

use crate::constants::NUM_PRIMITIVES;

pub fn push_src(
    primitive: &str,
    use_crate_items: &mut Vec<Tokens>,
    functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _trait_impls: &mut Vec<Tokens>,
) {
    use_crate_items.push(quote! { ScalarZero, ScalarOne });

    functions.push(quote! {
        $("// The following code is generated for all number primitives")

        $("/// A vector of all minimum values.")
        pub const MIN: Self = Self::const_splat($primitive::MIN);
        $("/// A vector of all maximum values.")
        pub const MAX: Self = Self::const_splat($primitive::MAX);

        $(
            for &primitive2 in NUM_PRIMITIVES.iter().filter(|&&primitive2| primitive2 != primitive) join($['\n']) =>

            $(format!("/// Converts `self` to a vector of `{primitive2}` elements."))
            #[inline(always)]
            pub fn as_$(primitive2)(self) -> Vector<N, $primitive2, S> {
                self.map(|x| x as $primitive2)
            }
        )
    });
}

pub fn push_tests(_n: usize, _primitive: &str, _is_simd: bool, _tests: &mut Vec<Tokens>) {}
