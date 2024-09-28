use crate::vec::*;

use super::*;

pub trait ElementDefaultImpl:
    fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display
{
}

unsafe impl<T: ElementDefaultImpl> Element for T
where
    [Self; 2]: FromVec2Splits<T = T>,
    [Self; 4]: FromVec3Splits<T = T>,
    [Self; 4]: FromVec4Splits<T = T>,
{
    type InnerVec2 = [Self; 2];
    type InnerVec3 = [Self; 4];
    type InnerVec4 = [Self; 4];
}
impl<T: ElementDefaultImpl> InnerVec2 for [T; 2] where [T; 2]: FromVec2Splits<T = T> {}
impl<T: ElementDefaultImpl> InnerVec3 for [T; 4] where [T; 4]: FromVec3Splits<T = T> {}
impl<T: ElementDefaultImpl> InnerVec4 for [T; 4] where [T; 4]: FromVec4Splits<T = T> {}
impl<T: ElementDefaultImpl> InnerVecN<2> for [T; 2]
where
    [T; 2]: FromVec2Splits<T = T>,
{
    fn default() -> Self {
        [T::default(); 2]
    }
}
impl<T: ElementDefaultImpl> InnerVecN<3> for [T; 4]
where
    [T; 4]: FromVec3Splits<T = T>,
{
    fn default() -> Self {
        [T::default(); 4]
    }
}
impl<T: ElementDefaultImpl> InnerVecN<4> for [T; 4]
where
    [T; 4]: FromVec4Splits<T = T>,
{
    fn default() -> Self {
        [T::default(); 4]
    }
}
impl<T: Element, const N: usize> ElementContainer for [T; N] {
    type T = T;
}
