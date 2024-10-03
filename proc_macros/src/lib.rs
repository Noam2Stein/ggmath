use proc_macro::TokenStream;

mod ops;
mod vec;

macro_rules! export {
    ($($path:ident)::* => $ident:ident) => {
        #[proc_macro]
        pub fn $ident(tokens: TokenStream) -> TokenStream {
            $($path)::*::$ident(tokens)
        }
    };
}

export!(vec::vecn::inner => impl_element_vec_inner);
export!(vec::vecn::const_swizzle::cget_mut => vec_cget_mut_wrappers);
export!(vec::vecn::const_swizzle::cget => vec_cget_wrappers);
export!(vec::vecn::const_swizzle::cset => vec_cset_wrappers);
export!(vec::vecn::const_swizzle::cwith => vec_cwith_wrappers);

export!(ops => self_ops);
export!(ops => rhs_ops);
export!(ops => assign_ops);
