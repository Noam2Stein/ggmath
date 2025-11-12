use ggmath::Vector;

use crate::{vec2, vec3, vec4};

type Vec2t = Vector<2, T>;
type Vec3t = Vector<3, T>;
type Vec4t = Vector<4, T>;

/// Inorder for these tests to work, these conditions must be met:
/// - `b` must be greater than `a`
/// - `d` must be greater than `c`
pub fn primitive_tests(a: T, b: T, c: T, d: T) {
    assert!(Vec2t::from_array([a, b]).to_array() == [a, b]);
    assert!(Vec3t::from_array([a, b, c]).to_array() == [a, b, c]);
    assert!(Vec4t::from_array([a, b, c, d]).to_array() == [a, b, c, d]);

    assert!(vec2!(a, b).to_array() == [a, b]);
    assert!(vec3!(a, b, c).to_array() == [a, b, c]);
    assert!(vec4!(a, b, c, d).to_array() == [a, b, c, d]);
}
