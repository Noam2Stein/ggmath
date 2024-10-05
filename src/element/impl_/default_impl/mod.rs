use super::*;

mod num;
mod ops;
mod vec;

pub trait ElementDefaultImpl:
    'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display
{
}
impl<T: ElementDefaultImpl> Element for T {}
