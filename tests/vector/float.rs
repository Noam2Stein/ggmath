use ggmath::{SupportedVecLen, VecLen, Vector};

use crate::{ggmath_assert_panic, vec2, vec3, vec4};

pub fn float_tests() {
    // be careful when adding new values.
    // too many values will make the test take too long.
    //
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

    assert_float_eq!(Vec2t::ZERO, vec2!(0.0));
    assert_float_eq!(Vec3t::ZERO, vec3!(0.0));
    assert_float_eq!(Vec4t::ZERO, vec4!(0.0));
    assert_float_eq!(Vec2t::ONE, vec2!(1.0));
    assert_float_eq!(Vec3t::ONE, vec3!(1.0));
    assert_float_eq!(Vec4t::ONE, vec4!(1.0));
    assert_float_eq!(Vec2t::X, vec2!(1.0, 0.0));
    assert_float_eq!(Vec3t::X, vec3!(1.0, 0.0, 0.0));
    assert_float_eq!(Vec4t::X, vec4!(1.0, 0.0, 0.0, 0.0));
    assert_float_eq!(Vec2t::NEG_X, vec2!(-1.0, 0.0));
    assert_float_eq!(Vec3t::NEG_X, vec3!(-1.0, 0.0, 0.0));
    assert_float_eq!(Vec4t::NEG_X, vec4!(-1.0, 0.0, 0.0, 0.0));
    assert_float_eq!(Vec2t::Y, vec2!(0.0, 1.0));
    assert_float_eq!(Vec3t::Y, vec3!(0.0, 1.0, 0.0));
    assert_float_eq!(Vec4t::Y, vec4!(0.0, 1.0, 0.0, 0.0));
    assert_float_eq!(Vec2t::NEG_Y, vec2!(0.0, -1.0));
    assert_float_eq!(Vec3t::NEG_Y, vec3!(0.0, -1.0, 0.0));
    assert_float_eq!(Vec4t::NEG_Y, vec4!(0.0, -1.0, 0.0, 0.0));
    assert_float_eq!(Vec3t::Z, vec3!(0.0, 0.0, 1.0));
    assert_float_eq!(Vec4t::Z, vec4!(0.0, 0.0, 1.0, 0.0));
    assert_float_eq!(Vec3t::NEG_Z, vec3!(0.0, 0.0, -1.0));
    assert_float_eq!(Vec4t::NEG_Z, vec4!(0.0, 0.0, -1.0, 0.0));
    assert_float_eq!(Vec4t::W, vec4!(0.0, 0.0, 0.0, 1.0));
    assert_float_eq!(Vec4t::NEG_W, vec4!(0.0, 0.0, 0.0, -1.0));

    assert_float_eq!(
        vec2!(1.0, 0.0).dot(vec2!(1.0, 0.0)),
        1.0 * 1.0 * T::to_radians(0.0).cos()
    );
    assert_float_eq!(
        vec3!(1.0, 0.0, 0.0).dot(vec3!(1.0, 0.0, 0.0)),
        1.0 * 1.0 * T::to_radians(0.0).cos()
    );
    assert_float_eq!(
        vec4!(1.0, 0.0, 0.0, 0.0).dot(vec4!(1.0, 0.0, 0.0, 0.0)),
        1.0 * 1.0 * T::to_radians(0.0).cos()
    );
    assert_approx_eq!(
        vec2!(1.0, 1.0).dot(vec2!(1.0, 0.0)),
        T::sqrt(2.0) * 1.0 * T::to_radians(45.0).cos(),
        0.00001
    );
    assert_approx_eq!(
        vec3!(1.0, 1.0, 0.0).dot(vec3!(1.0, 0.0, 0.0)),
        T::sqrt(2.0) * 1.0 * T::to_radians(45.0).cos(),
        0.00001
    );
    assert_approx_eq!(
        vec4!(1.0, 1.0, 0.0, 0.0).dot(vec4!(1.0, 0.0, 0.0, 0.0)),
        T::sqrt(2.0) * 1.0 * T::to_radians(45.0).cos(),
        0.00001
    );

    assert_float_eq!(vec2!(1.0, 0.0).normalize(), vec2!(1.0, 0.0));
    assert_float_eq!(vec3!(1.0, 0.0, 0.0).normalize(), vec3!(1.0, 0.0, 0.0));
    assert_float_eq!(
        vec4!(1.0, 0.0, 0.0, 0.0).normalize(),
        vec4!(1.0, 0.0, 0.0, 0.0)
    );
    assert_approx_eq!(
        vec2!(1.0, 1.0).normalize(),
        vec2!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0)),
        0.0001
    );
    assert_approx_eq!(
        vec3!(1.0, 1.0, 0.0).normalize(),
        vec3!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0), 0.0),
        0.0001
    );
    assert_approx_eq!(
        vec4!(1.0, 1.0, 0.0, 0.0).normalize(),
        vec4!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0), 0.0, 0.0),
        0.0001
    );

    assert_float_eq!(vec2!(1.0, 0.0).try_normalize(), Some(vec2!(1.0, 0.0)));
    assert_float_eq!(
        vec3!(1.0, 0.0, 0.0).try_normalize(),
        Some(vec3!(1.0, 0.0, 0.0))
    );
    assert_float_eq!(
        vec4!(1.0, 0.0, 0.0, 0.0).try_normalize(),
        Some(vec4!(1.0, 0.0, 0.0, 0.0))
    );
    assert_approx_eq!(
        vec2!(1.0, 1.0).try_normalize(),
        Some(vec2!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0))),
        0.0001
    );
    assert_approx_eq!(
        vec3!(1.0, 1.0, 0.0).try_normalize(),
        Some(vec3!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0), 0.0)),
        0.0001
    );
    assert_approx_eq!(
        vec4!(1.0, 1.0, 0.0, 0.0).try_normalize(),
        Some(vec4!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0), 0.0, 0.0)),
        0.0001
    );

    assert_float_eq!(vec2!(1.0, 0.0).normalize_or(vec2!(T::NAN)), vec2!(1.0, 0.0));
    assert_float_eq!(
        vec3!(1.0, 0.0, 0.0).normalize_or(vec3!(T::NAN)),
        vec3!(1.0, 0.0, 0.0)
    );
    assert_float_eq!(
        vec4!(1.0, 0.0, 0.0, 0.0).normalize_or(vec4!(T::NAN)),
        vec4!(1.0, 0.0, 0.0, 0.0)
    );
    assert_approx_eq!(
        vec2!(1.0, 1.0).normalize_or(vec2!(T::NAN)),
        vec2!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0)),
        0.0001
    );
    assert_approx_eq!(
        vec3!(1.0, 1.0, 0.0).normalize_or(vec3!(T::NAN)),
        vec3!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0), 0.0),
        0.0001
    );
    assert_approx_eq!(
        vec4!(1.0, 1.0, 0.0, 0.0).normalize_or(vec4!(T::NAN)),
        vec4!(1.0 / T::sqrt(2.0), 1.0 / T::sqrt(2.0), 0.0, 0.0),
        0.0001
    );

    for (x1, y1) in VALUES {
        assert_float_eq!(-vec2!(x1, y1), vec2!(-x1, -y1));
        assert_float_eq!(-vec3!(x1, y1, x1), vec3!(-x1, -y1, -x1));
        assert_float_eq!(-vec4!(x1, y1, x1, y1), vec4!(-x1, -y1, -x1, -y1));

        assert_float_eq!(vec2!(x1, y1).floor(), vec2!(x1.floor(), y1.floor()));
        assert_float_eq!(
            vec3!(x1, y1, x1).floor(),
            vec3!(x1.floor(), y1.floor(), x1.floor())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).floor(),
            vec4!(x1.floor(), y1.floor(), x1.floor(), y1.floor())
        );

        assert_float_eq!(vec2!(x1, y1).ceil(), vec2!(x1.ceil(), y1.ceil()));
        assert_float_eq!(
            vec3!(x1, y1, x1).ceil(),
            vec3!(x1.ceil(), y1.ceil(), x1.ceil())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).ceil(),
            vec4!(x1.ceil(), y1.ceil(), x1.ceil(), y1.ceil())
        );

        assert_float_eq!(vec2!(x1, y1).round(), vec2!(x1.round(), y1.round()));
        assert_float_eq!(
            vec3!(x1, y1, x1).round(),
            vec3!(x1.round(), y1.round(), x1.round())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).round(),
            vec4!(x1.round(), y1.round(), x1.round(), y1.round())
        );

        assert_float_eq!(vec2!(x1, y1).trunc(), vec2!(x1.trunc(), y1.trunc()));
        assert_float_eq!(
            vec3!(x1, y1, x1).trunc(),
            vec3!(x1.trunc(), y1.trunc(), x1.trunc())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).trunc(),
            vec4!(x1.trunc(), y1.trunc(), x1.trunc(), y1.trunc())
        );

        assert_float_eq!(vec2!(x1, y1).fract(), vec2!(x1.fract(), y1.fract()));
        assert_float_eq!(
            vec3!(x1, y1, x1).fract(),
            vec3!(x1.fract(), y1.fract(), x1.fract())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).fract(),
            vec4!(x1.fract(), y1.fract(), x1.fract(), y1.fract())
        );

        assert_float_eq!(vec2!(x1, y1).sqrt(), vec2!(x1.sqrt(), y1.sqrt()));
        assert_float_eq!(
            vec3!(x1, y1, x1).sqrt(),
            vec3!(x1.sqrt(), y1.sqrt(), x1.sqrt())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).sqrt(),
            vec4!(x1.sqrt(), y1.sqrt(), x1.sqrt(), y1.sqrt())
        );

        assert_float_eq!(vec2!(x1, y1).sin(), vec2!(x1.sin(), y1.sin()));
        assert_float_eq!(vec3!(x1, y1, x1).sin(), vec3!(x1.sin(), y1.sin(), x1.sin()));
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).sin(),
            vec4!(x1.sin(), y1.sin(), x1.sin(), y1.sin())
        );

        assert_float_eq!(vec2!(x1, y1).cos(), vec2!(x1.cos(), y1.cos()));
        assert_float_eq!(vec3!(x1, y1, x1).cos(), vec3!(x1.cos(), y1.cos(), x1.cos()));
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).cos(),
            vec4!(x1.cos(), y1.cos(), x1.cos(), y1.cos())
        );

        assert_float_eq!(vec2!(x1, y1).tan(), vec2!(x1.tan(), y1.tan()));
        assert_float_eq!(vec3!(x1, y1, x1).tan(), vec3!(x1.tan(), y1.tan(), x1.tan()));
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).tan(),
            vec4!(x1.tan(), y1.tan(), x1.tan(), y1.tan())
        );

        assert_float_eq!(vec2!(x1, y1).asin(), vec2!(x1.asin(), y1.asin()));
        assert_float_eq!(
            vec3!(x1, y1, x1).asin(),
            vec3!(x1.asin(), y1.asin(), x1.asin())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).asin(),
            vec4!(x1.asin(), y1.asin(), x1.asin(), y1.asin())
        );

        assert_float_eq!(vec2!(x1, y1).acos(), vec2!(x1.acos(), y1.acos()));
        assert_float_eq!(
            vec3!(x1, y1, x1).acos(),
            vec3!(x1.acos(), y1.acos(), x1.acos())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).acos(),
            vec4!(x1.acos(), y1.acos(), x1.acos(), y1.acos())
        );

        assert_float_eq!(vec2!(x1, y1).atan(), vec2!(x1.atan(), y1.atan()));
        assert_float_eq!(
            vec3!(x1, y1, x1).atan(),
            vec3!(x1.atan(), y1.atan(), x1.atan())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).atan(),
            vec4!(x1.atan(), y1.atan(), x1.atan(), y1.atan())
        );

        assert_float_eq!(vec2!(x1, y1).recip(), vec2!(x1.recip(), y1.recip()));
        assert_float_eq!(
            vec3!(x1, y1, x1).recip(),
            vec3!(x1.recip(), y1.recip(), x1.recip())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).recip(),
            vec4!(x1.recip(), y1.recip(), x1.recip(), y1.recip())
        );

        assert_float_eq!(vec2!(x1, y1).abs(), vec2!(x1.abs(), y1.abs()));
        assert_float_eq!(vec3!(x1, y1, x1).abs(), vec3!(x1.abs(), y1.abs(), x1.abs()));
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).abs(),
            vec4!(x1.abs(), y1.abs(), x1.abs(), y1.abs())
        );

        assert_float_eq!(vec2!(x1, y1).signum(), vec2!(x1.signum(), y1.signum()));
        assert_float_eq!(
            vec3!(x1, y1, x1).signum(),
            vec3!(x1.signum(), y1.signum(), x1.signum())
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).signum(),
            vec4!(x1.signum(), y1.signum(), x1.signum(), y1.signum())
        );

        assert_float_eq!(vec2!(x1, y1).element_sum(), x1 + y1);
        assert_float_eq!(vec3!(x1, y1, x1).element_sum(), x1 + y1 + x1);
        assert_float_eq!(vec4!(x1, y1, x1, y1).element_sum(), x1 + y1 + x1 + y1);

        assert_float_eq!(vec2!(x1, y1).element_product(), x1 * y1);
        assert_float_eq!(vec3!(x1, y1, x1).element_product(), x1 * y1 * x1);
        assert_float_eq!(vec4!(x1, y1, x1, y1).element_product(), x1 * y1 * x1 * y1);

        if [x1, y1].into_iter().any(T::is_nan) {
            ggmath_assert_panic!(vec2!(x1, y1).max_element());
            ggmath_assert_panic!(vec3!(x1, y1, x1).max_element());
            ggmath_assert_panic!(vec4!(x1, y1, x1, y1).max_element());

            ggmath_assert_panic!(vec2!(x1, y1).min_element());
            ggmath_assert_panic!(vec3!(x1, y1, x1).min_element());
            ggmath_assert_panic!(vec4!(x1, y1, x1, y1).min_element());
        } else {
            assert_float_eq!(vec2!(x1, y1).max_element(), x1.max(y1));
            assert_float_eq!(vec3!(x1, y1, x1).max_element(), x1.max(y1));
            assert_float_eq!(vec4!(x1, y1, x1, y1).max_element(), x1.max(y1));

            assert_float_eq!(vec2!(x1, y1).min_element(), x1.min(y1));
            assert_float_eq!(vec3!(x1, y1, x1).min_element(), x1.min(y1));
            assert_float_eq!(vec4!(x1, y1, x1, y1).min_element(), x1.min(y1));
        }

        assert_float_eq!(vec2!(x1, y1).length(), (x1 * x1 + y1 * y1).sqrt());
        assert_float_eq!(
            vec3!(x1, y1, x1).length(),
            (x1 * x1 + y1 * y1 + x1 * x1).sqrt()
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).length(),
            (x1 * x1 + y1 * y1 + x1 * x1 + y1 * y1).sqrt()
        );

        assert_float_eq!(vec2!(x1, y1).length_squared(), x1 * x1 + y1 * y1);
        assert_float_eq!(
            vec3!(x1, y1, x1).length_squared(),
            x1 * x1 + y1 * y1 + x1 * x1
        );
        assert_float_eq!(
            vec4!(x1, y1, x1, y1).length_squared(),
            x1 * x1 + y1 * y1 + x1 * x1 + y1 * y1
        );

        // `Vector::normalize` panics for the zero vector.
        // if `assert` is disabled, the result is unspecified.
        if x1 == 0.0 && y1 == 0.0 {
            ggmath_assert_panic!(vec2!(x1, y1).normalize());
            ggmath_assert_panic!(vec3!(x1, y1, x1).normalize());
            ggmath_assert_panic!(vec4!(x1, y1, x1, y1).normalize());

            assert_eq!(vec2!(x1, y1).try_normalize(), None);
            assert_eq!(vec3!(x1, y1, x1).try_normalize(), None);
            assert_eq!(vec4!(x1, y1, x1, y1).try_normalize(), None);

            assert_float_eq!(vec2!(x1, y1).normalize_or(vec2!(2401.0)), vec2!(2401.0));
            assert_float_eq!(vec3!(x1, y1, x1).normalize_or(vec3!(2401.0)), vec3!(2401.0));
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).normalize_or(vec4!(2401.0)),
                vec4!(2401.0)
            );
        }

        for (x2, y2) in VALUES {
            assert_eq!(vec2!(x1, y1) == vec2!(x2, y2), x1 == x2 && y1 == y2);
            assert_eq!(vec3!(x1, y1, x1) == vec3!(x2, y2, x2), x1 == x2 && y1 == y2);
            assert_eq!(
                vec4!(x1, y1, x1, y1) == vec4!(x2, y2, x2, y2),
                x1 == x2 && y1 == y2
            );

            assert_eq!(vec2!(x1, y1) != vec2!(x2, y2), x1 != x2 || y1 != y2);
            assert_eq!(vec3!(x1, y1, x1) != vec3!(x2, y2, x2), x1 != x2 || y1 != y2);
            assert_eq!(
                vec4!(x1, y1, x1, y1) != vec4!(x2, y2, x2, y2),
                x1 != x2 || y1 != y2
            );

            assert_float_eq!(vec2!(x1, y1) + vec2!(x2, y2), vec2!(x1 + x2, y1 + y2));
            assert_float_eq!(
                vec3!(x1, y1, x1) + vec3!(x2, y2, x2),
                vec3!(x1 + x2, y1 + y2, x1 + x2)
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) + vec4!(x2, y2, x2, y2),
                vec4!(x1 + x2, y1 + y2, x1 + x2, y1 + y2)
            );

            assert_float_eq!(vec2!(x1, y1) - vec2!(x2, y2), vec2!(x1 - x2, y1 - y2));
            assert_float_eq!(
                vec3!(x1, y1, x1) - vec3!(x2, y2, x2),
                vec3!(x1 - x2, y1 - y2, x1 - x2)
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) - vec4!(x2, y2, x2, y2),
                vec4!(x1 - x2, y1 - y2, x1 - x2, y1 - y2)
            );

            assert_float_eq!(vec2!(x1, y1) * vec2!(x2, y2), vec2!(x1 * x2, y1 * y2));
            assert_float_eq!(
                vec3!(x1, y1, x1) * vec3!(x2, y2, x2),
                vec3!(x1 * x2, y1 * y2, x1 * x2)
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) * vec4!(x2, y2, x2, y2),
                vec4!(x1 * x2, y1 * y2, x1 * x2, y1 * y2)
            );

            assert_float_eq!(vec2!(x1, y1) / vec2!(x2, y2), vec2!(x1 / x2, y1 / y2));
            assert_float_eq!(
                vec3!(x1, y1, x1) / vec3!(x2, y2, x2),
                vec3!(x1 / x2, y1 / y2, x1 / x2)
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) / vec4!(x2, y2, x2, y2),
                vec4!(x1 / x2, y1 / y2, x1 / x2, y1 / y2)
            );

            assert_float_eq!(vec2!(x1, y1) % vec2!(x2, y2), vec2!(x1 % x2, y1 % y2));
            assert_float_eq!(
                vec3!(x1, y1, x1) % vec3!(x2, y2, x2),
                vec3!(x1 % x2, y1 % y2, x1 % x2)
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1) % vec4!(x2, y2, x2, y2),
                vec4!(x1 % x2, y1 % y2, x1 % x2, y1 % y2)
            );

            assert_float_eq!(
                vec2!(x1, y1).div_euclid(vec2!(x2, y2)),
                vec2!(x1.div_euclid(x2), y1.div_euclid(y2))
            );
            assert_float_eq!(
                vec3!(x1, y1, x1).div_euclid(vec3!(x2, y2, x2)),
                vec3!(x1.div_euclid(x2), y1.div_euclid(y2), x1.div_euclid(x2))
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).div_euclid(vec4!(x2, y2, x2, y2)),
                vec4!(
                    x1.div_euclid(x2),
                    y1.div_euclid(y2),
                    x1.div_euclid(x2),
                    y1.div_euclid(y2)
                )
            );

            assert_float_eq!(
                vec2!(x1, y1).rem_euclid(vec2!(x2, y2)),
                vec2!(x1.rem_euclid(x2), y1.rem_euclid(y2))
            );
            assert_float_eq!(
                vec3!(x1, y1, x1).rem_euclid(vec3!(x2, y2, x2)),
                vec3!(x1.rem_euclid(x2), y1.rem_euclid(y2), x1.rem_euclid(x2))
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).rem_euclid(vec4!(x2, y2, x2, y2)),
                vec4!(
                    x1.rem_euclid(x2),
                    y1.rem_euclid(y2),
                    x1.rem_euclid(x2),
                    y1.rem_euclid(y2)
                )
            );

            // `Vector::max` and `Vector::min` panic if any element is NaN.
            // if `assert` is disabled, the result is unspecified.
            if [x1, y1, x2, y2].into_iter().any(T::is_nan) {
                ggmath_assert_panic!(vec2!(x1, y1).max(vec2!(x2, y2)));
                ggmath_assert_panic!(vec3!(x1, y1, x1).max(vec3!(x2, y2, x2)));
                ggmath_assert_panic!(vec4!(x1, y1, x1, y1).max(vec4!(x2, y2, x2, y2)));

                ggmath_assert_panic!(vec2!(x1, y1).min(vec2!(x2, y2)));
                ggmath_assert_panic!(vec3!(x1, y1, x1).min(vec3!(x2, y2, x2)));
                ggmath_assert_panic!(vec4!(x1, y1, x1, y1).min(vec4!(x2, y2, x2, y2)));
            } else {
                assert_float_eq!(
                    vec2!(x1, y1).max(vec2!(x2, y2)),
                    vec2!(x1.max(x2), y1.max(y2))
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).max(vec3!(x2, y2, x2)),
                    vec3!(x1.max(x2), y1.max(y2), x1.max(x2))
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).max(vec4!(x2, y2, x2, y2)),
                    vec4!(x1.max(x2), y1.max(y2), x1.max(x2), y1.max(y2))
                );

                assert_float_eq!(
                    vec2!(x1, y1).min(vec2!(x2, y2)),
                    vec2!(x1.min(x2), y1.min(y2))
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).min(vec3!(x2, y2, x2)),
                    vec3!(x1.min(x2), y1.min(y2), x1.min(x2))
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).min(vec4!(x2, y2, x2, y2)),
                    vec4!(x1.min(x2), y1.min(y2), x1.min(x2), y1.min(y2))
                );
            }

            assert_float_eq!(
                vec2!(x1, y1).midpoint(vec2!(x2, y2)),
                vec2!(x1.midpoint(x2), y1.midpoint(y2))
            );
            assert_float_eq!(
                vec3!(x1, y1, x1).midpoint(vec3!(x2, y2, x2)),
                vec3!(x1.midpoint(x2), y1.midpoint(y2), x1.midpoint(x2))
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).midpoint(vec4!(x2, y2, x2, y2)),
                vec4!(
                    x1.midpoint(x2),
                    y1.midpoint(y2),
                    x1.midpoint(x2),
                    y1.midpoint(y2)
                )
            );

            assert_float_eq!(
                vec2!(x1, y1).copysign(vec2!(x2, y2)),
                vec2!(x1.copysign(x2), y1.copysign(y2))
            );
            assert_float_eq!(
                vec3!(x1, y1, x1).copysign(vec3!(x2, y2, x2)),
                vec3!(x1.copysign(x2), y1.copysign(y2), x1.copysign(x2))
            );
            assert_float_eq!(
                vec4!(x1, y1, x1, y1).copysign(vec4!(x2, y2, x2, y2)),
                vec4!(
                    x1.copysign(x2),
                    y1.copysign(y2),
                    x1.copysign(x2),
                    y1.copysign(y2)
                )
            );

            for (x3, y3) in VALUES {
                assert_float_eq!(
                    vec2!(x1, y1).mul_add(vec2!(x2, y2), vec2!(x3, y3)),
                    vec2!(x1.mul_add(x2, x3), y1.mul_add(y2, y3))
                );
                assert_float_eq!(
                    vec3!(x1, y1, x1).mul_add(vec3!(x2, y2, x2), vec3!(x3, y3, x3)),
                    vec3!(x1.mul_add(x2, x3), y1.mul_add(y2, y3), x1.mul_add(x2, x3))
                );
                assert_float_eq!(
                    vec4!(x1, y1, x1, y1).mul_add(vec4!(x2, y2, x2, y2), vec4!(x3, y3, x3, y3)),
                    vec4!(
                        x1.mul_add(x2, x3),
                        y1.mul_add(y2, y3),
                        x1.mul_add(x2, x3),
                        y1.mul_add(y2, y3)
                    )
                );

                // `Vector::clamp` panics when either theres a NaN or `min` > `max`.
                // if `assert` is disabled, the result is unspecified.
                if [x1, y1, x2, y2, x3, y3].into_iter().any(T::is_nan) || x2 > x3 || y2 > y3 {
                    ggmath_assert_panic!(vec2!(x1, y1).clamp(vec2!(x2, y2), vec2!(x3, y3)));
                    ggmath_assert_panic!(
                        vec3!(x1, y1, x1).clamp(vec3!(x2, y2, x2), vec3!(x3, y3, x3))
                    );
                    ggmath_assert_panic!(
                        vec4!(x1, y1, x1, y1).clamp(vec4!(x2, y2, x2, y2), vec4!(x3, y3, x3, y3))
                    );
                } else {
                    assert_float_eq!(
                        vec2!(x1, y1).clamp(vec2!(x2, y2), vec2!(x3, y3)),
                        vec2!(x1.clamp(x2, x3), y1.clamp(y2, y3))
                    );
                    assert_float_eq!(
                        vec3!(x1, y1, x1).clamp(vec3!(x2, y2, x2), vec3!(x3, y3, x3)),
                        vec3!(x1.clamp(x2, x3), y1.clamp(y2, y3), x1.clamp(x2, x3))
                    );
                    assert_float_eq!(
                        vec4!(x1, y1, x1, y1).clamp(vec4!(x2, y2, x2, y2), vec4!(x3, y3, x3, y3)),
                        vec4!(
                            x1.clamp(x2, x3),
                            y1.clamp(y2, y3),
                            x1.clamp(x2, x3),
                            y1.clamp(y2, y3)
                        )
                    );
                }
            }
        }
    }
}

