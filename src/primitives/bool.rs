use super::*;

#[cfg(feature = "primitive_aliases")]
pub mod bool_aliases {
    use super::*;

    vector_aliases!(pub B => bool);
}

#[cfg(feature = "primitive_aliases")]
pub use bool_aliases::*;

impl Scalar for bool {
    type InnerVec2A = [Self; 2];
    type InnerVec3A = [Self; 3];
    type InnerVec4A = [Self; 4];

    const INNER_VEC2A_GARBAGE: Self::InnerVec2A = [false; 2];
    const INNER_VEC3A_GARBAGE: Self::InnerVec3A = [false; 3];
    const INNER_VEC4A_GARBAGE: Self::InnerVec4A = [false; 4];
}
