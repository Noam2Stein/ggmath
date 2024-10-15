use std::fmt::{self, Display, Formatter};

use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use syn::Ident;

pub struct ConstIdent {
    pub s: &'static str,
}
impl Display for ConstIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}
impl ToTokens for ConstIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::new(&self.s, Span::call_site()));
    }
}

macro_rules! idents {
    ($($ident:ident), * $(,)?) => {
        $(
            #[allow(dead_code)]
            #[allow(non_upper_case_globals)]
            pub const $ident: ConstIdent = ConstIdent { s: stringify!($ident) };
        )*
    };
}
idents!(
    gomath,
    Vector,
    Vector2,
    Vector3,
    Vector4,
    VecN,
    Vec2,
    Vec3,
    Vec4,
    VecNP,
    Vec2P,
    Vec3P,
    Vec4P,
    InnerVector,
    N,
    T,
    A,
    VecLen,
    Scalar,
    VecAlignment,
    ScalarCount,
    VecAligned,
    VecPacked,
    ScalarInnerVecs,
    VecLenInnerVecs,
    VecAlignmentInnerVecs,
    ScalarDefaultImpl,
    ScalarVec,
);
