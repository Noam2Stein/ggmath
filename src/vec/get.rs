use super::*;

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
