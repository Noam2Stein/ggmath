use std::ops::{Add, Sub};

use strum_macros::EnumIter;

use crate::iter::Length;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Axis {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum DirectionAxis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn from_usize(usize: usize) -> Option<Self> {
        match usize {
            0 => Some(Axis::X),
            1 => Some(Axis::Y),
            2 => Some(Axis::Z),
            3 => Some(Axis::W),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
            Axis::W => 3,
        }
    }

    pub fn lowercase_str(&self) -> &'static str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
            Axis::W => "w",
        }
    }

    pub fn uppercase_str(&self) -> &'static str {
        match self {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
            Axis::W => "W",
        }
    }

    pub fn ordinal_str(&self) -> &'static str {
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

    pub fn lowercase_str(&self) -> &'static str {
        self.as_axis().lowercase_str()
    }

    pub fn uppercase_str(&self) -> &'static str {
        self.as_axis().uppercase_str()
    }

    pub fn a_lowercase_str(&self) -> &'static str {
        match self {
            DirectionAxis::X => "right",
            DirectionAxis::Y => "up",
            DirectionAxis::Z => "forwards",
        }
    }

    pub fn a_uppercase_str(&self) -> &'static str {
        match self {
            DirectionAxis::X => "RIGHT",
            DirectionAxis::Y => "UP",
            DirectionAxis::Z => "FORWARDS",
        }
    }

    pub fn a_camelcase_str(&self) -> &'static str {
        match self {
            DirectionAxis::X => "Right",
            DirectionAxis::Y => "Up",
            DirectionAxis::Z => "Forwards",
        }
    }

    pub fn b_lowercase_str(&self) -> &'static str {
        match self {
            DirectionAxis::X => "left",
            DirectionAxis::Y => "down",
            DirectionAxis::Z => "backwards",
        }
    }

    pub fn b_uppercase_str(&self) -> &'static str {
        match self {
            DirectionAxis::X => "LEFT",
            DirectionAxis::Y => "DOWN",
            DirectionAxis::Z => "BACKWARDS",
        }
    }

    pub fn b_camelcase_str(&self) -> &'static str {
        match self {
            DirectionAxis::X => "Left",
            DirectionAxis::Y => "Down",
            DirectionAxis::Z => "Backwards",
        }
    }
}

impl From<DirectionAxis> for Axis {
    fn from(value: DirectionAxis) -> Self {
        value.as_axis()
    }
}

impl Add<usize> for Axis {
    type Output = Axis;

    fn add(self, rhs: usize) -> Self::Output {
        Axis::from_usize(self.as_usize() + rhs).unwrap()
    }
}

impl Sub<usize> for Axis {
    type Output = Axis;

    fn sub(self, rhs: usize) -> Self::Output {
        Axis::from_usize(self.as_usize() - rhs).unwrap()
    }
}

impl Add<Length> for Axis {
    type Output = Axis;

    fn add(self, rhs: Length) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl Sub<Length> for Axis {
    type Output = Axis;

    fn sub(self, rhs: Length) -> Self::Output {
        self - rhs.as_usize()
    }
}
