use super::*;

pub trait ElementVecDefault: ElementInnerVecs {
    fn default_vec2() -> Self::InnerVec2;
    fn default_vec3() -> Self::InnerVec3;
    fn default_vec4() -> Self::InnerVec4;
}

impl<T: Element> Default for Vec2<T> {
    fn default() -> Self {
        Self {
            inner: T::default_vec2(),
        }
    }
}
impl<T: Element> Default for Vec3<T> {
    fn default() -> Self {
        Self {
            inner: T::default_vec3(),
        }
    }
}
impl<T: Element> Default for Vec4<T> {
    fn default() -> Self {
        Self {
            inner: T::default_vec4(),
        }
    }
}
