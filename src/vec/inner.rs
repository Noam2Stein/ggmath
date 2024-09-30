use super::*;

pub trait InnerVecN<T: Element, const N: usize>:
    fmt::Debug + Copy + PartialEq + PartialOrd
{
    fn default() -> Self;
}
pub trait InnerVec2<T: Element>: InnerVecN<T, 2> + FromVec2Splits<T> {}
pub trait InnerVec3<T: Element>: InnerVecN<T, 3> + FromVec3Splits<T> {}
pub trait InnerVec4<T: Element>: InnerVecN<T, 4> + FromVec4Splits<T> {}

pub type Vec2Inner<T> = <T as Element>::InnerVec2;
pub type Vec3Inner<T> = <T as Element>::InnerVec3;
pub type Vec4Inner<T> = <T as Element>::InnerVec4;
