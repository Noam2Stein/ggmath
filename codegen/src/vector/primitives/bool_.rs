use std::collections::HashMap;

use genco::{lang::rust::Tokens, quote};

use crate::constants::INT_PRIMITIVES;

pub fn push_src(
    _primitive: &str,
    _use_crate_items: &mut Vec<Tokens>,
    functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _trait_impls: &mut Vec<Tokens>,
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

        $("/// Returns a vector of `T2` elements where each element is `if_true` if the corresponding element in `self` is `true` and `if_false` if the corresponding element in `self` is `false`.")
        #[inline(always)]
        pub fn select<T2: Scalar>(self, if_true: Vector<N, T2, A>, if_false: Vector<N, T2, A>) -> Vector<N, T2, A> {
            Vector::from_fn(|i| if self.index(i) { if_true.index(i) } else { if_false.index(i) })
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

pub fn push_test(_primitive: &str, _use_stmts: &mut Vec<Tokens>, _functions: &mut Vec<Tokens>) {}
