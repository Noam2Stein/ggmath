use quote::quote_spanned;
use syn::{parse_macro_input, Ident};

pub fn rhs_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    quote_spanned! {
        macro_ident.span() =>
        #macro_ident!(ElementAdd(vec2_add, vec3_add, vec4_add): Add(add));
        #macro_ident!(ElementSub(vec2_sub, vec3_sub, vec4_sub): Sub(sub));
        #macro_ident!(ElementMul(vec2_mul, vec3_mul, vec4_mul): Mul(mul));
        #macro_ident!(ElementDiv(vec2_div, vec3_div, vec4_div): Div(div));
        #macro_ident!(ElementRem(vec2_rem, vec3_rem, vec4_rem): Rem(rem));
        #macro_ident!(ElementBitAnd(vec2_bitand, vec3_bitand, vec4_bitand): BitAnd(bitand));
        #macro_ident!(ElementBitOr(vec2_bitor, vec3_bitor, vec4_bitor): BitOr(bitor));
        #macro_ident!(ElementBitXor(vec2_bitxor, vec3_bitxor, vec4_bitxor): BitXor(bitxor));
    }
    .into()
}
