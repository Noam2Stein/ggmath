use ggmath::vector::Vector;

use crate::{assert_eq, assert_eq_or_panic, vec2, vec3, vec4};

pub fn int_tests() {
    const VALUES: [(T, T); 24] = [
        (0, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (1, -1),
        (5, -23),
        (23, 0),
        (0, -23),
        (1, -13),
        (-1, 13),
        (-1, -13),
        (45, 23),
        (T::MAX, 23),
        (23, T::MAX),
        (-T::MAX, 10),
        (10, -T::MAX),
        (T::MIN, 1),
        (1, T::MIN),
        (T::MAX, 0),
        (0, T::MAX),
        (T::MIN, 0),
        (0, T::MIN),
        (-T::MAX, -45),
        (-45, T::MAX),
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
    assert_eq!(Vec2::NEG_X, vec2!(-1, 0));
    assert_eq!(Vec3::NEG_X, vec3!(-1, 0, 0));
    assert_eq!(Vec4::NEG_X, vec4!(-1, 0, 0, 0));
    assert_eq!(Vec2::Y, vec2!(0, 1));
    assert_eq!(Vec3::Y, vec3!(0, 1, 0));
    assert_eq!(Vec4::Y, vec4!(0, 1, 0, 0));
    assert_eq!(Vec2::NEG_Y, vec2!(0, -1));
    assert_eq!(Vec3::NEG_Y, vec3!(0, -1, 0));
    assert_eq!(Vec4::NEG_Y, vec4!(0, -1, 0, 0));
    assert_eq!(Vec3::Z, vec3!(0, 0, 1));
    assert_eq!(Vec4::Z, vec4!(0, 0, 1, 0));
    assert_eq!(Vec3::NEG_Z, vec3!(0, 0, -1));
    assert_eq!(Vec4::NEG_Z, vec4!(0, 0, -1, 0));
    assert_eq!(Vec4::W, vec4!(0, 0, 0, 1));
    assert_eq!(Vec4::NEG_W, vec4!(0, 0, 0, -1));

    for (x1, y1) in VALUES {
        let ctx = format_args!("x1: {x1:?}, y1: {y1:?}");

        assert_eq_or_panic!(-vec2!(x1, y1), vec2!(-x1, -y1), ctx);
        assert_eq_or_panic!(-vec3!(x1, y1, x1), vec3!(-x1, -y1, -x1), ctx);
        assert_eq_or_panic!(-vec4!(x1, y1, x1, y1), vec4!(-x1, -y1, -x1, -y1), ctx);

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
