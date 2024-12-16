use super::*;

use quote::quote;
use syn::{token::Paren, Type, Visibility};

macro_rules! aliases {
    ($($ident:ident $(($doc:literal))?), * $(,)?) => {
        &[$(Alias {
            ident: stringify!($ident),
            $(doc: Some($doc),)*
            ..Default::default()
        },)*]
    };
}

#[inline(always)]
pub fn vector_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        "[```Vector```] type aliases for [```***T***```].
        ```***Prefix***Vec2```, ```***Prefix***Vec3P```...",
        quote! { ggmath::vector::* },
        aliases!(
            Vec2("Type alias to [```Vector<2, ***T***, VecAligned>```]"),
            Vec3("Type alias to [```Vector<3, ***T***, VecAligned>```]"),
            Vec4("Type alias to [```Vector<4, ***T***, VecAligned>```]"),
            Vec2P(
                "Type alias to [```Vector<2, ***T***, VecPacked>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Vec2```]."
            ),
            Vec3P(
                "Type alias to [```Vector<3, ***T***, VecPacked>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Vec3```]."
            ),
            Vec4P(
                "Type alias to [```Vector<4, ***T***, VecPacked>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Vec4```]."
            ),
        ),
        input,
    )
}

#[inline(always)]
pub fn matrix_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        "[```Matrix```] type aliases for [```***T***```].
        ```***Prefix***Mat4C```, ```***Prefix***Mat3RP```...",
        quote! { ggmath::matrix::* },
        aliases!(
            Mat2C("Type alias to [```Matrix<2, 2, ***T***, VecAligned, ColumnMajor>```]."),
            Mat2x3C("Type alias to [```Matrix<2, 3, ***T***, VecAligned, ColumnMajor>```]."),
            Mat2x4C("Type alias to [```Matrix<2, 4, ***T***, VecAligned, ColumnMajor>```]."),
            Mat3x2C("Type alias to [```Matrix<3, 2, ***T***, VecAligned, ColumnMajor>```]."),
            Mat3C("Type alias to [```Matrix<3, 3, ***T***, VecAligned, ColumnMajor>```]."),
            Mat3x4C("Type alias to [```Matrix<3, 4, ***T***, VecAligned, ColumnMajor>```]."),
            Mat4x2C("Type alias to [```Matrix<4, 2, ***T***, VecAligned, ColumnMajor>```]."),
            Mat4x3C("Type alias to [```Matrix<4, 3, ***T***, VecAligned, ColumnMajor>```]."),
            Mat4C("Type alias to [```Matrix<4, 4, ***T***, VecAligned, ColumnMajor>```]."),
            Mat2R("Type alias to [```Matrix<2, 2, ***T***, VecAligned, RowMajor>```]."),
            Mat2x3R("Type alias to [```Matrix<2, 3, ***T***, VecAligned, RowMajor>```]."),
            Mat2x4R("Type alias to [```Matrix<2, 4, ***T***, VecAligned, RowMajor>```]."),
            Mat3x2R("Type alias to [```Matrix<3, 2, ***T***, VecAligned, RowMajor>```]."),
            Mat3R("Type alias to [```Matrix<3, 3, ***T***, VecAligned, RowMajor>```]."),
            Mat3x4R("Type alias to [```Matrix<3, 4, ***T***, VecAligned, RowMajor>```]."),
            Mat4x2R("Type alias to [```Matrix<4, 2, ***T***, VecAligned, RowMajor>```]."),
            Mat4x3R("Type alias to [```Matrix<4, 3, ***T***, VecAligned, RowMajor>```]."),
            Mat4R("Type alias to [```Matrix<4, 4, ***T***, VecAligned, RowMajor>```]."),
            Mat2CP(
                "Type alias to [```Matrix<2, 2, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat2C```]."
            ),
            Mat2x3CP(
                "Type alias to [```Matrix<2, 3, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat2x3C```]."
            ),
            Mat2x4CP(
                "Type alias to [```Matrix<2, 4, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat2x4C```]."
            ),
            Mat3x2CP(
                "Type alias to [```Matrix<3, 2, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat3x2C```]."
            ),
            Mat3CP(
                "Type alias to [```Matrix<3, 3, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat3C```]."
            ),
            Mat3x4CP(
                "Type alias to [```Matrix<3, 4, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat3x4C```]."
            ),
            Mat4x2CP(
                "Type alias to [```Matrix<4, 2, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat4x2C```]."
            ),
            Mat4x3CP(
                "Type alias to [```Matrix<4, 3, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat4x3C```]."
            ),
            Mat4CP(
                "Type alias to [```Matrix<4, 4, T, VecPacked, ColumnMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat4C```]."
            ),
            Mat2RP(
                "Type alias to [```Matrix<2, 2, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat2R```]."
            ),
            Mat2x3RP(
                "Type alias to [```Matrix<2, 3, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat2x3R```]."
            ),
            Mat2x4RP(
                "Type alias to [```Matrix<2, 4, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat2x4R```]."
            ),
            Mat3x2RP(
                "Type alias to [```Matrix<3, 2, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat3x2R```]."
            ),
            Mat3RP(
                "Type alias to [```Matrix<3, 3, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat3R```]."
            ),
            Mat3x4RP(
                "Type alias to [```Matrix<3, 4, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat3x4R```]."
            ),
            Mat4x2RP(
                "Type alias to [```Matrix<4, 2, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat4x2R```]."
            ),
            Mat4x3RP(
                "Type alias to [```Matrix<4, 3, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat4x3R```]."
            ),
            Mat4RP(
                "Type alias to [```Matrix<4, 4, T, VecPacked, RowMajor>```].
                If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```***Prefix***Mat4R```]."
            ),
        ),
        input,
    )
}

#[inline(always)]
pub fn rectangle_aliases(input: TokenStream1) -> TokenStream1 {
    scalar_aliases(
        "Rectangle",
        quote! { ggmath::rectangle::* },
        aliases!(
            Rect2, Rect3, Rect4, Rect2C, Rect3C, Rect4C, Rect2M, Rect3M, Rect4M, Rect2P, Rect3P,
            Rect4P, Rect2CP, Rect3CP, Rect4CP, Rect2MP, Rect3MP, Rect4MP,
        ),
        input,
    )
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Alias {
    ident: &'static str,
    doc: Option<&'static str>,
}

fn scalar_aliases(
    mod_doc: &str,
    use_items: TokenStream,
    aliases: &[Alias],
    input: TokenStream1,
) -> TokenStream1 {
    let Input {
        vis,
        mod_ident,
        scalar_type,
        _t_paren,
        prefix,
    } = parse_macro_input!(input as Input);

    let scalar_type_str = scalar_type.to_token_stream().to_string();
    let prefix_str = prefix.to_string();

    let mod_doc = mod_doc
        .replace("***T***", &scalar_type_str)
        .replace("***Prefix***", &prefix_str);

    let aliases_without_prefix = aliases
        .iter()
        .map(|alias| Ident::new(&alias.ident, Span::call_site()));

    let aliases_with_prefix = aliases
        .iter()
        .map(|alias| Ident::new(&format!("{prefix_str}{}", alias.ident), Span::call_site()));

    let alias_docs = aliases.iter().map(|alias| {
        alias.doc.map(|alias_doc| {
            let alias_doc = alias_doc
                .replace("***T***", &scalar_type_str)
                .replace("***Prefix***", &prefix_str);

            quote! { #[doc = #alias_doc] }
        })
    });

    quote! {
        #[doc = #mod_doc]
        #vis mod #mod_ident {
            use super::*;
            use #use_items;

            #(
                #alias_docs
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
