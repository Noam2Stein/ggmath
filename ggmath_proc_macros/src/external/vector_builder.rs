use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Expr, LitInt};

use super::*;

macro_rules! vector_builder_macro {
    ($ident:ident($n:literal, $a:ident)) => {
        pub fn $ident(input: TokenStream1) -> TokenStream1 {
            vector_builder(parse_quote!($n), parse_quote!($a), input)
        }
    };
}

vector_builder_macro!(vec2(2, VecAligned));
vector_builder_macro!(vec3(3, VecAligned));
vector_builder_macro!(vec4(4, VecAligned));
vector_builder_macro!(vec2p(2, VecPacked));
vector_builder_macro!(vec3p(3, VecPacked));
vector_builder_macro!(vec4p(4, VecPacked));

fn vector_builder(n: LitInt, a: Ident, input: TokenStream1) -> TokenStream1 {
    let Input { fields } = parse_macro_input!(input);
    quote! {
        ggmath::vector::build_vector::<#n, _, ggmath::vector::#a>((#fields))
    }
    .into()
}

#[derive(Parse)]
struct Input {
    #[call(Punctuated::parse_terminated)]
    fields: Punctuated<Expr, Token![,]>,
}
