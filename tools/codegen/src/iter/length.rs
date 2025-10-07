use std::fmt::Display;

use genco::{Tokens, lang::Lang, tokens::FormatInto};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Length {
    Two,
    Three,
    Four,
}

impl Length {
    #[expect(unused)]
    pub fn from_usize(value: usize) -> Option<Self> {
        match value {
            2 => Some(Length::Two),
            3 => Some(Length::Three),
            4 => Some(Length::Four),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Length::Two => 2,
            Length::Three => 3,
            Length::Four => 4,
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
        FormatInto::format_into(self.as_usize(), tokens);
    }
}
