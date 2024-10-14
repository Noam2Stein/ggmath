use std::marker::PhantomData;

use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    spanned::Spanned,
    Attribute, Error, Expr, FnArg, GenericArgument, GenericParam, Generics, Ident, ItemFn,
    Lifetime, Pat, PathArguments, Receiver, ReturnType, Signature, Token, TraitItemFn, Type,
    TypeReference, Visibility,
};

pub fn vec_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let input = match ProcessedInput::try_from(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let impl_block = vec(&input);
    let scalar = scalar(&input);
    let len = len(&input);
    let storage = storage(&input);

    quote! {
        #impl_block
        #scalar
        #len
        #storage
    }
    .into()
}

fn vec(input: &ProcessedInput) -> TokenStream {
    let impl_abstract_fns = input.abstract_fns.iter().cloned().map(|abstract_fn| {
        abstract_fn.into_fn_impl(
            ApiFnGenerics::None,
            &Perspective::Vec,
            Visibility::Public(Default::default()),
            parse_quote!(ScalarCount<N>),
            {
                let ident = len_trait_ident(&input.ident);
                parse_quote!(#ident<N>)
            },
            |input| input.pass_inner(),
            |expr, output| output.wrap_inner(expr),
        )
    });

    let impl_free_fns = input.free_fns.iter().cloned();

    quote! {
        impl<const N: usize, T: Scalar, S: VecStorage> Vector<N, T, S> where ScalarCount<N>: VecLen<N> {
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
    let trait_ident = scalar_trait_ident(&input.ident);

    let fns =
        input.abstract_fns.iter().cloned().map(|abstract_fn| {
            abstract_fn.into_trait_fn(ApiFnGenerics::None, &Perspective::Scalar)
        });

    quote! {
        pub trait #trait_ident<const N: usize, S: VecStorage>: inner::ScalarAlignedVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
    }
}
fn len(input: &ProcessedInput) -> TokenStream {
    let trait_ident = len_trait_ident(&input.ident);

    let fns =
        input.abstract_fns.iter().cloned().map(|abstract_fn| {
            abstract_fn.into_trait_fn(ApiFnGenerics::ST, &Perspective::Len(None))
        });

    let impls = (2..5).map(|n| {
        let n = Literal::usize_unsuffixed(n);

        let fns = input.abstract_fns.iter().cloned().map(|abstract_fn| {
            abstract_fn.into_fn_impl(
                ApiFnGenerics::ST,
                &Perspective::Len(Some(n.clone())),
                Visibility::Inherited,
                parse_quote!(S),
                {
                    let ident = storage_trait_ident(&input.ident);
                    parse_quote!(#ident<#n>)
                },
                |input| input.pass(),
                |expr, _| expr,
            )
        });

        quote! {
            impl #trait_ident<#n> for ScalarCount<#n> {
                #(
                    #fns
                )*
            }
        }
    });

    quote! {
        pub(super) trait #trait_ident<const N: usize>: inner::VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
        #(
            #impls
        )*
    }
}
fn storage(input: &ProcessedInput) -> TokenStream {
    let trait_ident = storage_trait_ident(&input.ident);

    let fns = input.abstract_fns.iter().cloned().map(|abstract_fn| {
        abstract_fn.into_trait_fn(ApiFnGenerics::T, &Perspective::Storage(None))
    });

    let impls = (2..5)
        .map(|n| {
            let n = Literal::usize_unsuffixed(n);

            let fns = input
                .abstract_fns
                .iter()
                .cloned()
                .map(|abstract_fn| {
                    abstract_fn.into_fn_impl(
                        ApiFnGenerics::T,
                        &Perspective::Storage(Some(n.clone())),
                        Visibility::Inherited,
                        parse_quote!(T),
                        {
                            let ident = scalar_trait_ident(&input.ident);
                            parse_quote!(#ident<#n, Self>)
                        },
                        |input| input.pass(),
                        |expr, _| expr,
                    )
                })
                .collect::<Box<[ItemFn]>>();

            ["VecPacked", "VecAligned"].map(|s| {
                let s = Ident::new(s, Span::call_site());

                quote! {
                    impl #trait_ident<#n> for #s {
                        #(
                            #fns
                        )*
                    }
                }
            })
        })
        .flatten();

    quote! {
        pub(super) trait #trait_ident<const N: usize>: inner::VecStorageInnerVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
        #(
            #impls
        )*
    }
}

