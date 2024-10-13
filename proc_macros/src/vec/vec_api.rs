use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Async, Const, Mut, Pub, Unsafe},
    Attribute, Error, Expr, FnArg, GenericArgument, Generics, Ident, ItemFn, Lifetime, Pat,
    PatType, PathArguments, Receiver, ReturnType, Signature, Token, TraitItemFn, Type,
    TypeReference, Visibility,
};

pub fn vec_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let input = match ProcessedInput::try_from(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let impl_block = impl_block(&input);
    let scalar = scalar(&input);

    quote! {
        #impl_block
        #scalar
    }
    .into()
}

fn impl_block(input: &ProcessedInput) -> TokenStream {
    let input = input.clone();

    let impl_abstract_fns = input
        .abstract_fns
        .into_iter()
        .map(|abstract_fn| ItemFn::from(abstract_fn));

    let impl_free_fns = input.free_fns;

    quote! {
        impl<const N: usize, S: VecStorage, T: Scalar> Vector<N, S, T> where ScalarCount<N>: VecLen<N> {
            #(
                #impl_abstract_fns
            )*
            #(
                #impl_free_fns
            )*
        }
    }
}
fn scalar(input: &ProcessedInput) -> TokenStream {
    let api_ident = &input.ident;
    let trait_ident = Ident::new(&format!("ScalarVec{api_ident}Impl"), api_ident.span());

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.into_scalar_fn());

    quote! {
        pub trait #trait_ident<const N: usize, S: VecStorage>: ScalarInnerVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
    }
}

struct Input {
    ident: Ident,
    fns: Vec<TraitItemFn>,
}
impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let mut fns = Vec::new();

        while !input.is_empty() {
            fns.push(input.parse()?);
        }

        Ok(Self { ident, fns })
    }
}

#[derive(Clone)]
struct ProcessedInput {
    ident: Ident,
    abstract_fns: Vec<AbstractApiFn>,
    free_fns: Vec<ItemFn>,
}
impl TryFrom<Input> for ProcessedInput {
    type Error = Error;
    fn try_from(Input { ident, fns }: Input) -> Result<Self, Self::Error> {
        let mut abstract_fns = Vec::new();
        let mut free_fns = Vec::new();

        for fn_ in fns {
            if let Some(block) = fn_.default {
                free_fns.push(ItemFn {
                    attrs: fn_.attrs,
                    vis: Visibility::Public(Pub::default()),
                    sig: fn_.sig,
                    block: Box::new(block),
                });
            } else {
                abstract_fns.push(fn_.try_into()?);
            }
        }

        Ok(Self {
            ident,
            abstract_fns,
            free_fns,
        })
    }
}

