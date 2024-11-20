use std::hash::{DefaultHasher, Hash, Hasher};

use super::*;

use quote::quote;
use syn::{token::Paren, Type};

pub fn inner_vectors(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        ty: Type,
        #[paren]
        _paren_token: Paren,
        #[inside(_paren_token)]
        size: Literal,
    }
    let Input {
        ty,
        _paren_token,
        size,
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

    let ty_size_assert_errm = format!(
        "the provided size for {}: {size} bytes, is not its size",
        ty.to_token_stream()
    );

    let mod_ident = Ident::new(
        &format!("inner_vecs_{}", {
            let mut state = DefaultHasher::default();
            ty.hash(&mut state);
            state.finish()
        }),
        Span::call_site(),
    );

    quote! {
        mod #mod_ident {
            use super::*;

            unsafe impl ggmath::scalar::ScalarInnerVectors for #ty {
                type InnerAlignedVec2 = InnerAlignedVec2;
                type InnerAlignedVec4 = InnerAlignedVec4;
            }

            #[repr(align(#vec2_align))]
            #[derive(Debug, Clone, Copy)]
            pub struct InnerAlignedVec2([#ty; 2]);

            #[repr(align(#vec4_align))]
            #[derive(Debug, Clone, Copy)]
            pub struct InnerAlignedVec4([#ty; 4]);

            const _: () = assert!(
                size_of::<#ty>() == #size,
                #ty_size_assert_errm,
            );
            const _: () = assert!(
                size_of::<InnerAlignedVec2>() == #size * 2,
                "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
            );
            const _: () = assert!(
                align_of::<InnerAlignedVec2>() == #size * 2,
                "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
            );
            const _: () = assert!(
                size_of::<InnerAlignedVec4>() == #size * 4,
                "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
            );
            const _: () = assert!(
                align_of::<InnerAlignedVec4>() == #size * 4,
                "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
            );
        }
    }
    .into()
}
