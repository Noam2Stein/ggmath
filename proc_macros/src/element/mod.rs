use derive_syn_parse::Parse;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Token, Type};

pub fn impl_element_inner_vecs(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        t: Type,
        _0: Token![:],
        vec2: Type,
        _1: Token![,],
        vec3: Type,
        _2: Token![,],
        vec4: Type,
        _3: Option<Token![,]>,
    }

    let Input {
        t,
        _0,
        vec2,
        _1,
        vec3,
        _2,
        vec4,
        _3,
    } = parse_macro_input!(tokens as Input);

    let impl_sig = quote_spanned! {
        t.span() =>
        unsafe impl ggmath::element::ElementInnerVecs for #t
    };
    let impl_vec2 = quote_spanned! {
        vec2.span() =>
        type InnerVec2 = #vec2;
    };
    let impl_vec3 = quote_spanned! {
        vec3.span() =>
        type InnerVec3 = #vec3;
    };
    let impl_vec4 = quote_spanned! {
        vec4.span() =>
        type InnerVec4 = #vec4;
    };
    let validate_vec2 = quote_spanned! {
        vec2.span() =>
        unsafe fn validate_vec2(value: <#t as ggmath::element::ElementInnerVecs>::InnerVec2) -> [$self; 2] { std::mem::transmute(value) }
    };
    let validate_vec3 = quote_spanned! {
        vec3.span() =>
        unsafe fn validate_vec3(value: <#t as ggmath::element::ElementInnerVecs>::InnerVec3) -> [$self; 4] { std::mem::transmute(value) }
    };
    let validate_vec4 = quote_spanned! {
        vec4.span() =>
        unsafe fn validate_vec4(value: <#t as ggmath::element::ElementInnerVecs>::InnerVec4) -> [$self; 4] { std::mem::transmute(value) }
    };

    quote! {
        #impl_sig {
            #impl_vec2
            #impl_vec3
            #impl_vec4
        }
        const _: () = {
            #validate_vec2
            #validate_vec3
            #validate_vec4
        };
    }
    .into()
}

pub fn impl_element_vecs_from_splits_transmute(
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        t: Type,
    }

    let Input { t } = parse_macro_input!(tokens as Input);

    quote_spanned! {
        t.span() =>
        impl ggmath::element::ElementVecsFromSplits for Ty {
            fn from_x_y(value: (Self, Self)) -> Self::InnerVec2 {
                unsafe { std::mem::transmute(value) }
            }
            fn from_x_y_z(value: (Self, Self, Self)) -> Self::InnerVec3 {
                unsafe { std::mem::transmute((value, Self::default())) }
            }
            fn from_xy_z(value: (Self::InnerVec2, Self)) -> Self::InnerVec3 {
                unsafe { std::mem::transmute((value, Self::default())) }
            }
            fn from_x_yz(value: (Self, Self::InnerVec2)) -> Self::InnerVec3 {
                unsafe { std::mem::transmute((value, Self::default())) }
            }
            fn from_x_y_z_w(value: (Self, Self, Self, Self)) -> Self::InnerVec4 {
                unsafe { std::mem::transmute(value) }
            }
            fn from_xy_z_w(value: (Self::InnerVec2, Self, Self)) -> Self::InnerVec4 {
                unsafe { std::mem::transmute(value) }
            }
            fn from_x_yz_w(value: (Self, Self::InnerVec2, Self)) -> Self::InnerVec4 {
                unsafe { std::mem::transmute(value) }
            }
            fn from_x_y_zw(value: (Self, Self, Self::InnerVec2)) -> Self::InnerVec4 {
                unsafe { std::mem::transmute(value) }
            }
            fn from_xy_zw(value: (Self::InnerVec2, Self::InnerVec2)) -> Self::InnerVec4 {
                unsafe { std::mem::transmute(value) }
            }
            fn from_xyz_w(value: (Self::InnerVec3, Self)) -> Self::InnerVec4 {
                unsafe {
                    std::mem::transmute((
                        std::mem::transmute_copy::<Self::InnerVec3, [Self; 3]>(&value.0),
                        value.1,
                    ))
                }
            }
            fn from_x_yzw(value: (Self, Self::InnerVec3)) -> Self::InnerVec4 {
                unsafe {
                    std::mem::transmute((
                        value.0,
                        std::mem::transmute_copy::<Self::InnerVec3, [Self; 3]>(&value.1),
                    ))
                }
            }
        }
    }
    .into()
}
