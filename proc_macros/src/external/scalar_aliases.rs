use super::*;

use quote::quote;
use syn::{token::Paren, Type, Visibility};

#[inline(always)]
pub fn vector_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(

        "[```Vector```] type aliases for [```***T***```].
        ```***Prefix***Vec2```, ```***Prefix***Vec3P```...",
        quote! { ggmath::vector::* },
        &[
            ("***Shorthand***Vec2", "Type alias to [```Vector<2, ***T***, VecAligned>```]"),
            ("***Shorthand***Vec3", "Type alias to [```Vector<3, ***T***, VecAligned>```]"),
            ("***Shorthand***Vec4", "Type alias to [```Vector<4, ***T***, VecAligned>```]"),
            ("***Shorthand***Vec2P",
            "Type alias to [```Vector<2, ***T***, VecAligned>```].
            If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Shorthand***Vec2```]."),
            ("***Shorthand***Vec3P",
            "Type alias to [```Vector<3, ***T***, VecAligned>```].
            If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Shorthand***Vec3```]."),
            ("***Shorthand***Vec4P",
            "Type alias to [```Vector<4, ***T***, VecAligned>```].
            If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Shorthand***Vec4```]."),
        ],
        input,
    )
}
#[inline(always)]
pub fn matrix_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        "Matrix",
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
        "Rectangle",
        quote! { ggmath::rectangle::* },
        &[
            "Rect2", "Rect3", "Rect4", "Rect2C", "Rect3C", "Rect4C", "Rect2M", "Rect3M", "Rect4M",
            "Rect2P", "Rect3P", "Rect4P", "Rect2CP", "Rect3CP", "Rect4CP", "Rect2MP", "Rect3MP",
            "Rect4MP",
        ],
        input,
    )
}

macro_rules! aliases {
    ($($ident:literal $(($doc:literal))? => $value:literal), * $(,)?) => {
        [$(Alias {
            ident: $ident,
            $(doc: Some($doc),)*
        })*]
    };
}

struct Alias {
    ident: &'static str,
    doc: Option<&'static str>,
    value: &'static str,
}

fn scalar_aliases(
    mod_doc: &str,
    use_items: TokenStream,
    aliases: &[(&str, &str)],
    input: TokenStream1,
) -> TokenStream1 {
    let Input {
        vis,
        mod_ident,
        scalar_type,
        _t_paren,
        shorthand,
    } = parse_macro_input!(input as Input);

    let scalar_type_str = scalar_type.to_token_stream().to_string();
    let shorthand_str = shorthand.to_string();

    let mod_doc = mod_doc
        .replace("***T***", &scalar_type_str)
        .replace("***Shorthand***", &shorthand_str);

    let aliases_without_prefix = aliases
        .iter()
        .map(|(alias, _)| Ident::new(&alias, Span::call_site()));
    let aliases_with_prefix = aliases
        .iter()
        .map(|(alias, _)| Ident::new(&format!("{shorthand}{alias}"), Span::call_site()));
    let alias_docs = aliases
        .iter()
        .map(|(_, alias)| format!("type alias to [```Vector<3, {T}, VecAligned>```]"));

    quote! {
        #[doc = #mod_doc]
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
    shorthand: Ident,
}
