use genco::quote;

use crate::{
    constants::{PRIMITIVE_PREFIXES, PRIMITIVES},
    module::{ModDir, TokensExt},
};

pub fn mod_() -> ModDir {
    quote! {
        $(
            for (&primitive, &prefix) in PRIMITIVES.iter().zip(PRIMITIVE_PREFIXES.iter()) =>

            $(format!("/// `{primitive}` type aliases."))
            #[cfg(feature = "primitive_aliases")]
            pub mod $(format!("{primitive}_aliases")) {{
                #[cfg(feature = "vector")]
                crate::vector_aliases!(pub $prefix => $primitive);
            }}
        )
    }
    .to_mod_dir("primitive_aliases")
}
