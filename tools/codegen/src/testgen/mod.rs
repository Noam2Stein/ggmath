use genco::quote;

use crate::backend::{TestDir, TokensExt};

mod vector;

pub fn testmod() -> TestDir {
    quote! {
        mod vector;
    }
    .to_testdir("")
    .with_submod_dir(vector::testmod())
}
