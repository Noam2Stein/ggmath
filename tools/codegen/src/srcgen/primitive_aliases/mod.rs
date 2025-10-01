use genco::quote;

use crate::{
    backend::{SrcDir, TokensExt},
    iter::Primitive,
};

pub fn srcmod() -> SrcDir {
    quote! {
        $(
            for primitive in Primitive::iter() =>

            $(let prefix = primitive.prefix_uppercase())

            $(format!("/// `{primitive}` type aliases."))
            #[cfg(feature = "primitive_aliases")]
            pub mod $(format!("{primitive}_aliases")) {
                #[cfg(feature = "vector")]
                crate::vector_aliases!(pub type $prefix => $primitive);
            }$['\n']
        )
    }
    .to_srcdir("primitive_aliases")
}
