use super::{generics::*, perspective::*, *};

use syn::{
    punctuated::Punctuated, token::Bracket, Expr, GenericArgument, Lifetime, PathArguments,
    PathSegment, Type, TypeArray, TypePath, TypeReference,
};

#[derive(Debug, Clone)]
pub enum ApiType {
    Vec(ApiTypeVec),
    T(Span),
    Array(ApiTypeArray),
    Reference(ApiTypeReference),
    Option(ApiTypeOption),
    Other(Type),
}
impl TryFrom<Type> for ApiType {
    type Error = Error;
    fn try_from(value: Type) -> std::result::Result<Self, Self::Error> {
        match value {
            Type::Array(value) => Ok(Self::Array(value.try_into()?)),
            Type::Path(value) => value.try_into(),
            Type::Reference(value) => Ok(Self::Reference(value.try_into()?)),
            _ => Err(Error::new(value.span(), "unsupported type")),
        }
    }
}
impl TryFrom<TypePath> for ApiType {
    type Error = Error;
    fn try_from(value: TypePath) -> std::result::Result<Self, Self::Error> {
        if value.path.segments.len() != 1 {
            return Err(Error::new(
                value.span(),
                "unsupported type, expected a one segment path",
            ));
        };

        let segment = value.path.segments.iter().last().unwrap();

        macro_rules! match_ident {
            ($($vec_alias:tt => ($vec_alias_n:expr, $vec_alias_t:expr, $vec_alias_a:expr)), * $(,)?) => {
                match segment.ident.to_string().as_str() {
                    $(stringify!($vec_alias) => Ok(Self::Vec(ApiTypeVec::try_from_arguments(
                        segment.clone(),
                        $vec_alias_n,
                        $vec_alias_t,
                        $vec_alias_a,
                    )?)),)*
                    "T" => Ok(Self::T(segment.ident.span())),
                    "Option" => Ok(Self::Option(segment.clone().try_into()?)),
                    "usize" => Ok(Self::Other(Type::Path(value))),
                    _ => Err(Error::new(segment.ident.span(), "unsupported type")),
                }
            };
        }
        match_ident!(
            Self => (Some(NArgument::SelfVec(segment.ident.span())), Some(TArgument::SelfVec(segment.ident.span())), Some(AArgument::SelfVec(segment.ident.span()))),
            Vector => (None, None, None),
            Vector2 => (Some(NArgument::Known(NArgumentKnown::U2(segment.ident.span()))), None, None),
            Vector3 => (Some(NArgument::Known(NArgumentKnown::U3(segment.ident.span()))), None, None),
            Vector4 => (Some(NArgument::Known(NArgumentKnown::U4(segment.ident.span()))), None, None),
            VecN => (None, None, Some(AArgument::Known(AArgumentKnown::VecAligned(segment.ident.span())))),
            Vec2 => (Some(NArgument::Known(NArgumentKnown::U2(segment.ident.span()))), None, Some(AArgument::Known(AArgumentKnown::VecAligned(segment.ident.span())))),
            Vec3 => (Some(NArgument::Known(NArgumentKnown::U3(segment.ident.span()))), None, Some(AArgument::Known(AArgumentKnown::VecAligned(segment.ident.span())))),
            Vec4 => (Some(NArgument::Known(NArgumentKnown::U4(segment.ident.span()))), None, Some(AArgument::Known(AArgumentKnown::VecAligned(segment.ident.span())))),
            VecNP => (None, None, Some(AArgument::Known(AArgumentKnown::VecPacked(segment.ident.span())))),
            Vec2P => (Some(NArgument::Known(NArgumentKnown::U2(segment.ident.span()))), None, Some(AArgument::Known(AArgumentKnown::VecPacked(segment.ident.span())))),
            Vec3P => (Some(NArgument::Known(NArgumentKnown::U3(segment.ident.span()))), None, Some(AArgument::Known(AArgumentKnown::VecPacked(segment.ident.span())))),
            Vec4P => (Some(NArgument::Known(NArgumentKnown::U4(segment.ident.span()))), None, Some(AArgument::Known(AArgumentKnown::VecPacked(segment.ident.span())))),
        )
    }
}
impl FromPerspective for ApiType {
    type Output = Type;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        match self {
            Self::Vec(ty) => ty.from_perspective(perspective),
            Self::T(span) => perspective.t(span),
            Self::Array(ty) => Type::Array(ty.from_perspective(perspective)),
            Self::Reference(ty) => Type::Reference(ty.from_perspective(perspective)),
            Self::Option(ty) => Type::Path(ty.from_perspective(perspective)),
            Self::Other(ty) => ty,
        }
    }
}
impl ApiType {
    pub fn outer_value_into_inner(&self, value: &Ident) -> Expr {
        match self {
            Self::Vec(_) => parse_quote_spanned! { value.span() => #value.inner },
            Self::T(_) => parse_quote_spanned! { value.span() => #value },
            Self::Array(ApiTypeArray {
                bracket_token: _,
                elem,
                semi_token: _,
                len: _,
            }) => {
                let f = elem.outer_value_into_inner(&Ident::new("item", value.span()));
                parse_quote_spanned! { value.span() => #value.map(|item| #f) }
            }
            Self::Reference(_) => {
                parse_quote_spanned! { value.span() => unsafe { std::mem::transmute(#value) } }
            }
            Self::Option(ApiTypeOption {
                ident_span: _,
                inner,
            }) => {
                let f = inner.outer_value_into_inner(&Ident::new("some", value.span()));
                parse_quote_spanned! { value.span() => #value.map(|some| #f) }
            }
            Self::Other(_) => parse_quote_spanned! { value.span() => #value },
        }
    }
    pub fn inner_value_into_outer(&self, value: &Ident) -> Expr {
        match self {
            Self::Vec(vec) => {
                let n = vec.n.clone().from_perspective(Perspective::Vec);
                let t = vec.t.clone().from_perspective(Perspective::Vec);
                let a = vec.a.clone().from_perspective(Perspective::Vec);
                parse_quote_spanned! { value.span() => Vector::<#n, #t, #a> { inner: #value } }
            }
            Self::T(_) => parse_quote_spanned! { value.span() => #value },
            Self::Array(ApiTypeArray {
                bracket_token: _,
                elem,
                semi_token: _,
                len: _,
            }) => {
                let f = elem.inner_value_into_outer(&Ident::new("item", value.span()));
                parse_quote_spanned! { value.span() => #value.map(|item| #f) }
            }
            Self::Reference(_) => {
                parse_quote_spanned! { value.span() => unsafe { std::mem::transmute(#value) } }
            }
            Self::Option(ApiTypeOption {
                ident_span: _,
                inner,
            }) => {
                let f = inner.inner_value_into_outer(&Ident::new("some", value.span()));
                parse_quote_spanned! { value.span() => #value.map(|some| #f) }
            }
            Self::Other(_) => parse_quote_spanned! { value.span() => #value },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeVec {
    ident_span: Span,
    n: NArgument,
    t: TArgument,
    a: AArgument,
}
impl ApiTypeVec {
    fn try_from_arguments(
        segment: PathSegment,
        n: Option<NArgument>,
        t: Option<TArgument>,
        a: Option<AArgument>,
    ) -> Result<Self, Error> {
        let mut arguments = match segment.arguments {
            PathArguments::AngleBracketed(arguments) => arguments.args,
            PathArguments::None => Punctuated::new(),
            PathArguments::Parenthesized(_) => {
                panic!("unexpected parenthesized generics for vector")
            }
        }
        .into_iter();

        let n = n.map_or_else(
            || match arguments.next().unwrap() {
                GenericArgument::Const(argument) => Ok(argument.into()),
                GenericArgument::AssocConst(argument) => Ok(argument.value.into()),
                GenericArgument::Type(argument) => argument.try_into(),
                _ => panic!("expected const generic argument for N"),
            },
            |n| Ok(n),
        )?;

        let t = t.unwrap_or_else(|| match arguments.next().unwrap() {
            GenericArgument::Type(argument) => argument.into(),
            GenericArgument::AssocType(argument) => argument.ty.into(),
            _ => panic!("expected const generic argument for T"),
        });

        let a = a.unwrap_or_else(|| match arguments.next().unwrap() {
            GenericArgument::Type(argument) => argument.into(),
            GenericArgument::AssocType(argument) => argument.ty.into(),
            _ => panic!("expected const generic argument for A"),
        });

        Ok(Self {
            ident_span: segment.ident.span(),
            n,
            t,
            a,
        })
    }
}
impl FromPerspective for ApiTypeVec {
    type Output = Type;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        if perspective != Perspective::Vec {
            let n = self.n.from_perspective(perspective);
            let t = self.t.from_perspective(perspective);
            let a = self.a.from_perspective(perspective);

            return parse_quote_spanned! { self.ident_span => crate::vec::inner::InnerVector<#n, #t, #a> };
        }

        if let NArgument::SelfVec(_) = self.n {
            if let TArgument::SelfVec(_) = self.t {
                if let AArgument::SelfVec(_) = self.a {
                    return parse_quote_spanned! { self.ident_span => Self };
                }
            }
        }

        let t = self.t.from_perspective(perspective);
        match self.n {
            NArgument::Known(NArgumentKnown::U2(_)) => match self.a {
                AArgument::Known(AArgumentKnown::VecAligned(_)) => {
                    parse_quote_spanned! { self.ident_span => crate::vec::Vec2<#t> }
                }
                AArgument::Known(AArgumentKnown::VecPacked(_)) => {
                    parse_quote_spanned! { self.ident_span => crate::vec::Vec2P<#t> }
                }
                a => {
                    let a = a.from_perspective(perspective);
                    parse_quote_spanned! { self.ident_span => crate::vec::Vector2<#t, #a> }
                }
            },
            NArgument::Known(NArgumentKnown::U3(_)) => match self.a {
                AArgument::Known(AArgumentKnown::VecAligned(_)) => {
                    parse_quote_spanned! { self.ident_span => crate::vec::Vec3<#t> }
                }
                AArgument::Known(AArgumentKnown::VecPacked(_)) => {
                    parse_quote_spanned! { self.ident_span => crate::vec::Vec3P<#t> }
                }
                a => {
                    let a = a.from_perspective(perspective);
                    parse_quote_spanned! { self.ident_span => crate::vec::Vector3<#t, #a> }
                }
            },
            NArgument::Known(NArgumentKnown::U4(_)) => match self.a {
                AArgument::Known(AArgumentKnown::VecAligned(_)) => {
                    parse_quote_spanned! { self.ident_span => crate::vec::Vec4<#t> }
                }
                AArgument::Known(AArgumentKnown::VecPacked(_)) => {
                    parse_quote_spanned! { self.ident_span => crate::vec::Vec4P<#t> }
                }
                a => {
                    let a = a.from_perspective(perspective);
                    parse_quote_spanned! { self.ident_span => crate::vec::Vector4<#t, #a> }
                }
            },
            n => {
                let n = n.from_perspective(perspective);
                match self.a {
                    AArgument::Known(AArgumentKnown::VecAligned(_)) => {
                        parse_quote_spanned! { self.ident_span => crate::vec::VecN<#n, #t> }
                    }
                    AArgument::Known(AArgumentKnown::VecPacked(_)) => {
                        parse_quote_spanned! { self.ident_span => crate::vec::VecNP<#n, #t> }
                    }
                    a => {
                        let a = a.from_perspective(perspective);
                        parse_quote_spanned! { self.ident_span => crate::vec::Vector<#n, #t, #a> }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeArray {
    bracket_token: Bracket,
    elem: Box<ApiType>,
    semi_token: Token![;],
    len: NArgument,
}
impl TryFrom<TypeArray> for ApiTypeArray {
    type Error = Error;
    fn try_from(value: TypeArray) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            bracket_token: value.bracket_token,
            elem: Box::new((*value.elem).try_into()?),
            semi_token: value.semi_token,
            len: value.len.into(),
        })
    }
}
impl FromPerspective for ApiTypeArray {
    type Output = TypeArray;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        TypeArray {
            bracket_token: self.bracket_token,
            elem: Box::new(self.elem.from_perspective(perspective)),
            semi_token: self.semi_token,
            len: self.len.from_perspective(perspective),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeOption {
    ident_span: Span,
    inner: Box<ApiType>,
}
impl TryFrom<PathSegment> for ApiTypeOption {
    type Error = Error;
    fn try_from(value: PathSegment) -> Result<Self, Self::Error> {
        if value.ident.to_string().as_str() != "Option" {
            return Err(Error::new(value.ident.span(), "expected Option"));
        };

        let args = match value.arguments {
            PathArguments::AngleBracketed(angled) => {
                angled.args.into_iter().collect::<Box<[GenericArgument]>>()
            }
            _ => return Err(Error::new(value.ident.span(), "expected Option<...>")),
        };

        let inner = match &*args {
            [arg] => match arg {
                GenericArgument::Type(inner) => inner.clone().try_into()?,
                _ => return Err(Error::new(value.ident.span(), "expected Option<'type'>")),
            },
            _ => return Err(Error::new(value.ident.span(), "expected Option<'type'>")),
        };

        Ok(Self {
            ident_span: value.ident.span(),
            inner: Box::new(inner),
        })
    }
}
impl FromPerspective for ApiTypeOption {
    type Output = TypePath;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        let inner = self.inner.from_perspective(perspective);

        parse_quote_spanned! { self.ident_span => Option<#inner> }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeReference {
    and_token: Token![&],
    lifetime: Option<Lifetime>,
    mutability: Option<Token![mut]>,
    elem: Box<ApiType>,
}
impl TryFrom<TypeReference> for ApiTypeReference {
    type Error = Error;
    fn try_from(value: TypeReference) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            and_token: value.and_token,
            lifetime: value.lifetime,
            mutability: value.mutability,
            elem: Box::new((*value.elem).try_into()?),
        })
    }
}
impl FromPerspective for ApiTypeReference {
    type Output = TypeReference;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        TypeReference {
            and_token: self.and_token,
            lifetime: self.lifetime,
            mutability: self.mutability,
            elem: Box::new((*self.elem).from_perspective(perspective)),
        }
    }
}
