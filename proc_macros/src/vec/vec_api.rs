use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input, parse_quote,
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
    let impl_abstract_fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.into_impl(&input.ident));

    let impl_free_fns = input.free_fns.iter().cloned();

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
    let trait_ident = scalar_trait_ident(&input.ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.into_scalar_trait_fn());

    quote! {
        pub trait #trait_ident<const N: usize, S: VecStorage>: ScalarInnerVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
    }
}
fn len(input: &ProcessedInput) -> TokenStream {
    let trait_ident = len_trait_ident(&input.ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.into_len_trait_fn());

    let impls = (2..5).map(|n| {
        let fns = input
            .abstract_fns
            .iter()
            .cloned()
            .map(|abstract_fn| abstract_fn.into_len_trait_impl(&input.ident, n));

        quote! {
            impl #trait_ident<#n> for ScalarCount<#n> {
                #(
                    #fns
                )*
            }
        }
    });

    quote! {
        pub trait #trait_ident<const N: usize>: VecLenInnerVec where ScalarCount<N>: VecLen<N> {
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

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.into_storage_trait_fn());

    let impls = (2..5).map(|n| ["VecPacked", "VecAligned"].map(|s| {
        let s = Ident::new(s, Span::call_site());

        let fns = input
            .abstract_fns
            .iter()
            .cloned()
            .map(|abstract_fn| abstract_fn.into_storage_trait_impl(&input.ident, n, &s));

        quote! {
            impl #trait_ident<#n> for #s {
                #(
                    #fns
                )*
            }
        }
    })).flatten();

    quote! {
        pub trait #trait_ident<const N: usize>: VecLenInnerVec where ScalarCount<N>: VecLen<N> {
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
impl AbstractApiFn {
    fn into_scalar_trait_fn(self) -> TraitItemFn {
        self.into_trait_fn(|input| input.into_scalar_fn_arg(), |output| output.into_scalar_fn_output())
    }
    fn into_len_trait_fn(mut self) -> TraitItemFn {
        self.generics.params.insert(0, parse_quote!(S: VecStorageInnerVecs));
        self.generics.params.insert(1, parse_quote!(T: ScalarInnerVecs));

        self.into_trait_fn(|input| input.into_len_fn_arg(), |output| output.into_len_fn_output())
    }
    fn into_storage_trait_fn(mut self) -> TraitItemFn {
        self.generics.params.insert(1, parse_quote!(T: ScalarInnerVecs));

        self.into_trait_fn(|input| input.into_storage_fn_arg(), |output| output.into_storage_fn_output())
    }

    fn into_impl(self, api_ident: &Ident) -> ItemFn {
        let mut attrs = self.attrs;
        attrs.insert(0, parse_quote!( #[inline(always)] ));

        let call_trait_ident = len_trait_ident(&api_ident);
        let call_ident = &self.ident;

        let mut call_generics = self.generics.clone();
        call_generics.params.insert(0, parse_quote!(S));
        call_generics.params.insert(1, parse_quote!(T));

        let call_inputs = self.inputs.iter().map(|input| input.pass_inner());

        let call = self.output.wrap_inner(
            parse2(quote! {
                <ScalarCount<N> as #call_trait_ident>::#call_ident::#call_generics(#(#call_inputs), *)
            })
            .unwrap_or_else(|err| panic!("failed to parse N fn call: {err}")),
        );

        ItemFn {
            attrs,
            vis: Visibility::Public(Pub::default()),
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
                    .map(|input| FnArg::from(input))
                    .collect(),
                variadic: None,
                output: self.output.into(),
            },
            block: parse2(quote! { { #call } })
                .unwrap_or_else(|err| panic!("failed to parse abstract fn block: {err}")),
        }
    }
    fn into_len_trait_impl(self, api_ident: &Ident, n: usize) -> ItemFn {
        let mut generics = self.generics;
        generics
            .params
            .insert(0, parse_quote!(S: VecStorageInnerVecs));
        generics.params.insert(1, parse_quote!(T: ScalarInnerVecs));

        ItemFn {
            attrs: self.attrs,
            vis: Visibility::Inherited,
            sig: Signature {
                constness: self.constness,
                asyncness: self.asyncness,
                unsafety: self.unsafety,
                abi: None,
                fn_token: Default::default(),
                ident: self.ident,
                generics: generics,
                paren_token: Default::default(),
                inputs: self
                    .inputs
                    .into_iter()
                    .map(|input| input.into_len_fn_arg())
                    .collect(),
                variadic: None,
                output: self.output.into_len_fn_output(),
            },
            block: ,
        }
    }
    fn into_storage_trait_impl(self, api_ident: &Ident, n: usize, s: &Ident) -> ItemFn {
        
    }

    fn into_trait_fn(self, input_f: impl FnMut(ApiFnArg) -> FnArg, output_f: impl FnOnce(ApiReturnType) -> ReturnType) -> TraitItemFn {
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
                    .map(input_f)
                    .collect(),
                variadic: None,
                output: (output_f)(self.output),
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
    fn into_len_fn_arg(self) -> FnArg {
        self.into()
    }
    fn into_storage_fn_arg(self) -> FnArg {

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
    fn into_len_fn_output(self) -> ReturnType {
        self.into()
    }
    fn into_storage_fn_output(self) -> ReturnType {
        
    }

    fn into_ty_with_len(self) -> ReturnType {
        match self {
            ApiReturnType::Default => ReturnType::Default,
            ApiReturnType::Type(ty) => ty.into_ty_with_len()
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
