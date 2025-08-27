use super::*;

#[cfg(feature = "primitive_aliases")]
pub mod f32_aliases {
    use super::*;

    vector_aliases!(pub F => f32);
}

#[cfg(feature = "primitive_aliases")]
pub use f32_aliases::*;

impl Scalar for f32 {
    type InnerVec2A = [Self; 2];
    type InnerVec3A = [Self; 3];
    type InnerVec4A = [Self; 4];

    const INNER_VEC2A_GARBAGE: Self::InnerVec2A = [0.0; 2];
    const INNER_VEC3A_GARBAGE: Self::InnerVec3A = [0.0; 3];
    const INNER_VEC4A_GARBAGE: Self::InnerVec4A = [0.0; 4];
}
