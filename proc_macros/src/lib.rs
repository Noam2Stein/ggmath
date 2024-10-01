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
export!(vec::cget => impl_vec_cget_shortnames);
export!(vec::set => impl_vec_set_shortnames);
