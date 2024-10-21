use super::{arg::*, perspective::*, return_ty::*, traits::*, *};

use std::iter::once;

use syn::{token::Paren, Attribute, Generics, ItemFn, Signature, TraitItemFn, Visibility};

#[derive(Clone)]
pub struct AbstractApiFn {
    fn_token: Token![fn],
    semi_token: Token![;],
    paren_token: Paren,
    attrs: Vec<Attribute>,
    constness: Option<Token![const]>,
    asyncness: Option<Token![async]>,
    unsafety: Option<Token![unsafe]>,
    ident: Ident,
    generics: Generics,
    inputs: Vec<ApiFnArg>,
    output: ApiReturnType,
}
impl TryFrom<TraitItemFn> for AbstractApiFn {
    type Error = Error;
    fn try_from(value: TraitItemFn) -> Result<Self, Self::Error> {
        if value.default.is_some() {
            return Err(Error::new_spanned(value.default, "unexpected default"));
        };

        Ok(Self {
            fn_token: value.sig.fn_token,
            semi_token: value
                .semi_token
                .unwrap_or_else(|| panic!("expected fn semi colon")),
            paren_token: value.sig.paren_token,
            attrs: value.attrs,
            constness: value.sig.constness,
            asyncness: value.sig.asyncness,
            unsafety: value.sig.unsafety,
            ident: value.sig.ident,
            generics: value.sig.generics,
            inputs: value
                .sig
                .inputs
                .into_iter()
                .map(|input| ApiFnArg::try_from(input))
                .collect::<Result<Vec<ApiFnArg>, Self::Error>>()?,
            output: value.sig.output.try_into()?,
        })
    }
}
impl FromPerspective for AbstractApiFn {
    type Output = TraitItemFn;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        let mut attrs = self.attrs;
        match perspective {
            Perspective::Vec => {}
            _ => attrs.insert(
                0,
                parse_quote_spanned!(self.fn_token.span() => #[allow(missing_docs)]),
            ),
        }

        let mut generics = self.generics;
        match perspective {
            Perspective::Len(_) => generics.params.insert(
                0,
                parse2(quote_spanned!(self.ident.span() => A: VecAlignment))
                    .unwrap_or_else(|err| panic!("failed to parse fn generic A: {err}")),
            ),
            _ => {}
        };
        match perspective {
            Perspective::Len(_) | Perspective::Alignment(_) => generics.params.insert(
                0,
                parse2(quote_spanned!(self.ident.span() => T: Scalar))
                    .unwrap_or_else(|err| panic!("failed to parse fn generic T: {err}")),
            ),
            _ => {}
        };

        TraitItemFn {
            attrs,
            sig: Signature {
                constness: self.constness,
                asyncness: self.asyncness,
                unsafety: self.unsafety,
                abi: None,
                fn_token: self.fn_token,
                ident: self.ident,
                generics,
                paren_token: self.paren_token,
                inputs: self
                    .inputs
                    .into_iter()
                    .map(|input| input.from_perspective(perspective))
                    .collect(),
                variadic: None,
                output: self.output.from_perspective(perspective),
            },
            semi_token: Some(self.semi_token),
            default: None,
        }
    }
}
impl AbstractApiFn {
    pub fn impl_from_perspective(self, api_ident: &Ident, perspective: Perspective) -> ItemFn {
        let block = {
            let call_ty = match perspective {
                Perspective::Vec => {
                    let len_trait = len_trait_ident(api_ident);
                    quote_spanned! { self.ident.span() => <ScalarCount<N> as #len_trait<N>> }
                }
                Perspective::Len(_) => {
                    let alignment_trait = alignment_trait_ident(api_ident);
                    let n = perspective.n(self.fn_token.span);
                    quote_spanned! { self.ident.span() => <A as #alignment_trait<#n>> }
                }
                Perspective::Alignment(_) => {
                    let scalar_trait = scalar_trait_ident(api_ident);
                    let n = perspective.n(self.fn_token.span);
                    quote_spanned! { self.ident.span() => <T as #scalar_trait<#n, Self>> }
                }
                Perspective::Scalar => {
                    unreachable!("impl of scalar perspective")
                }
            };

            let call_ident = &self.ident;

            let mut call_generics = self.generics.params.clone();
            match perspective {
                Perspective::Vec => call_generics.insert(
                    0,
                    parse2(quote_spanned! { self.ident.span() => A }).unwrap_or_else(|err| {
                        panic!("failed to parse fn call generic argument A: {err}")
                    }),
                ),
                _ => {}
            };
            match perspective {
                Perspective::Vec | Perspective::Len(_) => call_generics.insert(
                    0,
                    parse2(quote_spanned! { self.ident.span() => T }).unwrap_or_else(|err| {
                        panic!("failed to parse fn call generic argument T: {err}")
                    }),
                ),
                _ => {}
            };

            let call_args = self
                .inputs
                .iter()
                .map(|input| input.pass_from_perspective(perspective));

            let call_into_outer = self
                .output
                .pass_from_perspective(&Ident::new("output", self.ident.span()), perspective);

            parse2(quote_spanned! {self.ident.span() => {
                let output = #call_ty::#call_ident::<#call_generics>(#(#call_args), *);
                #call_into_outer
            }})
            .unwrap_or_else(|err| panic!("failed to parse fn block: {err}"))
        };

        let trait_fn = self.from_perspective(perspective);

        let attrs = once(parse_quote_spanned!(trait_fn.sig.fn_token.span() => #[inline(always)]))
            .chain(trait_fn.attrs)
            .collect();

        ItemFn {
            attrs,
            vis: match perspective {
                Perspective::Vec => {
                    Visibility::Public(parse_quote_spanned! { trait_fn.sig.fn_token.span() => pub })
                }
                _ => Visibility::Inherited,
            },
            sig: trait_fn.sig,
            block,
        }
    }
}
