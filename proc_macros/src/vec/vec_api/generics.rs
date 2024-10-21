use super::{perspective::*, *};

use syn::{Expr, ExprLit, Lit, LitInt, PathSegment, Type};

#[derive(Debug, Clone)]
pub enum NArgument {
    SelfVec(Span),
    Known(NArgumentKnown),
    Generic(Expr),
}
impl From<Expr> for NArgument {
    fn from(value: Expr) -> Self {
        if let Expr::Path(value) = &value {
            match *value.path.segments.iter().collect::<Box<[&PathSegment]>>() {
                [segment] => {
                    if segment.ident.to_string() == "N" {
                        return Self::SelfVec(value.span());
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
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        match self {
            Self::SelfVec(span) => perspective.n(span),
            Self::Known(known) => Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: known.from_perspective(perspective),
            }),
            Self::Generic(generic) => generic,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NArgumentKnown {
    U2(Span),
    U3(Span),
    U4(Span),
}
impl FromPerspective for NArgumentKnown {
    type Output = Lit;
    fn from_perspective(self, _perspective: Perspective) -> Self::Output {
        match self {
            Self::U2(span) => Lit::Int(LitInt::new("2", span)),
            Self::U3(span) => Lit::Int(LitInt::new("3", span)),
            Self::U4(span) => Lit::Int(LitInt::new("4", span)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TArgument {
    SelfVec(Span),
    Specific(Type),
}
impl From<Type> for TArgument {
    fn from(value: Type) -> Self {
        if let Type::Path(value) = &value {
            match *value.path.segments.iter().collect::<Box<[&PathSegment]>>() {
                [segment] => {
                    if segment.ident.to_string() == "T" {
                        return Self::SelfVec(value.span());
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
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        match self {
            Self::SelfVec(span) => perspective.t(span),
            Self::Specific(specific) => specific,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AArgument {
    SelfVec(Span),
    Known(AArgumentKnown),
    Generic(Type),
}
impl From<Type> for AArgument {
    fn from(value: Type) -> Self {
        if let Type::Path(value) = &value {
            match *value.path.segments.iter().collect::<Box<[&PathSegment]>>() {
                [segment] => {
                    if segment.ident.to_string() == "A" && segment.arguments.is_empty() {
                        return Self::SelfVec(value.span());
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
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        match self {
            Self::SelfVec(span) => perspective.a(span),
            Self::Known(known) => parse2(known.from_perspective(perspective).to_token_stream())
                .unwrap_or_else(|err| panic!("failed to parse known A argument: {err}")),
            Self::Generic(generic) => generic,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AArgumentKnown {
    VecAligned(Span),
    VecPacked(Span),
}
impl FromPerspective for AArgumentKnown {
    type Output = Ident;
    fn from_perspective(self, _perspective: Perspective) -> Self::Output {
        match self {
            Self::VecAligned(span) => Ident::new("VecAligned", span),
            Self::VecPacked(span) => Ident::new("VecPacked", span),
        }
    }
}
