use std::iter::once;

use quote::{quote, quote_spanned};
use syn::{
    parse2, parse_quote, spanned::Spanned, Attribute, Error, Expr, FnArg, Generics, Ident, ItemFn,
    Pat, Receiver, Result, ReturnType, Signature, Token, TraitItemFn, Visibility,
};

use crate::idents::*;

use super::{perspective::*, traits::*, ty::*};

#[derive(Clone)]
pub struct AbstractApiFn {
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
    fn try_from(value: TraitItemFn) -> Result<Self> {
        if value.default.is_some() {
            return Err(Error::new_spanned(value.default, "unexpected default"));
        };

        Ok(Self {
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
                .collect::<Result<Vec<ApiFnArg>>>()?,
            output: value.sig.output.into(),
        })
    }
}
impl FromPerspective for AbstractApiFn {
    type Output = TraitItemFn;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        let mut generics = self.generics;
        match perspective {
            Perspective::Len(_) => generics.params.insert(
                0,
                parse2(quote_spanned!(generics.span() => #A: #VecAlignment))
                    .unwrap_or_else(|err| panic!("failed to parse fn generic A: {err}")),
            ),
            _ => {}
        };
        match perspective {
            Perspective::Len(_) | Perspective::Alignment(_) => generics.params.insert(
                0,
                parse2(quote_spanned!(generics.span() => #T: #Scalar))
                    .unwrap_or_else(|err| panic!("failed to parse fn generic T: {err}")),
            ),
            _ => {}
        };

        TraitItemFn {
            attrs: self.attrs,
            sig: Signature {
                constness: self.constness,
                asyncness: self.asyncness,
                unsafety: self.unsafety,
                abi: None,
                fn_token: Default::default(),
                ident: self.ident,
                generics,
                paren_token: Default::default(),
                inputs: self
                    .inputs
                    .into_iter()
                    .map(|input| input.from_perspective(perspective))
                    .collect(),
                variadic: None,
                output: self.output.from_perspective(perspective),
            },
            default: None,
            semi_token: Default::default(),
        }
    }
}
impl AbstractApiFn {
    pub fn impl_from_perspective(
        self,
        api_ident: &Ident,
        perspective: &Perspective,
        call_input_f: impl FnMut(&ApiFnArg) -> Expr,
        call_wrap: impl FnOnce(Expr, &ApiReturnType) -> Expr,
    ) -> ItemFn {
        let block = {
            let call_ty = match perspective {
                Perspective::Vec => {
                    let len_trait = len_trait_ident(api_ident);
                    quote! { <#ScalarCount<#N> as #len_trait<#N>> }
                }
                Perspective::Len(_) => {
                    let alignment_trait = alignment_trait_ident(api_ident);
                    let n = perspective.n();
                    quote! { <#A as #alignment_trait<#n>> }
                }
                Perspective::Alignment(_) => {
                    let scalar_trait = scalar_trait_ident(api_ident);
                    let n = perspective.n();
                    quote! { <#T as #scalar_trait<#n, Self>> }
                }
                Perspective::Scalar => {
                    unreachable!("impl of scalar perspective")
                }
            };

            let call_ident = &self.ident;

            let mut call_generics = self.generics.params.clone();
            match &perspective {
                Perspective::Vec => call_generics.insert(
                    0,
                    parse2(quote!(#A)).unwrap_or_else(|err| {
                        panic!("failed to parse fn call generic argument A: {err}")
                    }),
                ),
                _ => {}
            };
            match &perspective {
                Perspective::Vec | Perspective::Len(_) => call_generics.insert(
                    0,
                    parse2(quote!(#T)).unwrap_or_else(|err| {
                        panic!("failed to parse fn call generic argument T: {err}")
                    }),
                ),
                _ => {}
            };

            let call_args = self.inputs.iter().map(call_input_f);

            let expr = call_wrap(
                parse2(quote!(
                    #call_ty::#call_ident::<#call_generics>(#(#call_args), *)
                ))
                .unwrap_or_else(|err| panic!("failed to parse fn call expr: {err}")),
                &self.output,
            );

            parse2(quote_spanned! {self.ident.span() => { #expr }})
                .unwrap_or_else(|err| panic!("failed to parse fn block: {err}"))
        };

        let trait_fn = self.from_perspective(perspective);

        let attrs = once(parse_quote!(#[inline(always)]))
            .chain(trait_fn.attrs)
            .collect();

        ItemFn {
            attrs,
            vis: match perspective {
                Perspective::Vec => Visibility::Public(Default::default()),
                _ => Visibility::Inherited,
            },
            sig: trait_fn.sig,
            block,
        }
    }
}

#[derive(Clone)]
pub enum ApiFnArg {
    Receiver(Receiver),
    Typed {
        attrs: Vec<Attribute>,
        ident: Ident,
        ty: ApiType,
    },
}
impl TryFrom<FnArg> for ApiFnArg {
    type Error = Error;
    fn try_from(value: FnArg) -> Result<Self> {
        match value {
            FnArg::Receiver(value) => Ok(Self::Receiver(value)),
            FnArg::Typed(value) => Ok(Self::Typed {
                attrs: value.attrs,
                ident: match *value.pat {
                    Pat::Ident(pat) => {
                        if let Some(by_ref) = pat.by_ref {
                            return Err(Error::new_spanned(
                                by_ref,
                                "ref isn't supported in vec api fns",
                            ));
                        } else {
                            pat.ident
                        }
                    }
                    pat => return Err(Error::new_spanned(pat, "expected an ident pat")),
                },
                ty: (*value.ty).into(),
            }),
        }
    }
}
impl FromPerspective for ApiFnArg {
    type Output = FnArg;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match self {
            Self::Receiver(receiver) => match perspective {
                Perspective::Vec => FnArg::Receiver(receiver),
                perspective => {
                    let Receiver {
                        attrs,
                        reference,
                        mutability,
                        self_token: _,
                        colon_token: _,
                        ty: _,
                    } = receiver;

                    let n = perspective.n();
                    let t = perspective.t();
                    let a = perspective.a();

                    match reference {
                        Some((_, lifetime)) => {
                            parse2(quote! { #(#attrs)* vec: &#lifetime #mutability inner::#InnerVector<#n, #t, #a> }).unwrap_or_else(|err| panic!("failed to parse ref receiver as vec: {err}"))
                        }
                        None => parse2(quote! { #(#attrs)* vec: inner::#InnerVector<#n, #t, #a> }).unwrap_or_else(|err| panic!("failed to parse receiver as vec: {err}")),
                    }
                }
            },
            Self::Typed { attrs, ident, ty } => {
                let ty = ty.from_perspective(perspective);
                parse2(quote! { #(#attrs)* #ident: #ty })
                    .unwrap_or_else(|err| panic!("failed to parse argument: {err}"))
            }
        }
    }
}
impl ApiFnArg {
    pub fn pass_inner(&self) -> Expr {
        match self {
            ApiFnArg::Receiver(Receiver {
                attrs: _,
                reference,
                mutability,
                self_token: _,
                colon_token: _,
                ty: _,
            }) => match reference {
                Some(_) => parse2(quote! { & #mutability self.inner })
                    .unwrap_or_else(|err| panic!("failed to parse ref receiver pass inner: {err}")),
                None => parse2(quote! { self.inner })
                    .unwrap_or_else(|err| panic!("failed to parse receiver pass inner: {err}")),
            },
            ApiFnArg::Typed {
                attrs: _,
                ident,
                ty,
            } => ty.pass_inner(parse_quote!(#ident)),
        }
    }
    pub fn pass(&self) -> Expr {
        match self {
            ApiFnArg::Receiver(_) => parse_quote!(vec),
            ApiFnArg::Typed {
                attrs: _,
                ident,
                ty: _,
            } => parse_quote!(#ident),
        }
    }
}

#[derive(Clone)]
pub enum ApiReturnType {
    Default,
    Type(ApiType),
}
impl From<ReturnType> for ApiReturnType {
    fn from(value: ReturnType) -> Self {
        match value {
            ReturnType::Default => Self::Default,
            ReturnType::Type(_, ty) => Self::Type((*ty).into()),
        }
    }
}
impl FromPerspective for ApiReturnType {
    type Output = ReturnType;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match self {
            Self::Default => ReturnType::Default,
            Self::Type(ty) => ReturnType::Type(
                Default::default(),
                Box::new(ty.from_perspective(perspective)),
            ),
        }
    }
}
impl ApiReturnType {
    pub fn wrap_inner(&self, inner: Expr) -> Expr {
        match self {
            Self::Default => inner,
            Self::Type(ty) => ty.wrap_inner(inner),
        }
    }
}
