use genco::quote;

use crate::{
    constants::{PRIMITIVE_PREFIXES, PRIMITIVES},
    module::{SrcDir, TokensExt},
};

pub fn src_mod() -> SrcDir {
    quote! {
        $(
            for (&primitive, &prefix) in PRIMITIVES.iter().zip(PRIMITIVE_PREFIXES.iter()) =>

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
