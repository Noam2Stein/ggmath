use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token, Type};

pub fn vec_cset_wrappers(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
            let ident = Ident::new(&format!("set_{}", components[x]), components[x].span());
            quote! {
                #[inline(always)]
                pub fn #ident(&mut self, value: T) {
                    unsafe {
                        self.cset::<#x>(value)
                    }
                }
            }
        });
    }
    for x in 0..components.len() {
        for y in (0..components.len()).filter(|y| *y != x) {
            fns.extend({
                let ident = Ident::new(
                    &format!("set_{}{}", components[x], components[y]),
                    components[x].span(),
                );
                quote! {
                    #[inline(always)]
                    pub fn #ident(&mut self, value: Vec2<T>) {
                        unsafe {
                            self.cset2::<#x, #y>(value)
                        }
                    }
                }
            });
        }
    }
    for x in 0..components.len() {
        for y in (0..components.len()).filter(|y| *y != x) {
            for z in (0..components.len()).filter(|z| *z != x && *z != y) {
                fns.extend({
                    let ident = Ident::new(
                        &format!("set_{}{}{}", components[x], components[y], components[z]),
                        components[x].span(),
                    );
                    quote! {
                        #[inline(always)]
                        pub fn #ident(&mut self, value: Vec3<T>) {
                            unsafe {
                                self.cset3::<#x, #y, #z>(value)
                            }
                        }
                    }
                });
            }
        }
    }
    for x in 0..components.len() {
        for y in (0..components.len()).filter(|y| *y != x) {
            for z in (0..components.len()).filter(|z| *z != x && *z != y) {
                for w in (0..components.len()).filter(|w| *w != x && *w != y && *w != z) {
                    fns.extend({
                        let ident = Ident::new(
                            &format!(
                                "set_{}{}{}{}",
                                components[x], components[y], components[z], components[w]
                            ),
                            components[x].span(),
                        );
                        quote! {
                            #[inline(always)]
                            pub fn #ident(&mut self, value: Vec4<T>) {
                                unsafe {
                                    self.cset4::<#x, #y, #z, #w>(value)
                                }
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
