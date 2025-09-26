use genco::quote;

use crate::{
    module::{SrcDir, TokensExt},
    primitives::Primitive,
};

pub fn src_mod() -> SrcDir {
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
    .to_src_dir("primitive_aliases")
}
