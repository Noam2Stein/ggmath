use std::fmt::Display;

use genco::{Tokens, lang::Lang, tokens::FormatInto};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Length {
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionAxis {
    X,
    Y,
    Z,
}

impl Length {
    pub fn as_usize(&self) -> usize {
        match self {
            Length::Two => 2,
            Length::Three => 3,
            Length::Four => 4,
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
}

impl Axis {
    pub fn as_usize(&self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
            Axis::W => 3,
        }
    }

    pub fn as_str_lowercase(&self) -> &'static str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
            Axis::W => "w",
        }
    }

    pub fn as_str_uppercase(&self) -> &'static str {
        match self {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
            Axis::W => "W",
        }
    }

    pub fn as_str_ordinal(&self) -> &'static str {
        match self {
            Axis::X => "1st",
            Axis::Y => "2nd",
            Axis::Z => "3rd",
            Axis::W => "4th",
        }
    }

    pub fn as_dir_axis(&self) -> Option<DirectionAxis> {
        match self {
            Axis::X => Some(DirectionAxis::X),
            Axis::Y => Some(DirectionAxis::Y),
            Axis::Z => Some(DirectionAxis::Z),
            Axis::W => None,
        }
    }
}

impl DirectionAxis {
    pub fn as_axis(&self) -> Axis {
        match self {
            DirectionAxis::X => Axis::X,
            DirectionAxis::Y => Axis::Y,
            DirectionAxis::Z => Axis::Z,
        }
    }

    pub fn as_str_lowercase(&self) -> &'static str {
        self.as_axis().as_str_lowercase()
    }

    pub fn as_str_uppercase(&self) -> &'static str {
        self.as_axis().as_str_uppercase()
    }

    pub fn a_as_str_lowercase(&self) -> &'static str {
        match self {
            DirectionAxis::X => "right",
            DirectionAxis::Y => "up",
            DirectionAxis::Z => "forwards",
        }
    }

    pub fn a_as_str_uppercase(&self) -> &'static str {
        match self {
            DirectionAxis::X => "RIGHT",
            DirectionAxis::Y => "UP",
            DirectionAxis::Z => "FORWARDS",
        }
    }

    pub fn a_as_str_camelcase(&self) -> &'static str {
        match self {
            DirectionAxis::X => "Right",
            DirectionAxis::Y => "Up",
            DirectionAxis::Z => "Forwards",
        }
    }

    pub fn b_as_str_lowercase(&self) -> &'static str {
        match self {
            DirectionAxis::X => "left",
            DirectionAxis::Y => "down",
            DirectionAxis::Z => "backwards",
        }
    }

    pub fn b_as_str_uppercase(&self) -> &'static str {
        match self {
            DirectionAxis::X => "LEFT",
            DirectionAxis::Y => "DOWN",
            DirectionAxis::Z => "BACKWARDS",
        }
    }

    pub fn b_as_str_camelcase(&self) -> &'static str {
        match self {
            DirectionAxis::X => "Left",
            DirectionAxis::Y => "Down",
            DirectionAxis::Z => "Backwards",
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_usize())
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str_lowercase())
    }
}

impl Display for DirectionAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str_lowercase())
    }
}

impl<L: Lang> FormatInto<L> for Length {
    fn format_into(self, tokens: &mut Tokens<L>) {
        self.as_usize().format_into(tokens);
    }
}
