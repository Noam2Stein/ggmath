use super::*;

#[cfg(feature = "primitive_aliases")]
pub mod i16_aliases {
    use super::*;

    vector_aliases!(pub I16 => i16);
}

#[cfg(feature = "primitive_aliases")]
pub use i16_aliases::*;

impl Scalar for i16 {
    type InnerVec2A = [Self; 2];
    type InnerVec3A = [Self; 3];
    type InnerVec4A = [Self; 4];

    const INNER_VEC2A_GARBAGE: Self::InnerVec2A = [0; 2];
    const INNER_VEC3A_GARBAGE: Self::InnerVec3A = [0; 3];
    const INNER_VEC4A_GARBAGE: Self::InnerVec4A = [0; 4];
}
