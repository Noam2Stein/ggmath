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

export!(element => impl_element);
export!(element::default_impl => impl_element_default);

export!(vec::from_split => impl_from_splits_transmute);