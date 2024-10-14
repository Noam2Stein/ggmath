macro_rules! export {
    ($ident:ident in $($path:ident)::*) => {
        #[proc_macro]
        pub fn $ident(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($path::)*$ident::$ident(tokens)
        }
    };
}

mod scalar;
mod vec;

export!(scalar_default_impl in scalar);
export!(scalar_aliases in scalar);

export!(vec_api in vec);
export!(aligned_vecs in vec);
