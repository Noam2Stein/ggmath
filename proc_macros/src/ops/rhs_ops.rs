use proc_macro2::Span;
use quote::quote_spanned;
use syn::{parse_macro_input, Ident};

use super::{op_fragments, RHS_OPS};

pub fn rhs_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    let op_fragments = RHS_OPS.into_iter().map(|op| {
        op_fragments(
            Ident::new(&format!("Element{op}"), Span::call_site()),
            Ident::new(op, Span::call_site()),
            Ident::new(&op.to_lowercase(), Span::call_site()),
            Ident::new(&format!("ElementVec{op}"), Span::call_site()),
            Ident::new(&format!("vec_{}", op.to_lowercase()), Span::call_site()),
            Ident::new(&format!("VecNum{op}"), Span::call_site()),
        )
    });

    quote_spanned! {
        macro_ident.span() =>
        #macro_ident!(#(#op_fragments), *);
    }
    .into()
}
