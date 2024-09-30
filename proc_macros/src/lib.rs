use proc_macro::TokenStream;

mod element;

macro_rules! export {
    ($($path:ident)::* => $ident:ident) => {
        #[proc_macro]
        pub fn $ident(tokens: TokenStream) -> TokenStream {
            $($path)::*::$ident(tokens)
        }
    };
}

export!(element => impl_element_inner_vecs);
export!(element => impl_element_vecs_from_splits_transmute);