#[derive(Clone)]
struct AbstractApiFn {
    attrs: Vec<Attribute>,
    constness: Option<Const>,
    asyncness: Option<Async>,
    unsafety: Option<Unsafe>,
    ident: Ident,
    generics: Generics,
    inputs: Vec<ApiFnArg>,
    output: ApiReturnType,
}
impl TryFrom<TraitItemFn> for AbstractApiFn {
    type Error = Error;
    fn try_from(value: TraitItemFn) -> Result<Self, Self::Error> {
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
                .collect::<Result<Vec<ApiFnArg>, Error>>()?,
            output: value.sig.output.try_into()?,
        })
    }
}
impl From<AbstractApiFn> for ItemFn {
    fn from(value: AbstractApiFn) -> Self {
        let mut attrs = value.attrs;
        attrs.insert(0, parse_quote!( #[inline(always)] ));

        let call_ident = &value.ident;

        let mut call_generics = value.generics.clone();
        call_generics.params.insert(0, parse_quote!(S));
        call_generics.params.insert(1, parse_quote!(T));

        let call_inputs = value.inputs.iter().map(|input| input.pass_inner());

        let call = value.output.wrap_inner(
            parse2(quote! {
                <ScalarCount<N> as TheNTrait>::#call_ident::#call_generics(#(#call_inputs), *)
            })
            .unwrap_or_else(|err| panic!("failed to parse N fn call: {err}")),
        );

        Self {
            attrs,
            vis: Visibility::Public(Pub::default()),
            sig: Signature {
                constness: value.constness,
                asyncness: value.asyncness,
                unsafety: value.unsafety,
                abi: None,
                fn_token: Default::default(),
                ident: value.ident,
                generics: value.generics,
                paren_token: Default::default(),
                inputs: value
                    .inputs
                    .into_iter()
                    .map(|input| FnArg::from(input))
                    .collect(),
                variadic: None,
                output: value.output.into(),
            },
            block: parse2(quote! { { #call } })
                .unwrap_or_else(|err| panic!("failed to parse abstract fn block: {err}")),
        }
    }
}
impl AbstractApiFn {
    fn into_scalar_fn(self) -> TraitItemFn {
        TraitItemFn {
            attrs: self.attrs,
            sig: Signature {
                constness: self.constness,
                asyncness: self.asyncness,
                unsafety: self.unsafety,
                abi: None,
                fn_token: Default::default(),
                ident: self.ident,
                generics: self.generics,
                paren_token: Default::default(),
                inputs: self
                    .inputs
                    .into_iter()
                    .map(|input| input.into_scalar_fn_arg())
                    .collect(),
                variadic: None,
                output: self.output.into_scalar_fn_output(),
            },
            default: None,
            semi_token: Default::default(),
        }
    }
}

#[derive(Clone)]
enum ApiFnArg {
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
                ty: (*value.ty).try_into()?,
            }),
        }
    }
}
impl From<ApiFnArg> for FnArg {
    fn from(value: ApiFnArg) -> Self {
        match value {
            ApiFnArg::Receiver(value) => FnArg::Receiver(value),
            ApiFnArg::Typed { attrs, ident, ty } => FnArg::Typed(PatType {
                attrs,
                pat: Box::new(parse_quote!(#ident)),
                colon_token: Default::default(),
                ty: Box::new(ty.into()),
            }),
        }
    }
}
impl ApiFnArg {
    fn pass_inner(&self) -> Expr {
        match self {
            ApiFnArg::Receiver(Receiver {
                attrs: _,
                reference,
                mutability,
                self_token: _,
                colon_token: _,
                ty: _,
            }) => match reference {
                Some(_) => parse_quote!(& #mutability self.inner),
                None => parse_quote!(self.inner),
            },
            ApiFnArg::Typed {
                attrs: _,
                ident,
                ty,
            } => ty.pass_inner(parse_quote!(#ident)),
        }
    }
}
impl ApiFnArg {
    fn into_scalar_fn_arg(self) -> FnArg {
        match self {
            Self::Receiver(Receiver {
                attrs,
                reference,
                mutability,
                self_token: _,
                colon_token: _,
                ty: _,
            }) => match reference {
                Some((_, lifetime)) => {
                    parse_quote!(#(#attrs)* vec: &#lifetime #mutability InnerVector<N, S, Self>)
                }
                None => parse_quote!(#(#attrs)* vec: InnerVector<N, S, Self>),
            },
            Self::Typed { attrs, ident, ty } => {
                let ty = ty.into_scalar_fn_ty();
                parse_quote!(#(#attrs)* #ident: #ty)
            }
        }
    }
}

#[derive(Clone)]
enum ApiReturnType {
    Default,
    Type(ApiType),
}
impl TryFrom<ReturnType> for ApiReturnType {
    type Error = Error;
    fn try_from(value: ReturnType) -> Result<Self, Self::Error> {
        Ok(match value {
            ReturnType::Default => Self::Default,
            ReturnType::Type(_, ty) => Self::Type((*ty).try_into()?),
        })
    }
}
impl From<ApiReturnType> for ReturnType {
    fn from(value: ApiReturnType) -> Self {
        match value {
            ApiReturnType::Default => Self::Default,
            ApiReturnType::Type(ty) => Self::Type(Default::default(), Box::new(ty.into())),
        }
    }
}
impl ApiReturnType {
    fn wrap_inner(&self, inner: Expr) -> Expr {
        match self {
            Self::Default => inner,
            Self::Type(ty) => ty.wrap_inner(inner),
        }
    }

    fn into_scalar_fn_output(self) -> ReturnType {
        match self {
            Self::Default => ReturnType::Default,
            Self::Type(ty) => {
                ReturnType::Type(Default::default(), Box::new(ty.into_scalar_fn_ty()))
            }
        }
    }
}

#[derive(Clone)]
enum ApiType {
    SelfVec,
    Vec {
        n: VecGenericArgument,
        s: VecGenericArgument,
        t: VecGenericArgument,
    },
    T,
    Array {
        elem: Box<ApiType>,
        len: VecGenericArgument,
    },
    Reference {
        lifetime: Option<Lifetime>,
        mutability: Option<Mut>,
        elem: Box<Self>,
    },
    Type(Type),
}
impl TryFrom<Type> for ApiType {
    type Error = Error;
    fn try_from(value: Type) -> Result<Self, Self::Error> {
        Ok(match value {
            Type::Path(value) => {
                if value.path.segments.len() == 1 {
                    let segment = value.path.segments.iter().last().unwrap();
                    match segment.ident.to_string().as_str() {
                        "Self" => match &segment.arguments {
                            PathArguments::None => ApiType::SelfVec,
                            args => {
                                return Err(Error::new_spanned(args, "didn't expected generics"))
                            }
                        },
                        "Vector" => match &segment.arguments {
                            PathArguments::AngleBracketed(args) => {
                                match &*args
                                    .args
                                    .iter()
                                    .cloned()
                                    .collect::<Box<[GenericArgument]>>()
                                {
                                    [n, s, t] => ApiType::Vec {
                                        n: VecGenericArgument::from_general(n, "N"),
                                        s: VecGenericArgument::from_general(s, "S"),
                                        t: VecGenericArgument::from_general(t, "T"),
                                    },
                                    args => {
                                        return Err(Error::new(
                                            args.first().map_or_else(
                                                || Span::call_site(),
                                                |arg| arg.span(),
                                            ),
                                            "expected <N, S, T>",
                                        ))
                                    }
                                }
                            }
                            args => return Err(Error::new_spanned(args, "expected <N, S, T>")),
                        },
                        "T" => match &segment.arguments {
                            PathArguments::None => ApiType::T,
                            args => {
                                return Err(Error::new_spanned(args, "didn't expected generics"))
                            }
                        },
                        _ => ApiType::Type(Type::Path(value)),
                    }
                } else {
                    Self::Type(Type::Path(value))
                }
            }
            Type::Array(value) => Self::Array {
                elem: Box::new((*value.elem).try_into()?),
                len: VecGenericArgument::from_general(value.len, "N"),
            },
            Type::Reference(value) => Self::Reference {
                lifetime: value.lifetime,
                mutability: value.mutability,
                elem: Box::new(Self::try_from(*value.elem)?),
            },
            value => Self::Type(value),
        })
    }
}
impl From<ApiType> for Type {
    fn from(value: ApiType) -> Self {
        match value {
            ApiType::SelfVec => parse_quote!(Self),
            ApiType::Vec { n, s, t } => {
                let n = match n {
                    VecGenericArgument::Default => quote! { N },
                    VecGenericArgument::Other(tokens) => tokens,
                };
                let s = match s {
                    VecGenericArgument::Default => quote! { S },
                    VecGenericArgument::Other(tokens) => tokens,
                };
                let t = match t {
                    VecGenericArgument::Default => quote! { T },
                    VecGenericArgument::Other(tokens) => tokens,
                };

                parse_quote!(Vector<#n, #s, #t>)
            }
            ApiType::T => parse_quote!(T),
            ApiType::Array { elem, len } => {
                let elem = Type::from(*elem);
                let len = match len {
                    VecGenericArgument::Default => quote! { N },
                    VecGenericArgument::Other(tokens) => tokens,
                };

                parse_quote!([#elem; #len])
            }
            ApiType::Reference {
                lifetime,
                mutability,
                elem,
            } => Self::Reference(TypeReference {
                and_token: Default::default(),
                lifetime,
                mutability,
                elem: Box::new((*elem).into()),
            }),
            ApiType::Type(ty) => ty,
        }
    }
}
impl ApiType {
    fn pass_inner(&self, outer: Expr) -> Expr {
        match self {
            Self::SelfVec => parse_quote!(#outer.inner),
            Self::Vec { n: _, s: _, t: _ } => parse_quote!(#outer.inner),
            Self::T => outer,
            Self::Array { elem, len: _ } => {
                let f = elem.pass_inner(parse_quote!(item));
                parse_quote!(#outer.map(|item| #f))
            }
            Self::Reference {
                lifetime: _,
                mutability: _,
                elem: _,
            } => parse_quote!(unsafe { std::mem::transmute(#outer) }),
            Self::Type(_) => outer,
        }
    }
    fn wrap_inner(&self, inner: Expr) -> Expr {
        match self {
            Self::SelfVec => parse_quote!(Self { inner: #inner }),
            Self::Vec { n: _, s: _, t: _ } => parse_quote!(Vector { inner: #inner }),
            Self::T => inner,
            Self::Array { elem, len: _ } => {
                let f = elem.wrap_inner(parse_quote!(item));
                parse_quote!(#inner.map(|item| #f))
            }
            Self::Reference {
                lifetime: _,
                mutability: _,
                elem: _,
            } => parse_quote!(unsafe { std::mem::transmute(#inner) }),
            Self::Type(_) => inner,
        }
    }

    fn into_scalar_fn_ty(self) -> Type {
        match self {
            ApiType::SelfVec => parse_quote!(InnerVector<N, S, Self>),
            ApiType::Vec { n, s, t } => {
                let n = match n {
                    VecGenericArgument::Default => quote! { N },
                    VecGenericArgument::Other(tokens) => tokens,
                };
                let s = match s {
                    VecGenericArgument::Default => quote! { S },
                    VecGenericArgument::Other(tokens) => tokens,
                };
                let t = match t {
                    VecGenericArgument::Default => quote! { Self },
                    VecGenericArgument::Other(tokens) => tokens,
                };

                parse_quote!(InnerVector<#n, #s, #t>)
            }
            ApiType::T => parse_quote!(Self),
            Self::Array { elem, len } => {
                let elem = elem.into_scalar_fn_ty();
                let len = match len {
                    VecGenericArgument::Default => quote! { N },
                    VecGenericArgument::Other(tokens) => tokens,
                };

                parse_quote!([#elem; #len])
            }
            ApiType::Reference {
                lifetime,
                mutability,
                elem,
            } => Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime,
                mutability,
                elem: Box::new(elem.into_scalar_fn_ty()),
            }),
            ApiType::Type(ty) => ty,
        }
    }
}

#[derive(Clone)]
enum VecGenericArgument {
    Default,
    Other(TokenStream),
}
impl Default for VecGenericArgument {
    fn default() -> Self {
        Self::Default
    }
}
impl VecGenericArgument {
    fn from_general(value: impl ToTokens, default: &str) -> Self {
        let value = value
            .to_token_stream()
            .into_iter()
            .collect::<Box<[TokenTree]>>();

        if value.len() == 1 {
            if let TokenTree::Ident(ident) = value.iter().next().unwrap() {
                if ident.to_string() == default {
                    return Self::Default;
                }
            }
        };

        Self::Other(value.into_vec().into_iter().collect())
    }
}
