use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Expr, Ident, PathSegment, Type};

use crate::idents::*;

use super::{FromPerspective, Perspective};

#[derive(Debug, Clone)]
pub enum NArgument {
    SelfVec,
    Known(KnownVecLen),
    Generic(Expr),
}
impl From<Expr> for NArgument {
    fn from(value: Expr) -> Self {
        if let Expr::Path(value) = &value {
            match *value.path.segments.iter().collect::<Box<[&PathSegment]>>() {
                [segment] => {
                    if segment.ident.to_string() == N.s && segment.arguments.is_empty() {
                        return Self::SelfVec;
                    }
                }
                _ => {}
            }
        };

        Self::Generic(value)
    }
}
impl FromPerspective for NArgument {
    type Output = Expr;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match self {
            Self::SelfVec => perspective.n(),
            Self::Known(known) => parse_quote!(#known),
            Self::Generic(generic) => generic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnownVecLen {
    U2,
    U3,
    U4,
}
impl ToTokens for KnownVecLen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::usize_unsuffixed(self.into_usize()));
    }
}
impl KnownVecLen {
    pub fn into_usize(self) -> usize {
        match self {
            Self::U2 => 2,
            Self::U3 => 3,
            Self::U4 => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TArgument {
    SelfVec,
    Specific(Type),
}
impl From<Type> for TArgument {
    fn from(value: Type) -> Self {
        if let Type::Path(value) = &value {
            match *value.path.segments.iter().collect::<Box<[&PathSegment]>>() {
                [segment] => {
                    if segment.ident.to_string() == T.s && segment.arguments.is_empty() {
                        return Self::SelfVec;
                    }
                }
                _ => {}
            }
        };

        Self::Specific(value)
    }
}
impl FromPerspective for TArgument {
    type Output = Type;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match self {
            Self::SelfVec => perspective.t(),
            Self::Specific(specific) => specific,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AArgument {
    SelfVec,
    Known(KnownVecAlignment),
    Generic(Type),
}
impl From<Type> for AArgument {
    fn from(value: Type) -> Self {
        if let Type::Path(value) = &value {
            match *value.path.segments.iter().collect::<Box<[&PathSegment]>>() {
                [segment] => {
                    if segment.ident.to_string() == A.s && segment.arguments.is_empty() {
                        return Self::SelfVec;
                    }
                }
                _ => {}
            }
        };

        Self::Generic(value)
    }
}
impl FromPerspective for AArgument {
    type Output = Type;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output {
        match self {
            Self::SelfVec => perspective.a(),
            Self::Known(known) => parse_quote!(#known),
            Self::Generic(generic) => generic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnownVecAlignment {
    VecAligned,
    VecPacked,
}
impl ToTokens for KnownVecAlignment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::new(self.as_str(), Span::call_site()));
    }
}
impl KnownVecAlignment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VecAligned => VecAligned.s,
            Self::VecPacked => VecPacked.s,
        }
    }
}
