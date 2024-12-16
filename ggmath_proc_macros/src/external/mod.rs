use super::*;

mod derive_wrapper_scalar;
mod matrix_builder;
mod scalar_aliases;
mod scalar_inner_vectors;
mod vector_builder;
pub use derive_wrapper_scalar::derive_wrapper_scalar;
pub use matrix_builder::{
    mat2, mat2c, mat2cp, mat2p, mat2x3, mat2x3c, mat2x3cp, mat2x3p, mat2x4, mat2x4c, mat2x4cp,
    mat2x4p, mat3, mat3c, mat3cp, mat3p, mat3x2, mat3x2c, mat3x2cp, mat3x2p, mat3x4, mat3x4c,
    mat3x4cp, mat3x4p, mat4, mat4c, mat4cp, mat4p, mat4x2, mat4x2c, mat4x2cp, mat4x2p, mat4x3,
    mat4x3c, mat4x3cp, mat4x3p,
};
pub use scalar_aliases::{matrix_aliases, rectangle_aliases, vector_aliases};
pub use scalar_inner_vectors::scalar_inner_vectors;
pub use vector_builder::{vec2, vec2p, vec3, vec3p, vec4, vec4p};
