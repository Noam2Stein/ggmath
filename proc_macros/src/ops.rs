use derive_syn_parse::Parse;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, Ident};

const SELF_OPS: &[&'static str] = &["Neg", "Not"];
const RHS_OPS: &[&'static str] = &[
    "Add", "Sub", "Mul", "Div", "Rem", "BitAnd", "BitOr", "BitXor", "Shr", "Shl",
];

pub fn ops_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        macro_ident: Ident,
        macro_output: TokenTree,
    }

    let Input {
        macro_ident,
        macro_output,
    } = parse_macro_input!(tokens as Input);

    let op_fragments = op_fragments(
        quote! { $element_trait:ident },
        quote! { $std_trait:ident },
        quote! { $std_fn:ident },
        quote! { $vec_trait:ident },
        quote! { $vec_fn:ident },
        quote! { $vecnum_trait:ident },
    );

    quote! {
        macro_rules! #macro_ident {
            ($(#op_fragments), *) => #macro_output
        }
    }
    .into()
}

pub fn self_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    let op_fragments = SELF_OPS.into_iter().map(|op| {
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

fn op_fragments(
    element_trait: impl ToTokens,
    std_trait: impl ToTokens,
    std_fn: impl ToTokens,
    vec_trait: impl ToTokens,
    vec_fn: impl ToTokens,
    vecnum_trait: impl ToTokens,
) -> TokenStream {
    quote! { #element_trait #std_trait #std_fn #vec_trait #vec_fn #vecnum_trait }
}
