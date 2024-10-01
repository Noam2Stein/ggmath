use super::*;

mod default;
mod get;
mod inner;

impl<T: ElementDefaultImpl> ElementVec for T {}
