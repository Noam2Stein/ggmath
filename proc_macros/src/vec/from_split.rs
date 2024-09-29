use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, Type};

pub fn impl_from_splits_transmute(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = parse_macro_input!(tokens as Type);

    quote_spanned! {
        ty.span() =>
        impl ggmath::FromVec2Splits for ggmath::inner::Vec2Inner<#ty> {}
        impl ggmath::FromVec3Splits for ggmath::inner::Vec3Inner<#ty> {}
        impl ggmath::FromVec4Splits for ggmath::inner::Vec4Inner<#ty> {}

        impl ggmath::FromVecSplit<(#ty, #ty)> for ggmath::inner::Vec2Inner<#ty> {
            fn from_split(split: (#ty, #ty)) -> Self {
                unsafe {
                    std::mem::transmute(split)
                }
            }
        }
    }
    .into()
}
