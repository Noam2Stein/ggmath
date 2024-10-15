use proc_macro2::Literal;
use syn::{parse_quote, Expr, Type};

#[derive(Debug, Clone)]
pub enum Perspective {
    Vec,
    Scalar,
    Len(Option<Literal>),
    Alignment(Option<Literal>),
}
impl Perspective {
    pub fn n(&self) -> Expr {
        match self {
            Self::Vec => parse_quote! { N },
            Self::Scalar => parse_quote! { N },
            Self::Len(n) => n
                .as_ref()
                .map_or_else(|| parse_quote! { N }, |n| parse_quote! { #n }),
            Self::Alignment(n) => n
                .as_ref()
                .map_or_else(|| parse_quote! { N }, |n| parse_quote! { #n }),
        }
    }
    pub fn t(&self) -> Type {
        match self {
            Self::Vec => parse_quote! { T },
            Self::Scalar => parse_quote! { Self },
            Self::Len(_) => parse_quote! { T },
            Self::Alignment(_) => parse_quote! { T },
        }
    }
    pub fn a(&self) -> Type {
        match self {
            Self::Vec => parse_quote! { S },
            Self::Scalar => parse_quote! { S },
            Self::Len(_) => parse_quote! { S },
            Self::Alignment(_) => parse_quote! { Self },
        }
    }
}

pub trait FromPerspective {
    type Output;
    fn from_perspective(self, perspective: &Perspective) -> Self::Output;
}
