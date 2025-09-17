use genco::{lang::rust::Tokens, quote};

use crate::constants::INT_PRIMITIVES;

pub fn push_fns(
    _primitive: &str,
    functions: &mut Vec<Tokens>,
    _std_functions: &mut Vec<Tokens>,
    _trait_impls: &mut Vec<Tokens>,
    _use_crate_items: &mut Vec<Tokens>,
) {
    functions.push(quote! {
        $("// The following code is only for bool")

        $("/// All `true`.")
        pub const TRUE: Self = Self::const_splat(true);
        $("/// All `false`.")
        pub const FALSE: Self = Self::const_splat(false);

        $("/// Returns `true` if all elements are `true`.")
        #[inline(always)]
        pub fn all_true(self) -> bool {
            self.all(|x| x)
        }

        $("/// Returns `true` if any element is `true`.")
        #[inline(always)]
        pub fn any_true(self) -> bool {
            self.any(|x| x)
        }

        $("/// Returns `true` if all elements are `false`.")
        #[inline(always)]
        pub fn all_false(self) -> bool {
            self.all(|x| !x)
        }

        $("/// Returns `true` if any element is `false`.")
        #[inline(always)]
        pub fn any_false(self) -> bool {
            self.any(|x| !x)
        }

        $("/// Returns the number of `true` elements.")
        #[inline(always)]
        pub fn count_true(self) -> usize {
            self.count(|x| x)
        }

        $("/// Returns the number of `false` elements.")
        #[inline(always)]
        pub fn count_false(self) -> usize {
            self.count(|x| !x)
        }

        $(
            for &primitive2 in INT_PRIMITIVES join($['\n']) =>

            $(format!("/// Converts `self` to a vector of `{primitive2}` elements."))
            #[inline(always)]
            pub fn as_$(primitive2)(self) -> Vector<N, $primitive2, A> {
                self.map(|x| x as $primitive2)
            }
        )
    });
}
