use quote::quote;
use syn::{parse_macro_input, Type};

pub fn impl_element_default(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = parse_macro_input!(tokens as Type);

    quote! {
        impl ggmath::default_impl::ElementDefaultImpl for #ty {}

        const _: () = {
            const fn validate<T: ggmath::Element>() {}

            validate::<#ty>()
        };

        ggmath::impl_from_splits_transmute!(#ty);
    }.into()
}