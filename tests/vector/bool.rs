use ggmath::vector::Vector;

use crate::{assert_eq, vec2, vec3, vec4};

pub fn bool_tests() {
    const VALUES: [(bool, bool); 4] = [(false, false), (false, true), (true, false), (true, true)];

    for (x1, y1) in VALUES {
        let ctx = format_args!("x1: {x1:?}, y1: {y1:?}");

        assert_eq!(!vec2!(x1, y1), vec2!(!x1, !y1), ctx);
        assert_eq!(!vec3!(x1, y1, x1), vec3!(!x1, !y1, !x1), ctx);
        assert_eq!(!vec4!(x1, y1, x1, y1), vec4!(!x1, !y1, !x1, !y1), ctx);

        for (x2, y2) in VALUES {
            let ctx = format_args!("x1: {x1:?}, y1: {y1:?}, x2: {x2:?}, y2: {y2:?}");

            assert_eq!(vec2!(x1, y1) == vec2!(x2, y2), x1 == x2 && y1 == y2, ctx);
            assert_eq!(
                vec3!(x1, y1, x1) == vec3!(x2, y2, x2),
                x1 == x2 && y1 == y2,
                ctx
            );
            assert_eq!(
                vec4!(x1, y1, x1, y1) == vec4!(x2, y2, x2, y2),
                x1 == x2 && y1 == y2,
                ctx
            );

            assert_eq!(vec2!(x1, y1) != vec2!(x2, y2), x1 != x2 || y1 != y2, ctx);
            assert_eq!(
                vec3!(x1, y1, x1) != vec3!(x2, y2, x2),
                x1 != x2 || y1 != y2,
                ctx
            );
            assert_eq!(
                vec4!(x1, y1, x1, y1) != vec4!(x2, y2, x2, y2),
                x1 != x2 || y1 != y2,
                ctx
            );

            assert_eq!(vec2!(x1, y1) & vec2!(x2, y2), vec2!(x1 & x2, y1 & y2), ctx);
            assert_eq!(
                vec3!(x1, y1, x1) & vec3!(x2, y2, x2),
                vec3!(x1 & x2, y1 & y2, x1 & x2),
                ctx
            );
            assert_eq!(
                vec4!(x1, y1, x1, y1) & vec4!(x2, y2, x2, y2),
                vec4!(x1 & x2, y1 & y2, x1 & x2, y1 & y2),
                ctx
            );

            assert_eq!(vec2!(x1, y1) | vec2!(x2, y2), vec2!(x1 | x2, y1 | y2), ctx);
            assert_eq!(
                vec3!(x1, y1, x1) | vec3!(x2, y2, x2),
                vec3!(x1 | x2, y1 | y2, x1 | x2),
                ctx
            );
            assert_eq!(
                vec4!(x1, y1, x1, y1) | vec4!(x2, y2, x2, y2),
                vec4!(x1 | x2, y1 | y2, x1 | x2, y1 | y2),
                ctx
            );

            assert_eq!(vec2!(x1, y1) ^ vec2!(x2, y2), vec2!(x1 ^ x2, y1 ^ y2), ctx);
            assert_eq!(
                vec3!(x1, y1, x1) ^ vec3!(x2, y2, x2),
                vec3!(x1 ^ x2, y1 ^ y2, x1 ^ x2),
                ctx
            );
            assert_eq!(
                vec4!(x1, y1, x1, y1) ^ vec4!(x2, y2, x2, y2),
                vec4!(x1 ^ x2, y1 ^ y2, x1 ^ x2, y1 ^ y2),
                ctx
            );
        }
    }
}
