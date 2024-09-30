use derive_syn_parse::Parse;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, token::For, Type};

#[derive(Parse)]
struct Input {
    split: Type,
    _for: For,
    self_ty: Type,
}

pub fn impl_from_split_transmute(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        split,
        _for,
        self_ty,
    } = parse_macro_input!(tokens as Input);

    quote_spanned! {
        self_ty.span() =>
        impl ggmath::FromVecSplit<#split> for #self_ty {
            fn from_split(split: #split) -> Self {
                unsafe {
                    std::mem::transmute(split)
                }
            }
        }
    }
    .into()
}

pub fn impl_from_splits_transmute(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = parse_macro_input!(tokens as Type);

    quote_spanned! {
        ty.span() =>
        impl ggmath::FromVec2Splits<#ty> for ggmath::inner::Vec2Inner<#ty> {}
        impl ggmath::FromVec3Splits<#ty> for ggmath::inner::Vec3Inner<#ty> {}
        impl ggmath::FromVec4Splits<#ty> for ggmath::inner::Vec4Inner<#ty> {}

        ggmath::impl_from_split_transmute!((#ty, #ty) for ggmath::inner::Vec2Inner<#ty>);
        ggmath::impl_from_split_transmute!(ggmath::Vec2<#ty> for ggmath::inner::Vec2Inner<#ty>);

        ggmath::impl_from_split_transmute!((#ty, #ty, #ty) for ggmath::inner::Vec3Inner<#ty>);
        ggmath::impl_from_split_transmute!((ggmath::Vec2<#ty>, #ty) for ggmath::inner::Vec3Inner<#ty>);
        ggmath::impl_from_split_transmute!((#ty, ggmath::Vec2<#ty>) for ggmath::inner::Vec3Inner<#ty>);
        ggmath::impl_from_split_transmute!(ggmath::Vec3<#ty> for ggmath::inner::Vec3Inner<#ty>);

        ggmath::impl_from_split_transmute!((#ty, #ty, #ty, #ty) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!((ggmath::Vec2<#ty>, #ty, #ty) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!((#ty, ggmath::Vec2<#ty>, #ty) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!((#ty, #ty, ggmath::Vec2<#ty>) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!((ggmath::Vec2<#ty>, ggmath::Vec2<#ty>) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!((#ty, ggmath::Vec3<#ty>) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!((ggmath::Vec3<#ty>, #ty) for ggmath::inner::Vec4Inner<#ty>);
        ggmath::impl_from_split_transmute!(ggmath::Vec4<#ty> for ggmath::inner::Vec4Inner<#ty>);
    }
    .into()
}
