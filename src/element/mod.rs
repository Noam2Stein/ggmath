use std::fmt::{self, Display};

use crate::vec::*;

mod primitive_impl;

pub mod default_impl;
pub mod ops;

pub trait Element:
    'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display + ElementVec
{
}
