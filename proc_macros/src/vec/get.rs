use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token, Type};

pub fn impl_vec_cget_shortnames(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        t: Type,
        _0: Token![:],
        #[call(Punctuated::parse_terminated)]
        components: Punctuated<Ident, Token![,]>,
    }

    let Input { t, _0, components } = parse_macro_input!(tokens as Input);

    let mut fns = TokenStream::new();

    for x in 0..components.len() {
        fns.extend({
            let ident = components[x].clone();
            quote! {
                #[inline(always)]
                pub fn #ident(self) -> T {
                    self.cget::<#x>()
                }
            }
        });
    }
    for x in 0..components.len() {
        for y in 0..components.len() {
            fns.extend({
                let ident = Ident::new(
                    &format!("{}{}", components[x], components[y]),
                    components[x].span(),
                );
                quote! {
                    #[inline(always)]
                    pub fn #ident(self) -> Vec2<T> {
                        self.cget2::<#x, #y>()
                    }
                }
            });
        }
    }
    for x in 0..components.len() {
        for y in 0..components.len() {
            for z in 0..components.len() {
                fns.extend({
                    let ident = Ident::new(
                        &format!("{}{}{}", components[x], components[y], components[z]),
                        components[x].span(),
                    );
                    quote! {
                        #[inline(always)]
                        pub fn #ident(self) -> Vec3<T> {
                            self.cget3::<#x, #y, #z>()
                        }
                    }
                });
            }
        }
    }
    for x in 0..components.len() {
        for y in 0..components.len() {
            for z in 0..components.len() {
                for w in 0..components.len() {
                    fns.extend({
                        let ident = Ident::new(
                            &format!(
                                "{}{}{}{}",
                                components[x], components[y], components[z], components[w]
                            ),
                            components[x].span(),
                        );
                        quote! {
                            #[inline(always)]
                            pub fn #ident(self) -> Vec4<T> {
                                self.cget4::<#x, #y, #z, #w>()
                            }
                        }
                    });
                }
            }
        }
    }

    quote! {
        impl<T: Element> #t<T> {
            #fns
        }
    }
    .into()
}
