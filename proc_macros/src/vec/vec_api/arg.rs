use super::{perspective::*, ty::*, *};

use syn::{Attribute, Expr, FnArg, Pat, PatIdent, PatType, Receiver, Type, TypeReference};

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
    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        Ok(match value {
            FnArg::Receiver(value) => Self::Receiver(value),
            FnArg::Typed(value) => Self::Typed {
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
                ty: (*value.ty).try_into()?,
            },
        })
    }
}
impl FromPerspective for ApiFnArg {
    type Output = FnArg;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        match self {
            Self::Receiver(receiver) => match perspective {
                Perspective::Vec => FnArg::Receiver(receiver),
                perspective => {
                    let Receiver {
                        attrs,
                        reference,
                        mutability,
                        self_token,
                        colon_token: _,
                        ty: _,
                    } = receiver;

                    let n = perspective.n(self_token.span);
                    let t = perspective.t(self_token.span);
                    let a = perspective.a(self_token.span);

                    match reference {
                        Some((and_token, lifetime)) => FnArg::Typed(PatType {
                            attrs: Vec::new(),
                            pat: Box::new(Pat::Ident(PatIdent {
                                attrs,
                                by_ref: None,
                                mutability: None,
                                ident: Ident::new("vec", self_token.span()),
                                subpat: None,
                            })),
                            colon_token: parse_quote_spanned! { self_token.span() => : },
                            ty: Box::new(Type::Reference(TypeReference {
                                and_token,
                                lifetime,
                                mutability,
                                elem: Box::new(
                                    parse_quote_spanned! { self_token.span() => inner::InnerVector<#n, #t, #a> },
                                ),
                            })),
                        }),
                        None => parse2(quote_spanned! { self_token.span() => #(#attrs)* vec: inner::InnerVector<#n, #t, #a> })
                            .unwrap_or_else(|err| panic!("failed to parse receiver as vec: {err}")),
                    }
                }
            },
            Self::Typed { attrs, ident, ty } => {
                let ty = ty.from_perspective(perspective);
                FnArg::Typed(PatType {
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs,
                        by_ref: None,
                        mutability: None,
                        ident,
                        subpat: None,
                    })),
                    attrs: Vec::new(),
                    colon_token: parse_quote_spanned!( ty.span() => : ),
                    ty: Box::new(ty),
                })
            }
        }
    }
}
impl ApiFnArg {
    pub fn pass_from_perspective(&self, perspective: Perspective) -> Expr {
        if let Perspective::Vec = perspective {
            match self {
                ApiFnArg::Receiver(Receiver {
                    attrs: _,
                    reference,
                    mutability,
                    self_token,
                    colon_token: _,
                    ty: _,
                }) => match reference {
                    Some(_) => {
                        parse2(quote_spanned! { self_token.span() => & #mutability self.inner })
                            .unwrap_or_else(|err| {
                                panic!("failed to parse ref receiver pass inner: {err}")
                            })
                    }
                    None => parse2(quote_spanned! { self_token.span() => self.inner })
                        .unwrap_or_else(|err| panic!("failed to parse receiver pass inner: {err}")),
                },
                ApiFnArg::Typed {
                    attrs: _,
                    ident,
                    ty,
                } => ty.outer_value_into_inner(ident),
            }
        } else {
            match self {
                ApiFnArg::Receiver(receiver) => parse_quote_spanned! { receiver.span() => vec },
                ApiFnArg::Typed {
                    attrs: _,
                    ident,
                    ty: _,
                } => parse_quote_spanned! { ident.span() => #ident },
            }
        }
    }
}
