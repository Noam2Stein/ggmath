use super::*;

use quote::quote;
use syn::{token::Paren, Type, Visibility};

#[inline(always)]
pub fn vector_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        quote! { ggmath::vector::* },
        &["Vec2", "Vec3", "Vec4", "Vec2P", "Vec3P", "Vec4P"],
        input,
    )
}
#[inline(always)]
pub fn matrix_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        quote! { ggmath::matrix::* },
        &[
            "Mat2C", "Mat2x3C", "Mat2x4C", "Mat3x2C", "Mat3C", "Mat3x4C", "Mat4x2C", "Mat4x3C",
            "Mat4C", "Mat2R", "Mat2x3R", "Mat2x4R", "Mat3x2R", "Mat3R", "Mat3x4R", "Mat4x2R",
            "Mat4x3R", "Mat4R", "Mat2CP", "Mat2x3CP", "Mat2x4CP", "Mat3x2CP", "Mat3CP", "Mat3x4CP",
            "Mat4x2CP", "Mat4x3CP", "Mat4CP", "Mat2RP", "Mat2x3RP", "Mat2x4RP", "Mat3x2RP",
            "Mat3RP", "Mat3x4RP", "Mat4x2RP", "Mat4x3RP", "Mat4RP",
        ],
        input,
    )
}
#[inline(always)]
pub fn rectangle_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        quote! { ggmath::rectangle::* },
        &[
            "Rect2", "Rect3", "Rect4", "Rect2C", "Rect3C", "Rect4C", "Rect2M", "Rect3M", "Rect4M",
            "Rect2P", "Rect3P", "Rect4P", "Rect2CP", "Rect3CP", "Rect4CP", "Rect2MP", "Rect3MP",
            "Rect4MP",
        ],
        input,
    )
}

fn scalar_aliases(use_items: TokenStream, aliases: &[&str], input: TokenStream1) -> TokenStream1 {
    let Input {
        vis,
        mod_ident,
        scalar_type,
        _t_paren,
        prefix,
    } = parse_macro_input!(input as Input);

    let aliases_without_prefix = aliases
        .iter()
        .map(|alias| Ident::new(&alias, Span::call_site()));
    let aliases_with_prefix = aliases
        .iter()
        .map(|alias| Ident::new(&format!("{prefix}{alias}"), Span::call_site()));

    let mod_docs: TokenStream = parse_str(&format!(
        "/// mathamatical type-aliases for [```{}```]",
        scalar_type.to_token_stream()
    ))
    .unwrap_or_else(|err| panic!("failed to parse mod docs: {err}"));

    quote! {
        #mod_docs
        #vis mod #mod_ident {
            use super::*;
            use #use_items;

            #(
                pub type #aliases_with_prefix = #aliases_without_prefix<#scalar_type>;
            )*
        }
    }
    .into()
}

#[derive(Parse)]
struct Input {
    vis: Visibility,
    #[prefix(Token![mod])]
    mod_ident: Ident,
    #[prefix(Token![for])]
    scalar_type: Type,
    #[paren]
    _t_paren: Paren,
    #[inside(_t_paren)]
    prefix: Ident,
}
