use super::*;

use syn::{Expr, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Perspective {
    Vec,
    Scalar,
    Len(Option<KnownN>),
    Alignment(Option<KnownN>),
}
impl Perspective {
    pub fn n(&self, span: Span) -> Expr {
        match self {
            Self::Vec => parse_quote_spanned! { span => N },
            Self::Scalar => parse_quote_spanned! { span => N },
            Self::Len(n) => n.as_ref().map_or_else(
                || parse_quote_spanned! { span => N },
                |n| parse_quote_spanned! { span => #n },
            ),
            Self::Alignment(n) => n.as_ref().map_or_else(
                || parse_quote_spanned! { span => N },
                |n| parse_quote_spanned! { span => #n },
            ),
        }
    }
    pub fn t(&self, span: Span) -> Type {
        match self {
            Self::Vec => parse_quote_spanned! { span => T },
            Self::Scalar => parse_quote_spanned! { span => Self },
            Self::Len(_) => parse_quote_spanned! { span => T },
            Self::Alignment(_) => parse_quote_spanned! { span => T },
        }
    }
    pub fn a(&self, span: Span) -> Type {
        match self {
            Self::Vec => parse_quote_spanned! { span => A },
            Self::Scalar => parse_quote_spanned! { span => A },
            Self::Len(_) => parse_quote_spanned! { span => A },
            Self::Alignment(_) => parse_quote_spanned! { span => Self },
        }
    }
}

pub trait FromPerspective {
    type Output;
    fn from_perspective(self, perspective: Perspective) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnownN(pub usize);
impl ToTokens for KnownN {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut literal = Literal::usize_unsuffixed(self.0);
        literal.set_span(tokens.span());
        literal.to_tokens(tokens);
    }
}
