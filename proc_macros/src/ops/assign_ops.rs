use proc_macro2::Span;
use quote::quote_spanned;
use syn::{parse_macro_input, Ident};

use super::{op_fragments, RHS_OPS};

pub fn assign_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    let op_fragments = RHS_OPS.into_iter().map(|op| {
        op_fragments(
            Ident::new(&format!("Element{op}Assign"), Span::call_site()),
            Ident::new(&format!("{op}Assign"), Span::call_site()),
            Ident::new(&format!("{}_assign", op.to_lowercase()), Span::call_site()),
            Ident::new(&format!("ElementVec{op}Assign"), Span::call_site()),
            Ident::new(
                &format!("vec_{}_assign", op.to_lowercase()),
                Span::call_site(),
            ),
            Ident::new(&format!("VecNum{op}Assign"), Span::call_site()),
        )
    });

    quote_spanned! {
        macro_ident.span() =>
        #macro_ident!(#(#op_fragments), *);
    }
    .into()
}
