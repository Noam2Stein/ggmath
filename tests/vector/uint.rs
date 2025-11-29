use ggmath::vector::Vector;

use crate::{assert_eq, assert_eq_or_panic, vec2, vec3, vec4};

pub fn uint_tests() {
    const VALUES: [(T, T); 24] = [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (0, 2),
        (3, 0),
        (5, 23),
        (23, 0),
        (0, 23),
        (13, 1),
        (100, 200),
        (T::MAX, 23),
        (23, T::MAX),
        (T::MAX, 1),
        (1, T::MAX),
        (T::MAX, 0),
        (0, T::MAX),
        (T::MAX, T::MAX),
        (200, 100),
        (100, 200),
        (100, 250),
        (250, 100),
        (150, 255),
        (255, 150),
    ];

    assert_eq!(Vec2::ZERO, vec2!(0));
    assert_eq!(Vec3::ZERO, vec3!(0));
    assert_eq!(Vec4::ZERO, vec4!(0));
    assert_eq!(Vec2::ONE, vec2!(1));
    assert_eq!(Vec3::ONE, vec3!(1));
    assert_eq!(Vec4::ONE, vec4!(1));
    assert_eq!(Vec2::X, vec2!(1, 0));
    assert_eq!(Vec3::X, vec3!(1, 0, 0));
    assert_eq!(Vec4::X, vec4!(1, 0, 0, 0));
    assert_eq!(Vec2::Y, vec2!(0, 1));
    assert_eq!(Vec3::Y, vec3!(0, 1, 0));
    assert_eq!(Vec4::Y, vec4!(0, 1, 0, 0));
    assert_eq!(Vec3::Z, vec3!(0, 0, 1));
    assert_eq!(Vec4::Z, vec4!(0, 0, 1, 0));
    assert_eq!(Vec4::W, vec4!(0, 0, 0, 1));

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

            assert_eq_or_panic!(vec2!(x1, y1) + vec2!(x2, y2), vec2!(x1 + x2, y1 + y2), ctx);
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) + vec3!(x2, y2, x2),
                vec3!(x1 + x2, y1 + y2, x1 + x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) + vec4!(x2, y2, x2, y2),
                vec4!(x1 + x2, y1 + y2, x1 + x2, y1 + y2),
                ctx
            );

            assert_eq_or_panic!(vec2!(x1, y1) - vec2!(x2, y2), vec2!(x1 - x2, y1 - y2), ctx);
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) - vec3!(x2, y2, x2),
                vec3!(x1 - x2, y1 - y2, x1 - x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) - vec4!(x2, y2, x2, y2),
                vec4!(x1 - x2, y1 - y2, x1 - x2, y1 - y2),
                ctx
            );

            assert_eq_or_panic!(vec2!(x1, y1) * vec2!(x2, y2), vec2!(x1 * x2, y1 * y2), ctx);
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) * vec3!(x2, y2, x2),
                vec3!(x1 * x2, y1 * y2, x1 * x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) * vec4!(x2, y2, x2, y2),
                vec4!(x1 * x2, y1 * y2, x1 * x2, y1 * y2),
                ctx
            );

            assert_eq_or_panic!(vec2!(x1, y1) / vec2!(x2, y2), vec2!(x1 / x2, y1 / y2), ctx);
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) / vec3!(x2, y2, x2),
                vec3!(x1 / x2, y1 / y2, x1 / x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) / vec4!(x2, y2, x2, y2),
                vec4!(x1 / x2, y1 / y2, x1 / x2, y1 / y2),
                ctx
            );

            assert_eq_or_panic!(vec2!(x1, y1) % vec2!(x2, y2), vec2!(x1 % x2, y1 % y2), ctx);
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) % vec3!(x2, y2, x2),
                vec3!(x1 % x2, y1 % y2, x1 % x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) % vec4!(x2, y2, x2, y2),
                vec4!(x1 % x2, y1 % y2, x1 % x2, y1 % y2),
                ctx
            );

            assert_eq_or_panic!(
                vec2!(x1, y1) << vec2!(x2, y2),
                vec2!(x1 << x2, y1 << y2),
                ctx
            );
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) << vec3!(x2, y2, x2),
                vec3!(x1 << x2, y1 << y2, x1 << x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) << vec4!(x2, y2, x2, y2),
                vec4!(x1 << x2, y1 << y2, x1 << x2, y1 << y2),
                ctx
            );

            assert_eq_or_panic!(
                vec2!(x1, y1) >> vec2!(x2, y2),
                vec2!(x1 >> x2, y1 >> y2),
                ctx
            );
            assert_eq_or_panic!(
                vec3!(x1, y1, x1) >> vec3!(x2, y2, x2),
                vec3!(x1 >> x2, y1 >> y2, x1 >> x2),
                ctx
            );
            assert_eq_or_panic!(
                vec4!(x1, y1, x1, y1) >> vec4!(x2, y2, x2, y2),
                vec4!(x1 >> x2, y1 >> y2, x1 >> x2, y1 >> y2),
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

type Vec2 = Vector<2, T, A>;
type Vec3 = Vector<3, T, A>;
type Vec4 = Vector<4, T, A>;
