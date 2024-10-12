pub trait Construct: InnerConstruct + std::fmt::Display {}
impl<T: InnerConstruct + std::fmt::Display> Construct for T {}

pub trait InnerConstruct: Sized + Send + Sync + Copy + PartialEq + std::fmt::Debug {}
impl<T: Sized + Send + Sync + Copy + PartialEq + std::fmt::Debug> InnerConstruct for T {}
