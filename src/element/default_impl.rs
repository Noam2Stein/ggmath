use crate::vec::*;

use super::*;

pub use ggmath_proc_macros::impl_element_default;

pub unsafe trait ElementDefaultImpl:
    fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display
{
}

unsafe impl<T: ElementDefaultImpl> Element for T
where
    [Self; 2]: InnerVec2<T>,
    [Self; 4]: InnerVec3<T>,
    [Self; 4]: InnerVec4<T>,
{
    type InnerVec2 = [Self; 2];
    type InnerVec3 = [Self; 4];
    type InnerVec4 = [Self; 4];
}
impl<T: ElementDefaultImpl> InnerVec2<T> for [T; 2]
where
    [T; 2]: FromVec2Splits<T>,
    [T; 4]: FromVec3Splits<T> + FromVec4Splits<T>,
{
}
impl<T: ElementDefaultImpl> InnerVec3<T> for [T; 4]
where
    [T; 2]: FromVec2Splits<T>,
    [T; 4]: FromVec3Splits<T> + FromVec4Splits<T>,
{
}
impl<T: ElementDefaultImpl> InnerVec4<T> for [T; 4]
where
    [T; 2]: FromVec2Splits<T>,
    [T; 4]: FromVec3Splits<T> + FromVec4Splits<T>,
{
}
impl<T: ElementDefaultImpl> InnerVecN<T, 2> for [T; 2]
where
    [T; 2]: FromVec2Splits<T>,
    [T; 4]: FromVec3Splits<T> + FromVec4Splits<T>,
{
    fn default() -> Self {
        [T::default(); 2]
    }
}
impl<T: ElementDefaultImpl> InnerVecN<T, 3> for [T; 4]
where
    [T; 2]: FromVec2Splits<T>,
    [T; 4]: FromVec3Splits<T> + FromVec4Splits<T>,
{
    fn default() -> Self {
        [T::default(); 4]
    }
}
impl<T: ElementDefaultImpl> InnerVecN<T, 4> for [T; 4]
where
    [T; 2]: FromVec2Splits<T>,
    [T; 4]: FromVec3Splits<T> + FromVec4Splits<T>,
{
    fn default() -> Self {
        [T::default(); 4]
    }
}
