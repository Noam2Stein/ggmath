use super::*;

mod const_swizzle;
mod default;
mod inner;
mod swizzle;

impl<T: ElementDefaultImpl> ElementVec for T {}
