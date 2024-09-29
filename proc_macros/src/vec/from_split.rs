use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, Type};

pub fn impl_from_splits_transmute(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = parse_macro_input!(tokens as Type);

    quote_spanned! {
        ty.span() =>
        impl ggmath::FromVec2Splits for ggmath::Vec2Inner<#ty> {}
        impl ggmath::FromVec3Splits for ggmath::Vec3Inner<#ty> {}
        impl ggmath::FromVec4Splits for ggmath::Vec4Inner<#ty> {}

        impl ggmath::FromVecSplit<(#t, #t)> for ggmath::Vec2Inner<#ty>
    }.into()
}