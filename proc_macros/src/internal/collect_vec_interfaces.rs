use quote::quote;

use super::*;

pub fn collect_vec_interfaces(input: TokenStream1) -> TokenStream1 {
    let Input { scalar_traits } = parse_macro_input!(input as Input);

    let len_traits = scalar_traits
        .iter()
        .map(|scalar_trait| Ident::new(&format!("VecLen{scalar_trait}"), scalar_trait.span()));

    let alignment_traits = scalar_traits.iter().map(|scalar_trait| {
        Ident::new(&format!("VecAlignment{scalar_trait}"), scalar_trait.span())
    });

    quote! {
        #[allow(private_bounds)]
        pub(super) trait VecLenInterfaces<const N: usize>:
            #(#len_traits<N> + )*
        where
            ScalarCount<N>: VecLen<N>,
        {
        }

        #[allow(private_bounds)]
        pub(super) trait VecAlignmentInterfaces<const N: usize>:
        #(#alignment_traits<N> + )*
        where
            ScalarCount<N>: VecLen<N>,
        {
        }
    }
    .into()
}

struct Input {
    scalar_traits: Vec<Ident>,
}
impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut scalar_traits = Vec::new();
        while !input.is_empty() {
            scalar_traits.push(input.parse()?);
        }

        Ok(Self { scalar_traits })
    }
}
