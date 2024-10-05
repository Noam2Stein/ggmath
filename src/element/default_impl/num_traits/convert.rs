use cast::AsPrimitive;

use super::*;

impl<T: ElementDefaultImpl + Num, N: NumElement + AsPrimitive<T>> ElementFromNum<N> for T {
    fn from_num(value: N) -> Self {
        N::as_(value)
    }
    fn vec2_from_num(value: Vec2<N>) -> Vec2<Self> {
        Vec2::from_array([Self::from_num(value.x()), Self::from_num(value.y())])
    }
    fn vec3_from_num(value: Vec3<N>) -> Vec3<Self> {
        Vec3::from_array([
            Self::from_num(value.x()),
            Self::from_num(value.y()),
            Self::from_num(value.z()),
        ])
    }
    fn vec4_from_num(value: Vec4<N>) -> Vec4<Self> {
        Vec4::from_array([
            Self::from_num(value.x()),
            Self::from_num(value.y()),
            Self::from_num(value.z()),
            Self::from_num(value.w()),
        ])
    }
}
