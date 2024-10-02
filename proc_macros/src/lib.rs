use proc_macro::TokenStream;

mod element;
mod vec;

macro_rules! export {
    ($($path:ident)::* => $ident:ident) => {
        #[proc_macro]
        pub fn $ident(tokens: TokenStream) -> TokenStream {
            $($path)::*::$ident(tokens)
        }
    };
}

export!(element => impl_element_inner_vecs);

export!(vec::cget_mut => vec_cget_mut_wrappers);
export!(vec::cget => vec_cget_wrappers);
export!(vec::cset => vec_cset_wrappers);
export!(vec::cwith => vec_cwith_wrappers);