type Vec2t = Vector<2, T>;
type Vec3t = Vector<3, T>;
type Vec4t = Vector<4, T>;

trait FloatEq {
    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool;
    fn approx_eq(self, other: Self, epsilon: T, ignore_zero_sign: bool) -> bool;
}

macro_rules! assert_float_eq {
    ($a:expr, $b:expr $(,)?) => {
        if !$a.float_eq($b, false) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}",
                stringify!($a),
                stringify!($b),
                $b,
                $a
            );
        }
    };
    ($a:expr, $b:expr, 0.0 = -0.0, $(,)?) => {
        if !$a.float_eq($b, true) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}",
                stringify!($a),
                stringify!($b),
                $b,
                $a
            );
        }
    };
}

use assert_float_eq;

#[expect(unused)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $e:expr $(,)?) => {
        if !$a.approx_eq($b, $e, false) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}",
                stringify!($a),
                stringify!($b),
                $b,
                $a
            );
        }
    };
    ($a:expr, $b:expr, $e:expr, 0.0 = -0.0, $(,)?) => {
        if !$a.approx_eq($b, $e, true) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}",
                stringify!($a),
                stringify!($b),
                $b,
                $a
            );
        }
    };
}

use assert_approx_eq;

impl FloatEq for T {
    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self == 0.0 && other == 0.0 && ignore_zero_sign {
            true
        } else if self.is_sign_positive() != other.is_sign_positive() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            self == other
        }
    }

    fn approx_eq(self, other: Self, epsilon: T, ignore_zero_sign: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self == 0.0 && other == 0.0 && ignore_zero_sign {
            true
        } else if self.is_sign_positive() != other.is_sign_positive() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            (self - other).abs() < epsilon
        }
    }
}

impl<const N: usize> FloatEq for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.float_eq(b, ignore_zero_sign))
    }

    fn approx_eq(self, other: Self, epsilon: T, ignore_zero_sign: bool) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.approx_eq(b, epsilon, ignore_zero_sign))
    }
}

impl<S: FloatEq> FloatEq for Option<S> {
    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        match (self, other) {
            (Some(value), Some(other)) => value.float_eq(other, ignore_zero_sign),
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
        }
    }

    fn approx_eq(self, other: Self, epsilon: T, ignore_zero_sign: bool) -> bool {
        match (self, other) {
            (Some(value), Some(other)) => value.approx_eq(other, epsilon, ignore_zero_sign),
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
        }
    }
}
