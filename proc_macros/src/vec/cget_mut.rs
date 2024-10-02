use derive_syn_parse::Parse;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token, Type};

pub fn impl_vec_cget_mut_shortnames(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct SplitItem {
        space: usize,
        len: usize,
        s: String,
    }
    impl SplitItem {
        fn taken_space(&self) -> usize {
            self.space + self.len
        }
        fn output_ty(&self) -> TokenStream {
            match self.len {
                1 => quote! { &mut T },
                2 => quote! { &mut Vec2<T> },
                3 => quote! { &mut Vec3<T> },
                4 => quote! { &mut Vec4<T> },
                _ => panic!("invalid SplitItem len"),
            }
        }
        fn only_item_fn_call(&self) -> TokenStream {
            let fn_ident = Ident::new(
                &format!(
                    "cget_mut{}",
                    match self.len {
                        1 => "".to_string(),
                        len => len.to_string(),
                    }
                ),
                Span::call_site(),
            );
            let space = self.space;
            quote! {
                self.#fn_ident::<#space>()
            }
        }
    }
    fn push_fns(components: &Box<[Ident]>, split: &mut Vec<SplitItem>, fns: &mut TokenStream) {
        let left_len =
            components.len() - split.iter().map(|item| item.taken_space()).sum::<usize>();
        if left_len > 0 {
            for space in 0..left_len {
                for len in 1..left_len - space + 1 {
                    split.push(SplitItem {
                        space,
                        len,
                        s: components[components.len() - left_len + space
                            ..components.len() - left_len + space + len]
                            .iter()
                            .map(|c| c.to_string())
                            .collect(),
                    });

                    fns.extend({
                        let fn_ident = Ident::new(
                            &format!(
                                "{}_mut",
                                split
                                    .iter()
                                    .map(|item| item.s.as_str())
                                    .collect::<Box<[&str]>>()
                                    .join("_")
                            ),
                            Span::call_site(),
                        );
                        let fn_output_ty = match split.as_slice() {
                            [only_item] => only_item.output_ty(),
                            split => {
                                let item_output_tys = split.iter().map(|item| item.output_ty());
                                quote! {
                                    (#(#item_output_tys), *)
                                }
                            }
                        };
                        let fn_call = match split.as_slice() {
                            [only_item] => only_item.only_item_fn_call(),
                            split => {
                                let call_fn_ident = Ident::new(
                                    &format!(
                                        "cget_mut_{}",
                                        split
                                            .iter()
                                            .map(|item| item.len.to_string())
                                            .collect::<Box<[String]>>()
                                            .join("_")
                                    ),
                                    Span::call_site(),
                                );
                                let spaces = split.iter().map(|item| item.space);
                                quote! {
                                    self.#call_fn_ident::<#(#spaces), *>()
                                }
                            }
                        };

                        quote! {
                            #[inline(always)]
                            pub fn #fn_ident(&mut self) -> #fn_output_ty {
                                #fn_call
                            }
                        }
                    });

                    push_fns(components, split, fns);

                    split.pop();
                }
            }
        }
    }

    #[derive(Parse)]
    struct Input {
        t: Type,
        _0: Token![:],
        #[call(Punctuated::parse_terminated)]
        components: Punctuated<Ident, Token![,]>,
    }

    let Input { t, _0, components } = parse_macro_input!(tokens as Input);
    let components = components.into_iter().collect::<Box<[Ident]>>();

    let mut fns = TokenStream::new();
    push_fns(
        &components,
        &mut Vec::with_capacity(components.len()),
        &mut fns,
    );

    quote! {
        impl<T: Element> #t<T> {
            #fns
        }
    }
    .into()
}
