use std::fmt::{self, Display};

use crate::vec::*;

mod impl_;

pub mod default_impl;
pub mod ops;

pub use gomath_proc_macros::impl_element_inner_vecs;

pub trait Element:
    'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display + ElementVec
{
}