fn scalar_trait_ident(api_ident: &Ident) -> Ident {
    Ident::new(&format!("ScalarVec{api_ident}Api"), api_ident.span())
}
fn len_trait_ident(api_ident: &Ident) -> Ident {
    Ident::new(&format!("VecLen{api_ident}Api"), api_ident.span())
}
fn storage_trait_ident(api_ident: &Ident) -> Ident {
    Ident::new(&format!("VecStorage{api_ident}Api"), api_ident.span())
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
                    vis: Visibility::Public(Default::default()),
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
impl AbstractApiFn {
    fn into_trait_fn(mut self, generics: ApiFnGenerics, perspective: &Perspective) -> TraitItemFn {
        for param in generics.params().into_vec().into_iter().rev() {
            self.generics.params.insert(0, param);
        }

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
                    .map(|input| input.into_fn_arg(perspective))
                    .collect(),
                variadic: None,
                output: self.output.into_return_ty(perspective),
            },
            default: None,
            semi_token: Default::default(),
        }
    }
    fn into_fn_impl(
        mut self,
        generics: ApiFnGenerics,
        perspective: &Perspective,
        vis: Visibility,
        call_ty: Type,
        call_trait: Type,
        call_input_f: impl FnMut(&ApiFnArg) -> Expr,
        call_wrap: impl FnOnce(Expr, &ApiReturnType) -> Expr,
    ) -> ItemFn {
        self.attrs.insert(0, parse_quote!(#[inline(always)]));

        let block = {
            let ident = &self.ident;
            let args = self.inputs.iter().map(call_input_f);

            let call_generics = generics.next().args().into_vec().into_iter().chain(
                self.generics.params.iter().filter_map(|param| match param {
                    GenericParam::Type(param) => Some({
                        let ident = &param.ident;
                        parse_quote! { #ident }
                    }),
                    GenericParam::Const(param) => Some({
                        let ident = &param.ident;
                        parse_quote! { #ident }
                    }),
                    GenericParam::Lifetime(_) => None,
                }),
            );

            let expr = call_wrap(
                parse_quote!(
                    <#call_ty as #call_trait>::#ident::<#(#call_generics), *>(#(#args), *)
                ),
                &self.output,
            );

            parse_quote!({ #expr })
        };

        for param in generics.params().into_vec().into_iter().rev() {
            self.generics.params.insert(0, param);
        }

        ItemFn {
            attrs: self.attrs,
            vis,
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
                    .map(|input| input.into_fn_arg(perspective))
                    .collect(),
                variadic: None,
                output: self.output.into_return_ty(perspective),
            },
            block,
        }
    }
}

#[derive(Clone, Copy)]
enum ApiFnGenerics {
    None,
    ST,
    T,
}
impl ApiFnGenerics {
    fn next(self) -> Self {
        match self {
            Self::None => Self::ST,
            Self::ST => Self::T,
            Self::T => Self::None,
        }
    }

    fn params(self) -> Box<[GenericParam]> {
        match self {
            Self::None => Box::new([]),
            Self::ST => Box::new([parse_quote!(S: VecStorage), parse_quote!(T: Scalar)]),
            Self::T => Box::new([parse_quote!(T: Scalar)]),
        }
    }
    fn args(self) -> Box<[GenericArgument]> {
        match self {
            Self::None => Box::new([]),
            Self::ST => Box::new([parse_quote!(S), parse_quote!(T)]),
            Self::T => Box::new([parse_quote!(T)]),
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
    fn pass(&self) -> Expr {
        match self {
            ApiFnArg::Receiver(_) => parse_quote!(vec),
            ApiFnArg::Typed {
                attrs: _,
                ident,
                ty: _,
            } => parse_quote!(#ident),
        }
    }

    fn into_fn_arg(self, perspective: &Perspective) -> FnArg {
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
                    let s = perspective.s();
                    let t = perspective.t();

                    match reference {
                        Some((_, lifetime)) => {
                            parse_quote!(#(#attrs)* vec: &#lifetime #mutability inner::InnerVector<#n, #t, #s>)
                        }
                        None => parse_quote!(#(#attrs)* vec: inner::InnerVector<#n, #t, #s>),
                    }
                }
            },
            Self::Typed { attrs, ident, ty } => {
                let ty = ty.into_ty(perspective);
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
impl ApiReturnType {
    fn wrap_inner(&self, inner: Expr) -> Expr {
        match self {
            Self::Default => inner,
            Self::Type(ty) => ty.wrap_inner(inner),
        }
    }

    fn into_return_ty(self, perspective: &Perspective) -> ReturnType {
        match self {
            Self::Default => ReturnType::Default,
            Self::Type(ty) => {
                ReturnType::Type(Default::default(), Box::new(ty.into_ty(perspective)))
            }
        }
    }
}

#[derive(Clone)]
enum ApiType {
    SelfVec,
    Vec(VecGenerics),
    T,
    Array {
        elem: Box<ApiType>,
        len: VecGeneric<N>,
    },
    Reference {
        lifetime: Option<Lifetime>,
        mutability: Option<Token![mut]>,
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
                            PathArguments::AngleBracketed(args) => ApiType::Vec(
                                (&*args
                                    .args
                                    .iter()
                                    .cloned()
                                    .collect::<Box<[GenericArgument]>>())
                                    .try_into()?,
                            ),
                            args => return Err(Error::new_spanned(args, "expected <N, T, S>")),
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
                len: VecGeneric::from_tokens(value.len),
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
impl ApiType {
    fn pass_inner(&self, outer: Expr) -> Expr {
        match self {
            Self::SelfVec => parse_quote!(#outer.inner),
            Self::Vec(_) => parse_quote!(#outer.inner),
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
            Self::Vec(_) => parse_quote!(Vector { inner: #inner }),
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

    fn into_ty(self, perspective: &Perspective) -> Type {
        match self {
            ApiType::SelfVec => match perspective {
                Perspective::Vec => parse_quote! { Self },
                perspective => {
                    let n = perspective.n();
                    let s = perspective.s();
                    let t = perspective.t();

                    parse_quote! { inner::InnerVector<#n, #t, #s> }
                }
            },
            ApiType::Vec(generics) => {
                let (n, t, s) = generics.tokens(perspective);

                parse_quote!(inner::InnerVector<#n, #t, #s>)
            }
            ApiType::T => {
                let t = perspective.t();
                parse_quote!(#t)
            }
            Self::Array { elem, len } => {
                let elem = elem.into_ty(perspective);
                let len = len.tokens(perspective);

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
                elem: Box::new(elem.into_ty(perspective)),
            }),
            ApiType::Type(ty) => ty,
        }
    }
}

#[derive(Clone)]
struct VecGenerics {
    n: VecGeneric<N>,
    t: VecGeneric<T>,
    s: VecGeneric<S>,
}
impl TryFrom<&[GenericArgument]> for VecGenerics {
    type Error = Error;
    fn try_from(value: &[GenericArgument]) -> Result<Self, Self::Error> {
        match &*value {
            [n, t, s] => Ok(Self {
                n: VecGeneric::from_tokens(n),
                s: VecGeneric::from_tokens(s),
                t: VecGeneric::from_tokens(t),
            }),
            args => {
                return Err(Error::new(
                    args.first()
                        .map_or_else(|| Span::call_site(), |arg| arg.span()),
                    "expected <N, T, S>",
                ))
            }
        }
    }
}
impl VecGenerics {
    fn tokens(&self, perspective: &Perspective) -> (TokenStream, TokenStream, TokenStream) {
        (
            self.n.tokens(perspective),
            self.t.tokens(perspective),
            self.s.tokens(perspective),
        )
    }
}

#[derive(Clone)]
enum VecGeneric<K: VecGenericKey> {
    SelfVec(PhantomData<K>),
    Other(TokenStream),
}
impl<K: VecGenericKey> VecGeneric<K> {
    fn from_tokens(value: impl ToTokens) -> Self {
        let value = value
            .to_token_stream()
            .into_iter()
            .collect::<Box<[TokenTree]>>();

        if value.len() == 1 {
            if let TokenTree::Ident(ident) = value.iter().next().unwrap() {
                if ident.to_string() == K::SELF_VEC_VALUE {
                    return Self::SelfVec(Default::default());
                }
            }
        };

        Self::Other(value.into_vec().into_iter().collect())
    }
    fn tokens(&self, perspective: &Perspective) -> TokenStream {
        match self {
            Self::SelfVec(_) => K::self_vec_value(perspective),
            Self::Other(output) => output.clone(),
        }
    }
}

trait VecGenericKey: Clone {
    const SELF_VEC_VALUE: &'static str;
    fn self_vec_value(perspective: &Perspective) -> TokenStream;
}
#[derive(Clone)]
struct N;
impl VecGenericKey for N {
    const SELF_VEC_VALUE: &'static str = "N";
    fn self_vec_value(perspective: &Perspective) -> TokenStream {
        perspective.n()
    }
}
#[derive(Clone)]
struct T;
impl VecGenericKey for T {
    const SELF_VEC_VALUE: &'static str = "T";
    fn self_vec_value(perspective: &Perspective) -> TokenStream {
        perspective.t()
    }
}
#[derive(Clone)]
struct S;
impl VecGenericKey for S {
    const SELF_VEC_VALUE: &'static str = "S";
    fn self_vec_value(perspective: &Perspective) -> TokenStream {
        perspective.s()
    }
}

enum Perspective {
    Vec,
    Scalar,
    Len(Option<Literal>),
    Storage(Option<Literal>),
}
impl Perspective {
    fn n(&self) -> TokenStream {
        match self {
            Self::Vec => quote! { N },
            Self::Scalar => quote! { N },
            Self::Len(n) => n.as_ref().map_or_else(|| quote! { N }, |n| quote! { #n }),
            Self::Storage(n) => n.as_ref().map_or_else(|| quote! { N }, |n| quote! { #n }),
        }
    }
    fn s(&self) -> TokenStream {
        match self {
            Self::Vec => quote! { S },
            Self::Scalar => quote! { S },
            Self::Len(_) => quote! { S },
            Self::Storage(_) => quote! { Self },
        }
    }
    fn t(&self) -> TokenStream {
        match self {
            Self::Vec => quote! { T },
            Self::Scalar => quote! { Self },
            Self::Len(_) => quote! { T },
            Self::Storage(_) => quote! { T },
        }
    }
}
