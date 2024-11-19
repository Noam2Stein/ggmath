use super::*;

mod builder;
mod inner_vecs;
mod scalar_aliases;
pub use builder::{
    mat2, mat2c, mat2cp, mat2p, mat2x3, mat2x3c, mat2x3cp, mat2x3p, mat2x4, mat2x4c, mat2x4cp,
    mat2x4p, mat3, mat3c, mat3cp, mat3p, mat3x2, mat3x2c, mat3x2cp, mat3x2p, mat3x4, mat3x4c,
    mat3x4cp, mat3x4p, mat4, mat4c, mat4cp, mat4p, mat4x2, mat4x2c, mat4x2cp, mat4x2p, mat4x3,
    mat4x3c, mat4x3cp, mat4x3p, vec2, vec2p, vec3, vec3p, vec4, vec4p,
};
pub use inner_vecs::inner_vecs;
pub use scalar_aliases::scalar_aliases;