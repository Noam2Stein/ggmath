use super::*;

use quote::quote;

use super::for_rhs_ops::RHS_OPS;

pub fn for_assign_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let ops = RHS_OPS.into_iter().map(|op_str| {
        let std_trait = Ident::new(&format!("{op_str}Assign"), Span::call_site());
        let std_fn = Ident::new(
            &format!("{}_assign", op_str.to_lowercase()),
            Span::call_site(),
        );
        let scalar_trait = Ident::new(&format!("Scalar{op_str}Assign"), Span::call_site());

        quote! {
            #std_trait #std_fn #scalar_trait
        }
    });

    quote! {
        macro_rules! some_rhs_ops_macro {
            ($($std_trait:ident $std_fn:ident $scalar_trait:ident)*) => { #input }
        }
        some_rhs_ops_macro!(#(#ops)*);
    }
    .into()
}
