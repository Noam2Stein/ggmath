use syn::{
    parse_quote, punctuated::Punctuated, Expr, GenericArgument, Lifetime, PathArguments, Token,
    Type, TypeArray, TypePath, TypeReference,
};

use crate::idents::*;

use super::{generics::*, perspective::*};

#[derive(Debug, Clone)]
pub enum ApiType {
    Vec(ApiTypeVec),
    T,
    Array(ApiTypeArray),
    Reference(ApiTypeReference),
    Other(Type),
}
impl From<Type> for ApiType {
    fn from(value: Type) -> Self {
        match value {
            Type::Array(value) => Self::Array(value.into()),
            Type::Path(value) => value.into(),
            Type::Reference(value) => Self::Reference(value.into()),
            value => Self::Other(value),
        }
    }
}
impl From<TypePath> for ApiType {
    fn from(value: TypePath) -> Self {
        if value.path.segments.len() != 1 {
            return Self::Other(Type::Path(value));
        };

        let last_segment = value.path.segments.iter().last().unwrap();

        macro_rules! match_ident {
            ($($vec_alias:tt => ($vec_alias_n:expr, $vec_alias_t:expr, $vec_alias_a:expr)), * $(,)?) => {
                match last_segment.ident.to_string().as_str() {
                    $(stringify!($vec_alias) => Self::Vec(ApiTypeVec::from_arguments(
                        last_segment.arguments.clone(),
                        $vec_alias_n,
                        $vec_alias_t,
                        $vec_alias_a,
                    )),)*
                    "T" => Self::T,
                    _ => Self::Other(Type::Path(value)),
                }
            };
        }
        match_ident!(
            Self => (Some(NArgument::SelfVec), Some(TArgument::SelfVec), Some(AArgument::SelfVec)),
            Vector => (None, None, None),
            Vector2 => (Some(NArgument::Known(KnownVecLen::U2)), None, None),
            Vector3 => (Some(NArgument::Known(KnownVecLen::U3)), None, None),
            Vector4 => (Some(NArgument::Known(KnownVecLen::U4)), None, None),
            VecN => (None, None, Some(AArgument::Known(KnownVecAlignment::VecAligned))),
            Vec2 => (Some(NArgument::Known(KnownVecLen::U2)), None, Some(AArgument::Known(KnownVecAlignment::VecAligned))),
            Vec3 => (Some(NArgument::Known(KnownVecLen::U3)), None, Some(AArgument::Known(KnownVecAlignment::VecAligned))),
            Vec4 => (Some(NArgument::Known(KnownVecLen::U4)), None, Some(AArgument::Known(KnownVecAlignment::VecAligned))),
            VecNP => (None, None, Some(AArgument::Known(KnownVecAlignment::VecPacked))),
            Vec2P => (Some(NArgument::Known(KnownVecLen::U2)), None, Some(AArgument::Known(KnownVecAlignment::VecPacked))),
            Vec3P => (Some(NArgument::Known(KnownVecLen::U3)), None, Some(AArgument::Known(KnownVecAlignment::VecPacked))),
            Vec4P => (Some(NArgument::Known(KnownVecLen::U4)), None, Some(AArgument::Known(KnownVecAlignment::VecPacked))),
        )
    }
}
impl FromPerspective for ApiType {
    type Output = Type;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match self {
            ApiType::T => {
                let t = perspective.t();
                parse_quote!(#t)
            }
            ApiType::Vec(ty) => ty.from_perspective(perspective),
            Self::Array(ty) => Type::Array(ty.from_perspective(perspective)),
            ApiType::Reference(ty) => Type::Reference(ty.from_perspective(perspective)),
            ApiType::Other(ty) => ty,
        }
    }
}
impl ApiType {
    pub fn pass_inner(&self, outer: Expr) -> Expr {
        match self {
            Self::Vec(_) => parse_quote!(#outer.inner),
            Self::T => outer,
            Self::Array(ApiTypeArray { elem, len: _ }) => {
                let f = elem.pass_inner(parse_quote!(item));
                parse_quote!(#outer.map(|item| #f))
            }
            Self::Reference(_) => parse_quote!(unsafe { std::mem::transmute(#outer) }),
            Self::Other(_) => outer,
        }
    }
    pub fn wrap_inner(&self, inner: Expr) -> Expr {
        match self {
            Self::Vec(_) => parse_quote!(Vector { inner: #inner }),
            Self::T => inner,
            Self::Array(ApiTypeArray { elem, len: _ }) => {
                let f = elem.wrap_inner(parse_quote!(item));
                parse_quote!(#inner.map(|item| #f))
            }
            Self::Reference(_) => parse_quote!(unsafe { std::mem::transmute(#inner) }),
            Self::Other(_) => inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeVec {
    n: NArgument,
    t: TArgument,
    a: AArgument,
}
impl ApiTypeVec {
    fn from_arguments(
        arguments: PathArguments,
        n: Option<NArgument>,
        t: Option<TArgument>,
        a: Option<AArgument>,
    ) -> Self {
        let mut arguments = match arguments {
            PathArguments::AngleBracketed(arguments) => arguments.args,
            PathArguments::None => Punctuated::new(),
            PathArguments::Parenthesized(_) => {
                panic!("unexpected parenthesized generics for vector")
            }
        }
        .into_iter();

        let n = n.unwrap_or_else(|| match arguments.next().unwrap() {
            GenericArgument::Const(argument) => argument.into(),
            GenericArgument::AssocConst(argument) => argument.value.into(),
            _ => panic!("expected const generic argument for {N}"),
        });

        let t = t.unwrap_or_else(|| match arguments.next().unwrap() {
            GenericArgument::Type(argument) => argument.into(),
            GenericArgument::AssocType(argument) => argument.ty.into(),
            _ => panic!("expected const generic argument for {T}"),
        });

        let a = a.unwrap_or_else(|| match arguments.next().unwrap() {
            GenericArgument::Type(argument) => argument.into(),
            GenericArgument::AssocType(argument) => argument.ty.into(),
            _ => panic!("expected const generic argument for {A}"),
        });

        Self { n, t, a }
    }
}
impl FromPerspective for ApiTypeVec {
    type Output = Type;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match perspective {
            Perspective::Vec => {
                if let NArgument::SelfVec = self.n {
                    if let TArgument::SelfVec = self.t {
                        if let AArgument::SelfVec = self.a {
                            return parse_quote!(Self);
                        }
                    }
                };

                let t = self.t.from_perspective(perspective);
                match self.n {
                    NArgument::Known(KnownVecLen::U2) => match self.a {
                        AArgument::Known(KnownVecAlignment::VecAligned) => parse_quote!(#Vec2<#t>),
                        AArgument::Known(KnownVecAlignment::VecPacked) => parse_quote!(#Vec2P<#t>),
                        a => {
                            let a = a.from_perspective(perspective);
                            parse_quote!(Vector2<#t, #a>)
                        }
                    },
                    NArgument::Known(KnownVecLen::U3) => match self.a {
                        AArgument::Known(KnownVecAlignment::VecAligned) => parse_quote!(#Vec3<#t>),
                        AArgument::Known(KnownVecAlignment::VecPacked) => parse_quote!(#Vec3P<#t>),
                        a => {
                            let a = a.from_perspective(perspective);
                            parse_quote!(Vector3<#t, #a>)
                        }
                    },
                    NArgument::Known(KnownVecLen::U4) => match self.a {
                        AArgument::Known(KnownVecAlignment::VecAligned) => parse_quote!(#Vec4<#t>),
                        AArgument::Known(KnownVecAlignment::VecPacked) => parse_quote!(#Vec4P<#t>),
                        a => {
                            let a = a.from_perspective(perspective);
                            parse_quote!(Vector4<#t, #a>)
                        }
                    },
                    n => {
                        let n = n.from_perspective(perspective);
                        match self.a {
                            AArgument::Known(KnownVecAlignment::VecAligned) => {
                                parse_quote!(#VecN<#n, #t>)
                            }
                            AArgument::Known(KnownVecAlignment::VecPacked) => {
                                parse_quote!(#VecNP<#n, #t>)
                            }
                            a => {
                                let a = a.from_perspective(perspective);
                                parse_quote!(Vector<#n, #t, #a>)
                            }
                        }
                    }
                }
            }
            perspective => {
                let n = self.n.from_perspective(perspective);
                let t = self.t.from_perspective(perspective);
                let a = self.a.from_perspective(perspective);

                parse_quote!(inner::#InnerVector<#n, #t, #a>)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeArray {
    elem: Box<ApiType>,
    len: NArgument,
}
impl From<TypeArray> for ApiTypeArray {
    fn from(value: TypeArray) -> Self {
        Self {
            elem: Box::new((*value.elem).into()),
            len: value.len.into(),
        }
    }
}
impl FromPerspective for ApiTypeArray {
    type Output = TypeArray;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        TypeArray {
            bracket_token: Default::default(),
            elem: Box::new(self.elem.from_perspective(perspective)),
            semi_token: Default::default(),
            len: self.len.from_perspective(perspective),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeReference {
    lifetime: Option<Lifetime>,
    mutability: Option<Token![mut]>,
    elem: Box<ApiType>,
}
impl From<TypeReference> for ApiTypeReference {
    fn from(value: TypeReference) -> Self {
        Self {
            lifetime: value.lifetime,
            mutability: value.mutability,
            elem: Box::new((*value.elem).into()),
        }
    }
}
impl FromPerspective for ApiTypeReference {
    type Output = TypeReference;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        TypeReference {
            and_token: Default::default(),
            lifetime: self.lifetime,
            mutability: self.mutability,
            elem: Box::new((*self.elem).from_perspective(perspective)),
        }
    }
}
