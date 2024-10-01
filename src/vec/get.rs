use super::*;

pub trait ElementVecGet: ElementInnerVecs {
    fn vec2_x(value: Self::InnerVec2) -> Self;
    fn vec2_y(value: Self::InnerVec2) -> Self;

    fn vec3_x(value: Self::InnerVec3) -> Self;
    fn vec3_y(value: Self::InnerVec3) -> Self;
    fn vec3_z(value: Self::InnerVec3) -> Self;

    fn vec4_x(value: Self::InnerVec4) -> Self;
    fn vec4_y(value: Self::InnerVec4) -> Self;
    fn vec4_z(value: Self::InnerVec4) -> Self;
    fn vec4_w(value: Self::InnerVec4) -> Self;
}

impl<T: Element> Vec2<T> {
    #[inline(always)]
    pub fn x(self) -> T {
        T::vec2_x(self.inner)
    }
    #[inline(always)]
    pub fn y(self) -> T {
        T::vec2_y(self.inner)
    }
}
impl<T: Element> Vec3<T> {
    #[inline(always)]
    pub fn x(self) -> T {
        T::vec3_x(self.inner)
    }
    #[inline(always)]
    pub fn y(self) -> T {
        T::vec3_y(self.inner)
    }
    #[inline(always)]
    pub fn z(self) -> T {
        T::vec3_z(self.inner)
    }
}
impl<T: Element> Vec4<T> {
    #[inline(always)]
    pub fn x(self) -> T {
        T::vec4_x(self.inner)
    }
    #[inline(always)]
    pub fn y(self) -> T {
        T::vec4_y(self.inner)
    }
    #[inline(always)]
    pub fn z(self) -> T {
        T::vec4_z(self.inner)
    }
    #[inline(always)]
    pub fn w(self) -> T {
        T::vec4_w(self.inner)
    }
}
