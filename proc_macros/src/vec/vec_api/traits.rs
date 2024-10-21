use super::*;

pub fn scalar_trait_ident(api_ident: &Ident) -> Ident {
    Ident::new(&format!("ScalarVec{api_ident}Api"), api_ident.span())
}
pub fn len_trait_ident(api_ident: &Ident) -> Ident {
    Ident::new(&format!("VecLen{api_ident}Api"), api_ident.span())
}
pub fn alignment_trait_ident(api_ident: &Ident) -> Ident {
    Ident::new(&format!("VecAlignment{api_ident}Api"), api_ident.span())
}
