use crate::{vec2, vec3, vec4};

pub fn float_tests() {
    assert_eq!(vec2!(1.0, 2.0) + vec2!(3.0, 4.0), vec2!(4.0, 6.0));
    assert_eq!(
        vec3!(1.0, 2.0, 3.0) + vec3!(4.0, 5.0, 6.0),
        vec3!(5.0, 7.0, 9.0)
    );
    assert_eq!(
        vec4!(1.0, 2.0, 3.0, 4.0) + vec4!(5.0, 6.0, 7.0, 8.0),
        vec4!(6.0, 8.0, 10.0, 12.0)
    );
}
