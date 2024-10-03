use super::*;

mod const_swizzle;
mod default;
mod inner;
mod splat;
mod swizzle;

impl<T: ElementDefaultImpl> ElementVec for T {}
