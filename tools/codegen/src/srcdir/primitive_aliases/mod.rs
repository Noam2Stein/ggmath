use genco::quote;

use crate::{
    iter::Primitive,
    module::{SrcDir, TokensExt},
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
