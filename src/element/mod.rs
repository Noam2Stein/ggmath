use std::fmt::{self, Display};

use crate::vec::*;

mod impl_;
pub use impl_::*;

pub mod num;
pub mod ops;

pub trait Element:
    'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display + ElementVec
{
}
