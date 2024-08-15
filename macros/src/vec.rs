use proc_macro2::Span;
use strum_macros::EnumIter;
use syn::{parse_quote, Expr, Ident, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum VecType {
    SingleElement,
    Vec2,
    Vec3,
    Vec3A,
    Vec4,
}
impl VecType {
    pub fn len(self) -> usize {
        match self {
            Self::SingleElement => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec3A => 3,
            Self::Vec4 => 4,
        }
    }
    pub fn ident_str(self) -> &'static str {
        match self {
            Self::SingleElement => "Element",
            Self::Vec2 => "Vec2",
            Self::Vec3 => "Vec3",
            Self::Vec3A => "Vec3A",
            Self::Vec4 => "Vec4",
        }
    }
    pub fn ident_lower_str(self) -> &'static str {
        match self {
            Self::SingleElement => "element",
            Self::Vec2 => "vec2",
            Self::Vec3 => "vec3",
            Self::Vec3A => "vec3a",
            Self::Vec4 => "vec4",
        }
    }
    pub fn ty_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => element_ty.to_string(),
            Self::Vec2 => format!("Vec2::<{element_ty}>"),
            Self::Vec3 => format!("Vec3::<{element_ty}>"),
            Self::Vec3A => format!("Vec3A::<{element_ty}>"),
            Self::Vec4 => format!("Vec4::<{element_ty}>"),
        }
    }
    pub fn inner_ty_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => element_ty.to_string(),
            Self::Vec2 => format!("<{element_ty} as Element>::Vec2Inner"),
            Self::Vec3 => format!("<{element_ty} as Element>::Vec3Inner"),
            Self::Vec3A => format!("<{element_ty} as Element>::Vec3AInner"),
            Self::Vec4 => format!("<{element_ty} as Element>::Vec4Inner"),
        }
    }
    pub fn wrap_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => format!("<{element_ty} as Element>::wrap_element"),
            Self::Vec2 => format!("<{element_ty} as Element>::wrap_vec2"),
            Self::Vec3 => format!("<{element_ty} as Element>::wrap_vec3"),
            Self::Vec3A => format!("<{element_ty} as Element>::wrap_vec3a"),
            Self::Vec4 => format!("<{element_ty} as Element>::wrap_vec4"),
        }
    }
    pub fn wrap_ref_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => format!("<{element_ty} as Element>::wrap_element_ref"),
            Self::Vec2 => format!("<{element_ty} as Element>::wrap_vec2_ref"),
            Self::Vec3 => format!("<{element_ty} as Element>::wrap_vec3_ref"),
            Self::Vec3A => format!("<{element_ty} as Element>::wrap_vec3a_ref"),
            Self::Vec4 => format!("<{element_ty} as Element>::wrap_vec4_ref"),
        }
    }
    pub fn wrap_mut_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => format!("<{element_ty} as Element>::wrap_element_mut"),
            Self::Vec2 => format!("<{element_ty} as Element>::wrap_vec2_mut"),
            Self::Vec3 => format!("<{element_ty} as Element>::wrap_vec3_mut"),
            Self::Vec3A => format!("<{element_ty} as Element>::wrap_vec3a_mut"),
            Self::Vec4 => format!("<{element_ty} as Element>::wrap_vec4_mut"),
        }
    }
    pub fn unwrap_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => format!("<{element_ty} as Element>::unwrap_element"),
            Self::Vec2 => format!("<{element_ty} as Element>::unwrap_vec2"),
            Self::Vec3 => format!("<{element_ty} as Element>::unwrap_vec3"),
            Self::Vec3A => format!("<{element_ty} as Element>::unwrap_vec3a"),
            Self::Vec4 => format!("<{element_ty} as Element>::unwrap_vec4"),
        }
    }
    pub fn unwrap_ref_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => format!("<{element_ty} as Element>::unwrap_element_ref"),
            Self::Vec2 => format!("<{element_ty} as Element>::unwrap_vec2_ref"),
            Self::Vec3 => format!("<{element_ty} as Element>::unwrap_vec3_ref"),
            Self::Vec3A => format!("<{element_ty} as Element>::unwrap_vec3a_ref"),
            Self::Vec4 => format!("<{element_ty} as Element>::unwrap_vec4_ref"),
        }
    }
    pub fn unwrap_mut_str(self, element_ty: &str) -> String {
        match self {
            Self::SingleElement => format!("<{element_ty} as Element>::unwrap_element_mut"),
            Self::Vec2 => format!("<{element_ty} as Element>::unwrap_vec2_mut"),
            Self::Vec3 => format!("<{element_ty} as Element>::unwrap_vec3_mut"),
            Self::Vec3A => format!("<{element_ty} as Element>::unwrap_vec3a_mut"),
            Self::Vec4 => format!("<{element_ty} as Element>::unwrap_vec4_mut"),
        }
    }
    pub fn ident(self) -> Ident {
        Ident::new(self.ident_str(), Span::call_site())
    }
    pub fn ident_lower(self) -> Ident {
        Ident::new(self.ident_lower_str(), Span::call_site())
    }
    pub fn ty(self, element_ty: &Type) -> Type {
        match self {
            Self::SingleElement => parse_quote! { #element_ty },
            Self::Vec2 => parse_quote! { Vec2::<#element_ty> },
            Self::Vec3 => parse_quote! { Vec3::<#element_ty> },
            Self::Vec3A => parse_quote! { Vec3A::<#element_ty> },
            Self::Vec4 => parse_quote! { Vec4::<#element_ty> },
        }
    }
    pub fn inner_ty(self, element_ty: &Type) -> Type {
        match self {
            Self::SingleElement => parse_quote! { #element_ty },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::Vec2Inner },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::Vec3Inner },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::Vec3AInner },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::Vec4Inner },
        }
    }
    pub fn wrap(self, element_ty: &Type) -> Expr {
        match self {
            Self::SingleElement => parse_quote! { <#element_ty as Element>::wrap_element },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::wrap_vec2 },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::wrap_vec3 },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::wrap_vec3a },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::wrap_vec4 },
        }
    }
    pub fn wrap_ref(self, element_ty: &Type) -> Expr {
        match self {
            Self::SingleElement => parse_quote! { <#element_ty as Element>::wrap_element_ref },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::wrap_vec2_ref },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::wrap_vec3_ref },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::wrap_vec3a_ref },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::wrap_vec4_ref },
        }
    }
    pub fn wrap_mut(self, element_ty: &Type) -> Expr {
        match self {
            Self::SingleElement => parse_quote! { <#element_ty as Element>::wrap_element_mut },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::wrap_vec2_mut },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::wrap_vec3_mut },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::wrap_vec3a_mut },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::wrap_vec4_mut },
        }
    }
    pub fn unwrap(self, element_ty: &Type) -> Expr {
        match self {
            Self::SingleElement => parse_quote! { <#element_ty as Element>::unwrap_element },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::unwrap_vec2 },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::unwrap_vec3 },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::unwrap_vec3a },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::unwrap_vec4 },
        }
    }
    pub fn unwrap_ref(self, element_ty: &Type) -> Expr {
        match self {
            Self::SingleElement => parse_quote! { <#element_ty as Element>::unwrap_element_ref },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::unwrap_vec2_ref },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::unwrap_vec3_ref },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::unwrap_vec3a_ref },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::unwrap_vec4_ref },
        }
    }
    pub fn unwrap_mut(self, element_ty: &Type) -> Expr {
        match self {
            Self::SingleElement => parse_quote! { <#element_ty as Element>::unwrap_element_mut },
            Self::Vec2 => parse_quote! { <#element_ty as Element>::unwrap_vec2_mut },
            Self::Vec3 => parse_quote! { <#element_ty as Element>::unwrap_vec3_mut },
            Self::Vec3A => parse_quote! { <#element_ty as Element>::unwrap_vec3a_mut },
            Self::Vec4 => parse_quote! { <#element_ty as Element>::unwrap_vec4_mut },
        }
    }
}