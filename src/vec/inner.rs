use super::*;

pub trait InnerVecN<const N: usize>: ElementContainer {
    fn default() -> Self;
}
pub trait InnerVec2: InnerVecN<2> + FromVec2Splits {}
pub trait InnerVec3: InnerVecN<3> + FromVec3Splits {}
pub trait InnerVec4: InnerVecN<4> + FromVec4Splits {}

pub type Vec2Inner<T: Element> = T::InnerVec2;
pub type Vec3Inner<T: Element> = T::InnerVec3;
pub type Vec4Inner<T: Element> = T::InnerVec4;
