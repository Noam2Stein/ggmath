use super::*;

mod cget;
mod cwith;
mod default;
mod inner;

impl<T: ElementDefaultImpl> ElementVec for T {}
