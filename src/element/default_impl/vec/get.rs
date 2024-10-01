use super::*;

impl<T: ElementDefaultImpl> ElementVecGet for T {
    #[inline(always)]
    fn vec2_x(value: Self::InnerVec2) -> Self {
        value[0]
    }
    #[inline(always)]
    fn vec2_y(value: Self::InnerVec2) -> Self {
        value[1]
    }

    #[inline(always)]
    fn vec3_x(value: Self::InnerVec3) -> Self {
        value[0]
    }
    #[inline(always)]
    fn vec3_y(value: Self::InnerVec3) -> Self {
        value[1]
    }
    #[inline(always)]
    fn vec3_z(value: Self::InnerVec3) -> Self {
        value[2]
    }

    #[inline(always)]
    fn vec4_x(value: Self::InnerVec4) -> Self {
        value[0]
    }
    #[inline(always)]
    fn vec4_y(value: Self::InnerVec4) -> Self {
        value[1]
    }
    #[inline(always)]
    fn vec4_z(value: Self::InnerVec4) -> Self {
        value[2]
    }
    #[inline(always)]
    fn vec4_w(value: Self::InnerVec4) -> Self {
        value[3]
    }
}
