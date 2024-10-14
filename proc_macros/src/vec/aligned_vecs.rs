use derive_syn_parse::Parse;
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, token::Paren, Attribute, Error, Ident, Token, Type};

pub fn aligned_vecs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        _lt,
        ty,
        _gt,
        _paren,
        size,
        _colon,
        attrs,
        vec2,
        _comma,
        vec4,
    } = parse_macro_input!(input as Input);

    let size = match size.to_string().parse::<usize>() {
        Ok(ok) => ok,
        Err(_) => {
            return Error::new_spanned(size, "expected a usize")
                .into_compile_error()
                .into()
        }
    };

    let vec2_align = Literal::usize_unsuffixed(size * 2);
    let vec4_align = Literal::usize_unsuffixed(size * 4);
    let size = Literal::usize_unsuffixed(size);

    let ty_assert_errm = format!(
        "the provided size for {}: {size} bytes, is not its size",
        ty.to_token_stream()
    );

    quote! {
        unsafe impl gomath::vec::ScalarAlignedVecs for #ty {
            type InnerAlignedVec2 = #vec2;
            type InnerAlignedVec4 = #vec4;
        }

        #[repr(align(#vec2_align))]
        #(#attrs)*
        pub struct #vec2([#ty; 2]);

        #[repr(align(#vec4_align))]
        #(#attrs)*
        pub struct #vec4([#ty; 4]);

        const _: () = assert!(
            size_of::<#ty>() == #size,
            #ty_assert_errm,
        );
        const _: () = assert!(
            size_of::<#vec2>() == #size * 2,
            "requirement not met: size_of<AlignedVec2> == size_of<T> * 2",
        );
        const _: () = assert!(
            align_of::<#vec2>() == #size * 2,
            "requirement not met: align_of<AlignedVec2> == size_of<T> * 2",
        );
        const _: () = assert!(
            size_of::<#vec4>() == #size * 4,
            "requirement not met: size_of<AlignedVec4> == size_of<T> * 4",
        );
        const _: () = assert!(
            align_of::<#vec4>() == #size * 4,
            "requirement not met: align_of<AlignedVec4> == size_of<T> * 4",
        );
    }
    .into()
}

#[derive(Parse)]
struct Input {
    _lt: Token![<],
    ty: Type,
    _gt: Token![>],
    #[paren]
    _paren: Paren,
    #[inside(_paren)]
    size: Literal,
    _colon: Token![:],
    #[call(Attribute::parse_outer)]
    attrs: Vec<Attribute>,
    vec2: Ident,
    _comma: Token![,],
    vec4: Ident,
}
