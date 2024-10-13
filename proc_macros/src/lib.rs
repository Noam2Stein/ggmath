macro_rules! export {
    ($ident:ident in $($path:ident)::*) => {
        #[proc_macro]
        pub fn $ident(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($path::)*$ident::$ident(tokens)
        }
    };
}

mod vec;

export!(vec_api in vec);
