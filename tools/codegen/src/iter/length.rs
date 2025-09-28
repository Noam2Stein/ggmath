use std::fmt::Display;

use genco::{Tokens, lang::Lang, tokens::FormatInto};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Length {
    N2,
    N3,
    N4,
}

impl Length {
    #[expect(unused)]
    pub fn from_usize(value: usize) -> Option<Self> {
        match value {
            2 => Some(Length::N2),
            3 => Some(Length::N3),
            4 => Some(Length::N4),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Length::N2 => 2,
            Length::N3 => 3,
            Length::N4 => 4,
        }
    }

    pub fn word_camelcase(&self) -> &'static str {
        match self {
            Length::N2 => "Two",
            Length::N3 => "Three",
            Length::N4 => "Four",
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_usize())
    }
}

impl<L: Lang> FormatInto<L> for Length {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.as_usize().format_into(tokens);
    }
}
