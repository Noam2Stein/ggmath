use std::fmt::Display;

use genco::{Tokens, lang::Lang, tokens::FormatInto};
use strum_macros::EnumIter;

use crate::iter::{Axis, DirectionAxis};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Length {
    Two,
    Three,
    Four,
}

impl Length {
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

    pub fn word_camelcase(&self) -> &'static str {
        match self {
            Length::Two => "Two",
            Length::Three => "Three",
            Length::Four => "Four",
        }
    }

    pub fn axes(self) -> impl Iterator<Item = Axis> {
        match self {
            Length::Two => [Axis::X, Axis::Y].iter().copied(),
            Length::Three => [Axis::X, Axis::Y, Axis::Z].iter().copied(),
            Length::Four => [Axis::X, Axis::Y, Axis::Z, Axis::W].iter().copied(),
        }
    }

    pub fn dir_axes(self) -> impl Iterator<Item = DirectionAxis> {
        self.axes().filter_map(|axis| axis.as_dir_axis())
    }

    pub fn has_axis(&self, axis: impl Into<Axis>) -> bool {
        axis.into().as_usize() < self.as_usize()
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
