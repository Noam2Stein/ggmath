use ggmath::Vector;

use crate::{
    assert_approx_eq, assert_assertion_panic, assert_eq, assert_float_eq, vec2, vec3, vec4,
};

pub fn float_tests() {
    // O(N) = N^3
    // O(24) = 13824
    const VALUES: [(T, T); 24] = [
        (0.0, 0.0),
        (-0.0, -0.0),
        (1.0, 1.0),
        (-1.0, -1.0),
        (0.0, -0.0),
        (1.0, -0.0),
        (-1.0, -0.0),
        (5.3, -0.0),
        (-5.3, 23.5),
        (0.00001, 0.00001),
        (-0.00001, 5.4),
        (-5.3, T::MAX),
        (-5.3, T::MIN),
        (-0.0, T::MIN),
        (T::NAN, 23.5),
        (-T::INFINITY, -T::INFINITY),
        (T::INFINITY, T::NAN),
        (1.0, T::INFINITY),
        (-1.0, T::INFINITY),
        (1.0, T::NAN),
        (T::MAX, T::NAN),
        (0.0, T::INFINITY),
        (0.0, T::NAN),
        (0.0000001, T::MAX),
    ];

    assert_float_eq!(Vec2::ZERO, vec2!(0.0));
    assert_float_eq!(Vec3::ZERO, vec3!(0.0));
    assert_float_eq!(Vec4::ZERO, vec4!(0.0));
    assert_float_eq!(Vec2::ONE, vec2!(1.0));
    assert_float_eq!(Vec3::ONE, vec3!(1.0));
    assert_float_eq!(Vec4::ONE, vec4!(1.0));
    assert_float_eq!(Vec2::X, vec2!(1.0, 0.0));
    assert_float_eq!(Vec3::X, vec3!(1.0, 0.0, 0.0));
    assert_float_eq!(Vec4::X, vec4!(1.0, 0.0, 0.0, 0.0));
    assert_float_eq!(Vec2::NEG_X, vec2!(-1.0, 0.0));
    assert_float_eq!(Vec3::NEG_X, vec3!(-1.0, 0.0, 0.0));
    assert_float_eq!(Vec4::NEG_X, vec4!(-1.0, 0.0, 0.0, 0.0));
    assert_float_eq!(Vec2::Y, vec2!(0.0, 1.0));
    assert_float_eq!(Vec3::Y, vec3!(0.0, 1.0, 0.0));
    assert_float_eq!(Vec4::Y, vec4!(0.0, 1.0, 0.0, 0.0));
    assert_float_eq!(Vec2::NEG_Y, vec2!(0.0, -1.0));
    assert_float_eq!(Vec3::NEG_Y, vec3!(0.0, -1.0, 0.0));
    assert_float_eq!(Vec4::NEG_Y, vec4!(0.0, -1.0, 0.0, 0.0));
    assert_float_eq!(Vec3::Z, vec3!(0.0, 0.0, 1.0));
    assert_float_eq!(Vec4::Z, vec4!(0.0, 0.0, 1.0, 0.0));
    assert_float_eq!(Vec3::NEG_Z, vec3!(0.0, 0.0, -1.0));
    assert_float_eq!(Vec4::NEG_Z, vec4!(0.0, 0.0, -1.0, 0.0));
    assert_float_eq!(Vec4::W, vec4!(0.0, 0.0, 0.0, 1.0));
    assert_float_eq!(Vec4::NEG_W, vec4!(0.0, 0.0, 0.0, -1.0));

    for (x1, y1) in VALUES {
        let ctx = format_args!("x1: {x1:?}, y1: {y1:?}");

        assert_float_eq!(-vec2!(x1, y1), vec2!(-x1, -y1), ctx);
        assert_float_eq!(-vec3!(x1, y1, x1), vec3!(-x1, -y1, -x1), ctx);
        assert_float_eq!(-vec4!(x1, y1, x1, y1), vec4!(-x1, -y1, -x1, -y1), ctx);

        assert_float_eq!(vec2!(x1, y1).recip(), vec2!(x1.recip(), y1.recip()), ctx);
        assert_float_eq!(
            vec3!(x1, y1, x1).recip(),
            vec3!(x1.recip(), y1.recip(), x1.recip()),
            ctx
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).recip(),
            vec4!(x1.recip(), y1.recip(), x1.recip(), y1.recip()),
            ctx
        );

        assert_float_eq!(vec2!(x1, y1).abs(), vec2!(x1.abs(), y1.abs()), ctx);
        assert_float_eq!(
            vec3!(x1, y1, x1).abs(),
            vec3!(x1.abs(), y1.abs(), x1.abs()),
            ctx
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).abs(),
            vec4!(x1.abs(), y1.abs(), x1.abs(), y1.abs()),
            ctx
        );

        assert_float_eq!(vec2!(x1, y1).signum(), vec2!(x1.signum(), y1.signum()), ctx);
        assert_float_eq!(
            vec3!(x1, y1, x1).signum(),
            vec3!(x1.signum(), y1.signum(), x1.signum()),
            ctx
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).signum(),
            vec4!(x1.signum(), y1.signum(), x1.signum(), y1.signum()),
            ctx
        );

        assert_float_eq!(vec2!(x1, y1).element_sum(), x1 + y1, ctx);
        assert_float_eq!(vec3!(x1, y1, x1).element_sum(), x1 + y1 + x1, ctx);
        assert_float_eq!(vec4!(x1, y1, x1, y1).element_sum(), x1 + y1 + x1 + y1, ctx);

        assert_float_eq!(vec2!(x1, y1).element_product(), x1 * y1, ctx);
        assert_float_eq!(vec3!(x1, y1, x1).element_product(), x1 * y1 * x1, ctx);
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).element_product(),
            x1 * y1 * x1 * y1,
            ctx
        );

        if [x1, y1].into_iter().any(T::is_nan) {
            assert_assertion_panic!(vec2!(x1, y1).max_element(), ctx);
            assert_assertion_panic!(vec3!(x1, y1, x1).max_element(), ctx);
            assert_assertion_panic!(vec4!(x1, y1, x1, y1).max_element(), ctx);

            assert_assertion_panic!(vec2!(x1, y1).min_element(), ctx);
            assert_assertion_panic!(vec3!(x1, y1, x1).min_element(), ctx);
            assert_assertion_panic!(vec4!(x1, y1, x1, y1).min_element(), ctx);
        } else {
            assert_float_eq!(vec2!(x1, y1).max_element(), x1.max(y1), 0.0 = -0.0, ctx);
            assert_float_eq!(vec3!(x1, y1, x1).max_element(), x1.max(y1), 0.0 = -0.0, ctx);
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).max_element(),
                x1.max(y1),
                0.0 = -0.0,
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).min_element(), x1.min(y1), 0.0 = -0.0, ctx);
            assert_float_eq!(vec3!(x1, y1, x1).min_element(), x1.min(y1), 0.0 = -0.0, ctx);
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).min_element(),
                x1.min(y1),
                0.0 = -0.0,
                ctx
            );
        }

        assert_float_eq!(vec2!(x1, y1).length_squared(), x1 * x1 + y1 * y1, ctx);
        assert_float_eq!(
            vec3!(x1, y1, x1).length_squared(),
            x1 * x1 + y1 * y1 + x1 * x1,
            ctx
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).length_squared(),
            x1 * x1 + y1 * y1 + x1 * x1 + y1 * y1,
            ctx
        );

        #[cfg(feature = "std")]
        {
            assert_float_eq!(vec2!(x1, y1).floor(), vec2!(x1.floor(), y1.floor()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).floor(),
                vec3!(x1.floor(), y1.floor(), x1.floor()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).floor(),
                vec4!(x1.floor(), y1.floor(), x1.floor(), y1.floor()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).ceil(), vec2!(x1.ceil(), y1.ceil()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).ceil(),
                vec3!(x1.ceil(), y1.ceil(), x1.ceil()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).ceil(),
                vec4!(x1.ceil(), y1.ceil(), x1.ceil(), y1.ceil()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).round(), vec2!(x1.round(), y1.round()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).round(),
                vec3!(x1.round(), y1.round(), x1.round()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).round(),
                vec4!(x1.round(), y1.round(), x1.round(), y1.round()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).trunc(), vec2!(x1.trunc(), y1.trunc()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).trunc(),
                vec3!(x1.trunc(), y1.trunc(), x1.trunc()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).trunc(),
                vec4!(x1.trunc(), y1.trunc(), x1.trunc(), y1.trunc()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).fract(), vec2!(x1.fract(), y1.fract()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).fract(),
                vec3!(x1.fract(), y1.fract(), x1.fract()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).fract(),
                vec4!(x1.fract(), y1.fract(), x1.fract(), y1.fract()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).sqrt(), vec2!(x1.sqrt(), y1.sqrt()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).sqrt(),
                vec3!(x1.sqrt(), y1.sqrt(), x1.sqrt()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).sqrt(),
                vec4!(x1.sqrt(), y1.sqrt(), x1.sqrt(), y1.sqrt()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).sin(), vec2!(x1.sin(), y1.sin()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).sin(),
                vec3!(x1.sin(), y1.sin(), x1.sin()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).sin(),
                vec4!(x1.sin(), y1.sin(), x1.sin(), y1.sin()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).cos(), vec2!(x1.cos(), y1.cos()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).cos(),
                vec3!(x1.cos(), y1.cos(), x1.cos()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).cos(),
                vec4!(x1.cos(), y1.cos(), x1.cos(), y1.cos()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).tan(), vec2!(x1.tan(), y1.tan()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).tan(),
                vec3!(x1.tan(), y1.tan(), x1.tan()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).tan(),
                vec4!(x1.tan(), y1.tan(), x1.tan(), y1.tan()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).asin(), vec2!(x1.asin(), y1.asin()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).asin(),
                vec3!(x1.asin(), y1.asin(), x1.asin()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).asin(),
                vec4!(x1.asin(), y1.asin(), x1.asin(), y1.asin()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).acos(), vec2!(x1.acos(), y1.acos()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).acos(),
                vec3!(x1.acos(), y1.acos(), x1.acos()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).acos(),
                vec4!(x1.acos(), y1.acos(), x1.acos(), y1.acos()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).atan(), vec2!(x1.atan(), y1.atan()), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).atan(),
                vec3!(x1.atan(), y1.atan(), x1.atan()),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).atan(),
                vec4!(x1.atan(), y1.atan(), x1.atan(), y1.atan()),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1).length(), (x1 * x1 + y1 * y1).sqrt(), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1).length(),
                (x1 * x1 + y1 * y1 + x1 * x1).sqrt(),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).length(),
                (x1 * x1 + y1 * y1 + x1 * x1 + y1 * y1).sqrt(),
                ctx
            );

            // `Vector::normalize` panics for the zero vector.
            // if `assert` is disabled, the result is unspecified.
            if x1 == 0.0 && y1 == 0.0 {
                assert_assertion_panic!(vec2!(x1, y1).normalize(), ctx);
                assert_assertion_panic!(vec3!(x1, y1, x1).normalize(), ctx);
                assert_assertion_panic!(vec4!(x1, y1, x1, y1).normalize(), ctx);

                assert_eq!(vec2!(x1, y1).try_normalize(), None, ctx);
                assert_eq!(vec3!(x1, y1, x1).try_normalize(), None, ctx);
                assert_eq!(vec4!(x1, y1, x1, y1).try_normalize(), None, ctx);

                assert_float_eq!(
                    vec2!(x1, y1).normalize_or(vec2!(2401.0)),
                    vec2!(2401.0),
                    ctx
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).normalize_or(vec3!(2401.0)),
                    vec3!(2401.0),
                    ctx
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).normalize_or(vec4!(2401.0)),
                    vec4!(2401.0),
                    ctx
                );
            }
        }

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

            assert_float_eq!(vec2!(x1, y1) + vec2!(x2, y2), vec2!(x1 + x2, y1 + y2), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1) + vec3!(x2, y2, x2),
                vec3!(x1 + x2, y1 + y2, x1 + x2),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) + vec4!(x2, y2, x2, y2),
                vec4!(x1 + x2, y1 + y2, x1 + x2, y1 + y2),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1) - vec2!(x2, y2), vec2!(x1 - x2, y1 - y2), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1) - vec3!(x2, y2, x2),
                vec3!(x1 - x2, y1 - y2, x1 - x2),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) - vec4!(x2, y2, x2, y2),
                vec4!(x1 - x2, y1 - y2, x1 - x2, y1 - y2),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1) * vec2!(x2, y2), vec2!(x1 * x2, y1 * y2), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1) * vec3!(x2, y2, x2),
                vec3!(x1 * x2, y1 * y2, x1 * x2),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) * vec4!(x2, y2, x2, y2),
                vec4!(x1 * x2, y1 * y2, x1 * x2, y1 * y2),
                ctx
            );

            assert_float_eq!(vec2!(x1, y1) / vec2!(x2, y2), vec2!(x1 / x2, y1 / y2), ctx);
            assert_float_eq!(
                vec3!(x1, y1, x1) / vec3!(x2, y2, x2),
                vec3!(x1 / x2, y1 / y2, x1 / x2),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) / vec4!(x2, y2, x2, y2),
                vec4!(x1 / x2, y1 / y2, x1 / x2, y1 / y2),
                ctx
            );

            // `Vector::rem` is allowed to be imprecise for extremely large values.
            if ![x1, y1, x2, y2].into_iter().any(|a| a.abs() > 9_999_999.0) {
                assert_approx_eq!(
                    vec2!(x1, y1) % vec2!(x2, y2),
                    vec2!(x1 % x2, y1 % y2),
                    T::EPSILON * 5.0,
                    ctx
                );
                assert_approx_eq!(
                    vec3!(x1, y1, x1) % vec3!(x2, y2, x2),
                    vec3!(x1 % x2, y1 % y2, x1 % x2),
                    T::EPSILON * 5.0,
                    ctx
                );
                assert_approx_eq!(
                    vec4!(x1, y1, x1, y1) % vec4!(x2, y2, x2, y2),
                    vec4!(x1 % x2, y1 % y2, x1 % x2, y1 % y2),
                    T::EPSILON * 5.0,
                    ctx
                );
            }

            // `Vector::max` and `Vector::min` panic if any element is NaN.
            // if `assert` is disabled, the result is unspecified.
            if [x1, y1, x2, y2].into_iter().any(T::is_nan) {
                assert_assertion_panic!(vec2!(x1, y1).max(vec2!(x2, y2)), ctx);
                assert_assertion_panic!(vec3!(x1, y1, x1).max(vec3!(x2, y2, x2)), ctx);
                assert_assertion_panic!(vec4!(x1, y1, x1, y1).max(vec4!(x2, y2, x2, y2)), ctx);

                assert_assertion_panic!(vec2!(x1, y1).min(vec2!(x2, y2)), ctx);
                assert_assertion_panic!(vec3!(x1, y1, x1).min(vec3!(x2, y2, x2)), ctx);
                assert_assertion_panic!(vec4!(x1, y1, x1, y1).min(vec4!(x2, y2, x2, y2)), ctx);
            } else {
                assert_float_eq!(
                    vec2!(x1, y1).max(vec2!(x2, y2)),
                    vec2!(x1.max(x2), y1.max(y2)),
                    0.0 = -0.0,
                    ctx
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).max(vec3!(x2, y2, x2)),
                    vec3!(x1.max(x2), y1.max(y2), x1.max(x2)),
                    0.0 = -0.0,
                    ctx
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).max(vec4!(x2, y2, x2, y2)),
                    vec4!(x1.max(x2), y1.max(y2), x1.max(x2), y1.max(y2)),
                    0.0 = -0.0,
                    ctx
                );

                assert_float_eq!(
                    vec2!(x1, y1).min(vec2!(x2, y2)),
                    vec2!(x1.min(x2), y1.min(y2)),
                    0.0 = -0.0,
                    ctx
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).min(vec3!(x2, y2, x2)),
                    vec3!(x1.min(x2), y1.min(y2), x1.min(x2)),
                    0.0 = -0.0,
                    ctx
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).min(vec4!(x2, y2, x2, y2)),
                    vec4!(x1.min(x2), y1.min(y2), x1.min(x2), y1.min(y2)),
                    0.0 = -0.0,
                    ctx
                );
            }

            assert_float_eq!(
                vec2!(x1, y1).copysign(vec2!(x2, y2)),
                vec2!(x1.copysign(x2), y1.copysign(y2)),
                ctx
            );
            assert_float_eq!(
                vec3!(x1, y1, x1).copysign(vec3!(x2, y2, x2)),
                vec3!(x1.copysign(x2), y1.copysign(y2), x1.copysign(x2)),
                ctx
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).copysign(vec4!(x2, y2, x2, y2)),
                vec4!(
                    x1.copysign(x2),
                    y1.copysign(y2),
                    x1.copysign(x2),
                    y1.copysign(y2)
                ),
                ctx
            );

            #[cfg(feature = "std")]
            {
                assert_float_eq!(
                    vec2!(x1, y1).div_euclid(vec2!(x2, y2)),
                    vec2!(x1.div_euclid(x2), y1.div_euclid(y2)),
                    ctx
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).div_euclid(vec3!(x2, y2, x2)),
                    vec3!(x1.div_euclid(x2), y1.div_euclid(y2), x1.div_euclid(x2)),
                    ctx
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).div_euclid(vec4!(x2, y2, x2, y2)),
                    vec4!(
                        x1.div_euclid(x2),
                        y1.div_euclid(y2),
                        x1.div_euclid(x2),
                        y1.div_euclid(y2)
                    ),
                    ctx
                );

                assert_float_eq!(
                    vec2!(x1, y1).rem_euclid(vec2!(x2, y2)),
                    vec2!(x1.rem_euclid(x2), y1.rem_euclid(y2)),
                    ctx
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).rem_euclid(vec3!(x2, y2, x2)),
                    vec3!(x1.rem_euclid(x2), y1.rem_euclid(y2), x1.rem_euclid(x2)),
                    ctx
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).rem_euclid(vec4!(x2, y2, x2, y2)),
                    vec4!(
                        x1.rem_euclid(x2),
                        y1.rem_euclid(y2),
                        x1.rem_euclid(x2),
                        y1.rem_euclid(y2)
                    ),
                    ctx
                );
            }

            for (x3, y3) in VALUES {
                let ctx = format_args!(
                    "x1: {x1:?}, y1: {y1:?}, x2: {x2:?}, y2: {y2:?}, x3: {x3:?}, y3: {y3:?}"
                );

                // `Vector::clamp` panics when either theres a NaN or `min` > `max`.
                // if `assert` is disabled, the result is unspecified.
                if [x1, y1, x2, y2, x3, y3].into_iter().any(T::is_nan) || x2 > x3 || y2 > y3 {
                    assert_assertion_panic!(vec2!(x1, y1).clamp(vec2!(x2, y2), vec2!(x3, y3)), ctx);
                    assert_assertion_panic!(
                        vec3!(x1, y1, x1).clamp(vec3!(x2, y2, x2), vec3!(x3, y3, x3)),
                        ctx
                    );
                    assert_assertion_panic!(
                        vec4!(x1, y1, x1, y1).clamp(vec4!(x2, y2, x2, y2), vec4!(x3, y3, x3, y3)),
                        ctx
                    );
                } else {
                    assert_float_eq!(
                        vec2!(x1, y1).clamp(vec2!(x2, y2), vec2!(x3, y3)),
                        vec2!(x1.clamp(x2, x3), y1.clamp(y2, y3)),
                        0.0 = -0.0,
                        ctx
                    );
                    assert_float_eq!(
                        vec3!(x1, y1, x1).clamp(vec3!(x2, y2, x2), vec3!(x3, y3, x3)),
                        vec3!(x1.clamp(x2, x3), y1.clamp(y2, y3), x1.clamp(x2, x3)),
                        0.0 = -0.0,
                        ctx
                    );
                    assert_float_eq!(
                        vec4!(x1, y1, x1, y1).clamp(vec4!(x2, y2, x2, y2), vec4!(x3, y3, x3, y3)),
                        vec4!(
                            x1.clamp(x2, x3),
                            y1.clamp(y2, y3),
                            x1.clamp(x2, x3),
                            y1.clamp(y2, y3)
                        ),
                        0.0 = -0.0,
                        ctx
                    );
                }

                #[cfg(feature = "std")]
                {
                    assert_float_eq!(
                        vec2!(x1, y1).mul_add(vec2!(x2, y2), vec2!(x3, y3)),
                        vec2!(x1.mul_add(x2, x3), y1.mul_add(y2, y3)),
                        ctx
                    );
                    assert_float_eq!(
                        vec3!(x1, y1, x1).mul_add(vec3!(x2, y2, x2), vec3!(x3, y3, x3)),
                        vec3!(x1.mul_add(x2, x3), y1.mul_add(y2, y3), x1.mul_add(x2, x3)),
                        ctx
                    );
                    assert_float_eq!(
                        vec4!(x1, y1, x1, y1).mul_add(vec4!(x2, y2, x2, y2), vec4!(x3, y3, x3, y3)),
                        vec4!(
                            x1.mul_add(x2, x3),
                            y1.mul_add(y2, y3),
                            x1.mul_add(x2, x3),
                            y1.mul_add(y2, y3)
                        ),
                        ctx
                    );
                }
            }
        }
    }
}

type Vec2 = Vector<2, T, A>;
type Vec3 = Vector<3, T, A>;
type Vec4 = Vector<4, T, A>;
