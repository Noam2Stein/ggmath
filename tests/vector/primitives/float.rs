use ggmath::Alignment;
use itertools::iproduct;

use crate::{float_eq, utils::vec3_with_padding};

pub fn float_tests<A: Alignment>() {
    const ARGS: [T; 19] = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        8.6,
        -8.6,
        20.3,
        -20.3,
        1005.2,
        -1005.2,
        500000.1,
        -500000.1,
        100000000000.5,
        -100000000000.5,
        T::MAX,
        T::MIN,
        T::INFINITY,
        T::NEG_INFINITY,
        T::NAN,
    ];

    for (x, y, z) in iproduct!(ARGS, ARGS, ARGS) {
        macro_rules! assert {
            ($($arg:expr),+ $(,)?) => {
                std::assert!($($arg),+, "x = {x:?}, y = {y:?}, z = {z:?}");
            };
        }

        macro_rules! assert_eq {
            ($($arg:expr),+ $(,)?) => {
                std::assert_eq!($($arg),+, "x = {x:?}, y = {y:?}, z = {z:?}");
            };
        }

        macro_rules! assert_float_eq {
            ($($arg:tt)*) => {
                $crate::assert_float_eq!($($arg)+, "x = {x:?}, y = {y:?}, z = {z:?}");
            };
        }

        macro_rules! assert_panic {
            ($($arg:expr),+ $(,)?) => {
                $crate::assert_panic!($($arg),+, "x = {x:?}, y = {y:?}, z = {z:?}");
            };
        }

        macro_rules! vec2 {
            ($($arg:expr),*$(,)?) => {
                ggmath::Vector::<2, T, A>::from(($($arg,)*))
            };
        }

        macro_rules! vec3 {
            ($($arg:expr),*$(,)?) => {
                vec3_with_padding(ggmath::Vector::<3, T, A>::from(($($arg,)*)), T::NAN)
            };
        }

        macro_rules! vec4 {
            ($($arg:expr),*$(,)?) => {
                ggmath::Vector::<4, T, A>::from(($($arg,)*))
            };
        }

        assert_float_eq!(-vec2!(x, y), vec2!(-x, -y));
        assert_float_eq!(-vec3!(x, y, z), vec3!(-x, -y, -z));
        assert_float_eq!(-vec4!(x, y, z, x), vec4!(-x, -y, -z, -x));

        assert_float_eq!(vec2!(x, y) + vec2!(z, x), vec2!(x + z, y + x));
        assert_float_eq!(vec3!(x, y, z) + vec3!(z, x, y), vec3!(x + z, y + x, z + y));
        assert_float_eq!(
            vec4!(x, y, z, x) + vec4!(z, x, y, z),
            vec4!(x + z, y + x, z + y, x + z)
        );

        assert_float_eq!(vec2!(x, y) - vec2!(z, x), vec2!(x - z, y - x));
        assert_float_eq!(vec3!(x, y, z) - vec3!(z, x, y), vec3!(x - z, y - x, z - y));
        assert_float_eq!(
            vec4!(x, y, z, x) - vec4!(z, x, y, z),
            vec4!(x - z, y - x, z - y, x - z)
        );

        assert_float_eq!(vec2!(x, y) * vec2!(z, x), vec2!(x * z, y * x));
        assert_float_eq!(vec3!(x, y, z) * vec3!(z, x, y), vec3!(x * z, y * x, z * y));
        assert_float_eq!(
            vec4!(x, y, z, x) * vec4!(z, x, y, z),
            vec4!(x * z, y * x, z * y, x * z)
        );

        assert_float_eq!(vec2!(x, y) / vec2!(z, x), vec2!(x / z, y / x));
        assert_float_eq!(vec3!(x, y, z) / vec3!(z, x, y), vec3!(x / z, y / x, z / y));
        assert_float_eq!(
            vec4!(x, y, z, x) / vec4!(z, x, y, z),
            vec4!(x / z, y / x, z / y, x / z)
        );

        // Remainder is allowed to be inaccurate for huge values.
        if x.abs() <= T::MAX / 2.0 && y.abs() <= T::MAX / 2.0 && z.abs() <= T::MAX / 2.0 {
            assert_float_eq!(
                vec2!(x, y) % vec2!(z, x),
                vec2!(x % z, y % x),
                tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
            );
            assert_float_eq!(
                vec3!(x, y, z) % vec3!(z, x, y),
                vec3!(x % z, y % x, z % y),
                tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
            );
            assert_float_eq!(
                vec4!(x, y, z, x) % vec4!(z, x, y, z),
                vec4!(x % z, y % x, z % y, x % z),
                tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
            );
        }

        assert_eq!(vec2!(x, y).is_nan(), x.is_nan() || y.is_nan());
        assert_eq!(
            vec3!(x, y, z).is_nan(),
            x.is_nan() || y.is_nan() || z.is_nan()
        );
        assert_eq!(
            vec4!(x, y, z, x).is_nan(),
            x.is_nan() || y.is_nan() || z.is_nan()
        );

        assert_eq!(vec2!(x, y).is_finite(), x.is_finite() && y.is_finite());
        assert_eq!(
            vec3!(x, y, z).is_finite(),
            x.is_finite() && y.is_finite() && z.is_finite()
        );
        assert_eq!(
            vec4!(x, y, z, x).is_finite(),
            x.is_finite() && y.is_finite() && z.is_finite()
        );

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(
                vec2!(x, y).max(vec2!(z, x)),
                vec2!(x.max(z), y.max(x)),
                0.0 = -0.0
            );
            assert_float_eq!(
                vec3!(x, y, z).max(vec3!(z, x, y)),
                vec3!(x.max(z), y.max(x), z.max(y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                vec4!(x, y, z, x).max(vec4!(z, x, y, z)),
                vec4!(x.max(z), y.max(x), z.max(y), x.max(z)),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            assert_panic!(vec2!(x, y).max(vec2!(z, x)));
            assert_panic!(vec3!(x, y, z).max(vec3!(z, x, y)));
            assert_panic!(vec4!(x, y, z, x).max(vec4!(z, x, y, z)));
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(
                vec2!(x, y).min(vec2!(z, x)),
                vec2!(x.min(z), y.min(x)),
                0.0 = -0.0
            );
            assert_float_eq!(
                vec3!(x, y, z).min(vec3!(z, x, y)),
                vec3!(x.min(z), y.min(x), z.min(y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                vec4!(x, y, z, x).min(vec4!(z, x, y, z)),
                vec4!(x.min(z), y.min(x), z.min(y), x.min(z)),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            assert_panic!(vec2!(x, y).min(vec2!(z, x)));
            assert_panic!(vec3!(x, y, z).min(vec3!(z, x, y)));
            assert_panic!(vec4!(x, y, z, x).min(vec4!(z, x, y, z)));
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() && x <= y && y <= z {
            assert_float_eq!(
                vec2!(x, y).clamp(vec2!(x, y), vec2!(y, z)),
                vec2!(x.clamp(x, y), y.clamp(y, z)),
                0.0 = -0.0
            );
            assert_float_eq!(
                vec3!(x, y, z).clamp(vec3!(x, y, x), vec3!(y, z, y)),
                vec3!(x.clamp(x, y), y.clamp(y, y), z.clamp(x, y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                vec4!(x, y, z, x).clamp(vec4!(x, y, x, y), vec4!(y, z, y, z)),
                vec4!(x.clamp(x, y), y.clamp(y, z), z.clamp(x, y), x.clamp(y, z)),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            assert_panic!(vec2!(x, y).clamp(vec2!(x, y), vec2!(y, z)));
            assert_panic!(vec3!(x, y, z).clamp(vec3!(x, y, x), vec3!(y, z, y)));
            assert_panic!(vec4!(x, y, z, x).clamp(vec4!(x, y, x, y), vec4!(y, z, y, z)));
        }

        assert_float_eq!(vec2!(x, y).abs(), vec2!(x.abs(), y.abs()));
        assert_float_eq!(vec3!(x, y, z).abs(), vec3!(x.abs(), y.abs(), z.abs()));
        assert_float_eq!(
            vec4!(x, y, z, x).abs(),
            vec4!(x.abs(), y.abs(), z.abs(), x.abs())
        );

        assert_float_eq!(vec2!(x, y).signum(), vec2!(x.signum(), y.signum()));
        assert_float_eq!(
            vec3!(x, y, z).signum(),
            vec3!(x.signum(), y.signum(), z.signum())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).signum(),
            vec4!(x.signum(), y.signum(), z.signum(), x.signum())
        );

        assert_float_eq!(
            vec2!(x, y).copysign(vec2!(z, x)),
            vec2!(x.copysign(z), y.copysign(x))
        );
        assert_float_eq!(
            vec3!(x, y, z).copysign(vec3!(z, x, y)),
            vec3!(x.copysign(z), y.copysign(x), z.copysign(y))
        );
        assert_float_eq!(
            vec4!(x, y, z, x).copysign(vec4!(z, x, y, z)),
            vec4!(x.copysign(z), y.copysign(x), z.copysign(y), x.copysign(z))
        );

        assert_float_eq!(vec2!(x, y).element_sum(), x + y, 0.0 = -0.0);
        assert!(
            float_eq!(vec3!(x, y, z).element_sum(), x + y + z, 0.0 = -0.0)
                || float_eq!(vec3!(x, y, z).element_sum(), x + z + y, 0.0 = -0.0)
        );
        assert!(
            float_eq!(vec4!(x, y, z, x).element_sum(), x + y + z + x, 0.0 = -0.0)
                || float_eq!(vec4!(x, y, z, x).element_sum(), x + y + (z + x), 0.0 = -0.0)
        );

        assert_float_eq!(vec2!(x, y).element_product(), x * y, 0.0 = -0.0);
        assert!(
            float_eq!(vec3!(x, y, z).element_product(), x * y * z, 0.0 = -0.0)
                || float_eq!(vec3!(x, y, z).element_product(), x * z * y, 0.0 = -0.0)
        );
        assert!(
            float_eq!(
                vec4!(x, y, z, x).element_product(),
                x * y * z * x,
                0.0 = -0.0
            ) || float_eq!(
                vec4!(x, y, z, x).element_product(),
                x * y * (z * x),
                0.0 = -0.0
            )
        );

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(vec2!(x, y).max_element(), x.max(y), 0.0 = -0.0);
            assert_float_eq!(vec3!(x, y, z).max_element(), x.max(y).max(z), 0.0 = -0.0);
            assert_float_eq!(
                vec4!(x, y, z, x).max_element(),
                x.max(y).max(z).max(x),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            if x.is_nan() || y.is_nan() {
                assert_panic!(vec2!(x, y).max_element());
            }

            assert_panic!(vec3!(x, y, z).max_element());
            assert_panic!(vec4!(x, y, z, x).max_element());
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(vec2!(x, y).min_element(), x.min(y), 0.0 = -0.0);
            assert_float_eq!(vec3!(x, y, z).min_element(), x.min(y).min(z), 0.0 = -0.0);
            assert_float_eq!(
                vec4!(x, y, z, x).min_element(),
                x.min(y).min(z).min(x),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            if x.is_nan() || y.is_nan() {
                assert_panic!(vec2!(x, y).min_element());
            }

            assert_panic!(vec3!(x, y, z).min_element());
            assert_panic!(vec4!(x, y, z, x).min_element());
        }

        assert_float_eq!(vec2!(x, y).floor(), vec2!(x.floor(), y.floor()));
        assert_float_eq!(
            vec3!(x, y, z).floor(),
            vec3!(x.floor(), y.floor(), z.floor())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).floor(),
            vec4!(x.floor(), y.floor(), z.floor(), x.floor())
        );

        assert_float_eq!(vec2!(x, y).ceil(), vec2!(x.ceil(), y.ceil()));
        assert_float_eq!(vec3!(x, y, z).ceil(), vec3!(x.ceil(), y.ceil(), z.ceil()));
        assert_float_eq!(
            vec4!(x, y, z, x).ceil(),
            vec4!(x.ceil(), y.ceil(), z.ceil(), x.ceil())
        );

        assert_float_eq!(vec2!(x, y).round(), vec2!(x.round(), y.round()));
        assert_float_eq!(
            vec3!(x, y, z).round(),
            vec3!(x.round(), y.round(), z.round())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).round(),
            vec4!(x.round(), y.round(), z.round(), x.round())
        );

        assert_float_eq!(vec2!(x, y).trunc(), vec2!(x.trunc(), y.trunc()));
        assert_float_eq!(
            vec3!(x, y, z).trunc(),
            vec3!(x.trunc(), y.trunc(), z.trunc())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).trunc(),
            vec4!(x.trunc(), y.trunc(), z.trunc(), x.trunc())
        );

        assert_float_eq!(vec2!(x, y).fract(), vec2!(x.fract(), y.fract()));
        assert_float_eq!(
            vec3!(x, y, z).fract(),
            vec3!(x.fract(), y.fract(), z.fract())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).fract(),
            vec4!(x.fract(), y.fract(), z.fract(), x.fract())
        );

        assert_float_eq!(
            vec2!(x, y).mul_add(vec2!(z, x), vec2!(y, z)),
            vec2!(x.mul_add(z, y), y.mul_add(x, z))
        );
        assert_float_eq!(
            vec3!(x, y, z).mul_add(vec3!(z, x, y), vec3!(x, y, z)),
            vec3!(x.mul_add(z, x), y.mul_add(x, y), z.mul_add(y, z))
        );
        assert_float_eq!(
            vec4!(x, y, z, x).mul_add(vec4!(z, x, y, z), vec4!(x, y, z, x)),
            vec4!(
                x.mul_add(z, x),
                y.mul_add(x, y),
                z.mul_add(y, z),
                x.mul_add(z, x)
            )
        );

        assert_float_eq!(
            vec2!(x, y).div_euclid(vec2!(z, x)),
            vec2!(x.div_euclid(z), y.div_euclid(x))
        );
        assert_float_eq!(
            vec3!(x, y, z).div_euclid(vec3!(z, x, y)),
            vec3!(x.div_euclid(z), y.div_euclid(x), z.div_euclid(y))
        );
        assert_float_eq!(
            vec4!(x, y, z, x).div_euclid(vec4!(z, x, y, z)),
            vec4!(
                x.div_euclid(z),
                y.div_euclid(x),
                z.div_euclid(y),
                x.div_euclid(z)
            )
        );

        assert_float_eq!(
            vec2!(x, y).rem_euclid(vec2!(z, x)),
            vec2!(x.rem_euclid(z), y.rem_euclid(x))
        );
        assert_float_eq!(
            vec3!(x, y, z).rem_euclid(vec3!(z, x, y)),
            vec3!(x.rem_euclid(z), y.rem_euclid(x), z.rem_euclid(y))
        );
        assert_float_eq!(
            vec4!(x, y, z, x).rem_euclid(vec4!(z, x, y, z)),
            vec4!(
                x.rem_euclid(z),
                y.rem_euclid(x),
                z.rem_euclid(y),
                x.rem_euclid(z)
            )
        );

        assert_float_eq!(vec2!(x, y).sqrt(), vec2!(x.sqrt(), y.sqrt()));
        assert_float_eq!(vec3!(x, y, z).sqrt(), vec3!(x.sqrt(), y.sqrt(), z.sqrt()));
        assert_float_eq!(
            vec4!(x, y, z, x).sqrt(),
            vec4!(x.sqrt(), y.sqrt(), z.sqrt(), x.sqrt())
        );

        assert_float_eq!(
            vec2!(x, y).sin(),
            vec2!(x.sin(), y.sin()),
            tol = T::EPSILON * x.abs().max(y.abs())
        );
        assert_float_eq!(
            vec3!(x, y, z).sin(),
            vec3!(x.sin(), y.sin(), z.sin()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).sin(),
            vec4!(x.sin(), y.sin(), z.sin(), x.sin()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(
            vec2!(x, y).cos(),
            vec2!(x.cos(), y.cos()),
            tol = T::EPSILON * x.abs().max(y.abs())
        );
        assert_float_eq!(
            vec3!(x, y, z).cos(),
            vec3!(x.cos(), y.cos(), z.cos()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).cos(),
            vec4!(x.cos(), y.cos(), z.cos(), x.cos()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(vec2!(x, y).tan(), vec2!(x.tan(), y.tan()), ulps <= 4);
        assert_float_eq!(
            vec3!(x, y, z).tan(),
            vec3!(x.tan(), y.tan(), z.tan()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).tan(),
            vec4!(x.tan(), y.tan(), z.tan(), x.tan()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(vec2!(x, y).asin(), vec2!(x.asin(), y.asin()), ulps <= 4);
        assert_float_eq!(
            vec3!(x, y, z).asin(),
            vec3!(x.asin(), y.asin(), z.asin()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).asin(),
            vec4!(x.asin(), y.asin(), z.asin(), x.asin()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(vec2!(x, y).acos(), vec2!(x.acos(), y.acos()), ulps <= 4);
        assert_float_eq!(
            vec3!(x, y, z).acos(),
            vec3!(x.acos(), y.acos(), z.acos()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).acos(),
            vec4!(x.acos(), y.acos(), z.acos(), x.acos()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(vec2!(x, y).atan(), vec2!(x.atan(), y.atan()), ulps <= 4);
        assert_float_eq!(
            vec3!(x, y, z).atan(),
            vec3!(x.atan(), y.atan(), z.atan()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            vec4!(x, y, z, x).atan(),
            vec4!(x.atan(), y.atan(), z.atan(), x.atan()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        if let Some(try_normalize) = vec2!(x, y).try_normalize() {
            assert_float_eq!(vec2!(x, y).normalize(), try_normalize, ulps <= 1);
        } else if cfg!(assertions) {
            assert_panic!(vec2!(x, y).normalize());
        }

        if let Some(try_normalize) = vec3!(x, y, z).try_normalize() {
            assert_float_eq!(vec3!(x, y, z).normalize(), try_normalize, ulps <= 1);
        } else if cfg!(assertions) {
            assert_panic!(vec3!(x, y, z).normalize());
        }

        if let Some(try_normalize) = vec4!(x, y, z, x).try_normalize() {
            assert_float_eq!(vec4!(x, y, z, x).normalize(), try_normalize, ulps <= 1);
        } else if cfg!(assertions) {
            assert_panic!(vec4!(x, y, z, x).normalize());
        }
    }
}
