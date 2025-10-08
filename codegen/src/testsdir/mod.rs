use genco::lang::rust::Tokens;
use strum::IntoEnumIterator;

use crate::iter::Simdness;

mod vector;

pub fn generate() {
    vector::generate();
}

trait TokensExtendExt {
    fn extend_for_simdness(&mut self, f: impl FnMut(Simdness) -> Self);
}

impl TokensExtendExt for Tokens {
    fn extend_for_simdness(&mut self, mut f: impl FnMut(Simdness) -> Self) {
        for s in Simdness::iter() {
            self.extend(f(s));
        }
    }
}
