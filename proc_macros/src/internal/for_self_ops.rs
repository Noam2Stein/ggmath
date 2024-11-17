use super::*;

use quote::quote;

const SELF_OPS: &[&str] = &["Not", "Neg"];

pub fn for_self_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let ops = SELF_OPS.into_iter().map(|op_str| {
        let std_trait = Ident::new(op_str, Span::call_site());
        let std_fn = Ident::new(&op_str.to_lowercase(), Span::call_site());
        let scalar_trait = Ident::new(&format!("Scalar{op_str}"), Span::call_site());

        quote! {
            #std_trait #std_fn #scalar_trait
        }
    });

    quote! {
        macro_rules! some_self_ops_macro {
            ($($std_trait:ident $std_fn:ident $scalar_trait:ident)*) => { #input }
        }
        some_self_ops_macro!(#(#ops)*);
    }
    .into()
}
