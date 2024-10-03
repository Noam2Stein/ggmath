use super::*;

#[inline(always)]
pub fn splat2<T: Element>(value: T) -> Vec2<T> {
    Vec2::splat(value)
}
#[inline(always)]
pub fn splat3<T: Element>(value: T) -> Vec3<T> {
    Vec3::splat(value)
}
#[inline(always)]
pub fn splat4<T: Element>(value: T) -> Vec4<T> {
    Vec4::splat(value)
}

pub trait VecNSplat<T: Element> {
    fn splat(value: T) -> Self;
}

pub trait ElementVecSplat: ElementVecInner {
    fn vec2_splat(value: Self) -> Self::InnerVec2;
    fn vec3_splat(value: Self) -> Self::InnerVec3;
    fn vec4_splat(value: Self) -> Self::InnerVec4;
}

impl<T: Element> VecNSplat<T> for Vec2<T> {
    #[inline(always)]
    fn splat(value: T) -> Self {
        Self::from_inner(T::vec2_splat(value))
    }
}
impl<T: Element> VecNSplat<T> for Vec3<T> {
    #[inline(always)]
    fn splat(value: T) -> Self {
        Self::from_inner(T::vec3_splat(value))
    }
}
impl<T: Element> VecNSplat<T> for Vec4<T> {
    #[inline(always)]
    fn splat(value: T) -> Self {
        Self::from_inner(T::vec4_splat(value))
    }
}
