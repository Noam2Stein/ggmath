use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Fields};

use super::*;

pub fn derive_wrapper_scalar(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);

    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return Error::new(Span::call_site(), "expected a struct")
            .to_compile_error()
            .into();
    };

    let fields = if let Fields::Unnamed(fields) = data.fields {
        fields
    } else {
        return Error::new(Span::call_site(), "expected (), found {}")
            .to_compile_error()
            .into();
    };

    let field = if fields.unnamed.len() == 1 {
        fields.unnamed.into_iter().next().unwrap()
    } else {
        return Error::new(Span::call_site(), "expected (), found {}")
            .to_compile_error()
            .into();
    };

    let repr_transparent_error = if !input.attrs.contains(&parse_quote!(#[repr(transparent)])) {
        Some(Error::new(Span::call_site(), "expected #[repr(transparent)]").to_compile_error())
    } else {
        None
    };

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let field_ty = field.ty;

    quote! {
        unsafe impl #impl_generics ggmath::scalar::WrapperScalar for #ident #ty_generics #where_clause {
            type InnerScalar = #field_ty;
        }

        #repr_transparent_error
    }
    .into()
}
