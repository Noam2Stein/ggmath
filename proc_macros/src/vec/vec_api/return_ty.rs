use super::{perspective::*, ty::*, *};

use syn::{parse_quote, Expr, ReturnType};

#[derive(Clone)]
pub enum ApiReturnType {
    Default,
    Type(ApiType),
}
impl TryFrom<ReturnType> for ApiReturnType {
    type Error = Error;
    fn try_from(value: ReturnType) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            ReturnType::Default => Self::Default,
            ReturnType::Type(_, ty) => Self::Type((*ty).try_into()?),
        })
    }
}
impl FromPerspective for ApiReturnType {
    type Output = ReturnType;
    fn from_perspective(self, perspective: Perspective) -> Self::Output {
        match self {
            Self::Default => ReturnType::Default,
            Self::Type(ty) => {
                let ty = ty.from_perspective(perspective);
                ReturnType::Type(parse_quote_spanned! { ty.span() => -> }, Box::new(ty))
            }
        }
    }
}
impl ApiReturnType {
    pub fn pass_into_outer(&self, value: &Ident) -> Expr {
        match self {
            Self::Default => parse_quote_spanned! { value.span() => #value },
            Self::Type(ty) => ty.inner_value_into_outer(value),
        }
    }
    pub fn pass_from_perspective(&self, value: &Ident, perspective: Perspective) -> Expr {
        if let Perspective::Vec = perspective {
            self.pass_into_outer(value)
        } else {
            parse_quote!(#value)
        }
    }
}
