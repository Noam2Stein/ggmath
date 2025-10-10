use genco::quote;

use crate::{iter::Primitive, util::TokensExt};

mod float;
mod primitive;

pub fn generate() {
    quote! {
        $(
            for t in Primitive::iter() join($['\r']) =>

            mod $t;
        )
    }
    .write_in_tests("vector/mod.rs");

    for t in Primitive::iter() {
        primitive::generate(t);
    }
}
