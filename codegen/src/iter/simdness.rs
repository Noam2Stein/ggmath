use std::fmt::Display;

use genco::{lang::Lang, tokens::FormatInto};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Simdness {
    Simd,
    NonSimd,
}

impl Simdness {
    pub fn snakecase(&self) -> &'static str {
        match self {
            Self::Simd => "simd",
            Self::NonSimd => "nonsimd",
        }
    }

    pub fn camelcase(&self) -> &'static str {
        match self {
            Self::Simd => "Simd",
            Self::NonSimd => "NonSimd",
        }
    }

    pub fn lowercase_postfix(&self) -> &'static str {
        match self {
            Self::Simd => "",
            Self::NonSimd => "s",
        }
    }

    pub fn uppercase_postfix(&self) -> &'static str {
        match self {
            Self::Simd => "",
            Self::NonSimd => "S",
        }
    }
}

impl Display for Simdness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.camelcase())
    }
}

impl<L: Lang> FormatInto<L> for Simdness {
    fn format_into(self, tokens: &mut genco::Tokens<L>) {
        tokens.append(self.camelcase());
    }
}
