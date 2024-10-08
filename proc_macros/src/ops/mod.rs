use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

mod assign_ops;
mod ops_macro;
mod rhs_ops;
mod self_ops;
pub use assign_ops::*;
pub use ops_macro::*;
pub use rhs_ops::*;
pub use self_ops::*;

const SELF_OPS: &[&'static str] = &["Neg", "Not"];
const RHS_OPS: &[&'static str] = &[
    "Add", "Sub", "Mul", "Div", "Rem", "BitAnd", "BitOr", "BitXor", "Shr", "Shl",
];

fn op_fragments(
    element_trait: impl ToTokens,
    std_trait: impl ToTokens,
    std_fn: impl ToTokens,
    vec_trait: impl ToTokens,
    vec_fn: impl ToTokens,
    vecnum_trait: impl ToTokens,
) -> TokenStream {
    quote! { #element_trait #std_trait #std_fn #vec_trait #vec_fn #vecnum_trait }
}
