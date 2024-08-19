use proc_macro2::{Literal, Span};
use strum_macros::{Display, EnumIter, EnumString};
use syn::{parse_quote, Ident, Type};

pub const COMPONENTS: [char; 4] = ['x', 'y', 'z', 'w'];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, EnumString, Display)]
pub enum VecType {
    Element,
    Vec2,
    Vec3,
    Vec3A,
    Vec4,
}
impl VecType {
    pub fn len(self) -> usize {
        match self {
            Self::Element => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec3A => 3,
            Self::Vec4 => 4,
        }
    }
    pub fn is_aligned(self) -> bool {
        self == Self::Vec3A
    }
    pub fn alen(self) -> usize {
        match self {
            Self::Element => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec3A => 4,
            Self::Vec4 => 4,
        }
    }
    pub fn ident(self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }
    pub fn ident_lower(self) -> Ident {
        Ident::new(&self.to_string().to_lowercase(), Span::call_site())
    }
    pub fn inner_ident(self) -> Ident {
        Ident::new(&format!("{self}Inner"), Span::call_site())
    }
    pub fn ty(self, element_ty: &Type) -> Type {
        match self {
            Self::Element => parse_quote! { #element_ty },
            Self::Vec2 => parse_quote! { Vec2::<#element_ty> },
            Self::Vec3 => parse_quote! { Vec3::<#element_ty> },
            Self::Vec3A => parse_quote! { Vec3A::<#element_ty> },
            Self::Vec4 => parse_quote! { Vec4::<#element_ty> },
        }
    }
    pub fn inner_ty(self, element_ty: &Type) -> Type {
        match self {
            Self::Element => parse_quote! { #element_ty },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::Vec2Inner },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::Vec3Inner },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::Vec3AInner },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::Vec4Inner },
        }
    }
    pub fn swizzle(self) -> Ident {
        Ident::new(&format!("{}_swizzle", self.to_string().to_lowercase()), Span::call_site())
    }
    pub fn get_swizzle(self) -> Ident {
        Ident::new(&format!("{}_get_swizzle", self.to_string().to_lowercase()), Span::call_site())
    }
    pub fn mut_swizzle(self) -> Ident {
        Ident::new(&format!("{}_mut_swizzle", self.to_string().to_lowercase()), Span::call_site())
    }
    pub fn set_swizzle(self) -> Ident {
        Ident::new(&format!("{}_set_swizzle", self.to_string().to_lowercase()), Span::call_site())
    }
    pub fn with_swizzle(self) -> Ident {
        Ident::new(&format!("{}_with_swizzle", self.to_string().to_lowercase()), Span::call_site())
    }
    pub fn display_literal(self) -> &'static str {
        match self {
            Self::Element => "{}",
            Self::Vec2 => "({}, {})",
            Self::Vec3 => "({}, {}, {})",
            Self::Vec3A => "({}, {}, {})",
            Self::Vec4 => "({}, {}, {}, {})",
        }
    }
    pub fn component_indicies(self) -> impl Iterator<Item = Literal> {
        (0..self.len()).map(|i| Literal::usize_suffixed(i))
    }
    pub fn components(self) -> impl Iterator<Item = Ident> {
        COMPONENTS[0..self.len()].iter().map(|c| Ident::new(&c.to_string(), Span::call_site()))
    }
}