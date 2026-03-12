use ggmath::{Alignment, Vector};
use itertools::iproduct;

use crate::float_eq;

pub fn test<A: Alignment>() {
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

        assert_float_eq!(
            -Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(-x, -y)
        );
        assert_float_eq!(
            -Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(-x, -y, -z)
        );
        assert_float_eq!(
            -Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(-x, -y, -z, -x)
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y) + Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x + z, y + x)
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z) + Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x + z, y + x, z + y)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x) + Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x + z, y + x, z + y, x + z)
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y) - Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x - z, y - x)
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z) - Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x - z, y - x, z - y)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x) - Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x - z, y - x, z - y, x - z)
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y) * Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x * z, y * x)
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z) * Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x * z, y * x, z * y)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x) * Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x * z, y * x, z * y, x * z)
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y) / Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x / z, y / x)
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z) / Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x / z, y / x, z / y)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x) / Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x / z, y / x, z / y, x / z)
        );

        // Remainder is allowed to be inaccurate for huge values.
        if x.abs() <= T::MAX / 2.0 && y.abs() <= T::MAX / 2.0 && z.abs() <= T::MAX / 2.0 {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y) % Vector::<2, T, A>::new(z, x),
                Vector::<2, T, A>::new(x % z, y % x),
                tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z) % Vector::<3, T, A>::new(z, x, y),
                Vector::<3, T, A>::new(x % z, y % x, z % y),
                tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x) % Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(x % z, y % x, z % y, x % z),
                tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
            );
        }

        assert_eq!(
            Vector::<2, T, A>::new(x, y).is_nan(),
            x.is_nan() || y.is_nan()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).is_nan(),
            x.is_nan() || y.is_nan() || z.is_nan()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).is_nan(),
            x.is_nan() || y.is_nan() || z.is_nan()
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).is_finite(),
            x.is_finite() && y.is_finite()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).is_finite(),
            x.is_finite() && y.is_finite() && z.is_finite()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).is_finite(),
            x.is_finite() && y.is_finite() && z.is_finite()
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).element_sum(),
            x + y,
            0.0 = -0.0
        );
        assert!(
            float_eq!(
                Vector::<3, T, A>::new(x, y, z).element_sum(),
                x + y + z,
                0.0 = -0.0
            ) || float_eq!(
                Vector::<3, T, A>::new(x, y, z).element_sum(),
                x + z + y,
                0.0 = -0.0
            )
        );
        assert!(
            float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_sum(),
                x + y + z + x,
                0.0 = -0.0
            ) || float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_sum(),
                x + y + (z + x),
                0.0 = -0.0
            )
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).element_product(),
            x * y,
            0.0 = -0.0
        );
        assert!(
            float_eq!(
                Vector::<3, T, A>::new(x, y, z).element_product(),
                x * y * z,
                0.0 = -0.0
            ) || float_eq!(
                Vector::<3, T, A>::new(x, y, z).element_product(),
                x * z * y,
                0.0 = -0.0
            )
        );
        assert!(
            float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_product(),
                x * y * z * x,
                0.0 = -0.0
            ) || float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_product(),
                x * y * (z * x),
                0.0 = -0.0
            )
        );

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).max(Vector::<2, T, A>::new(z, x)),
                Vector::<2, T, A>::new(x.max(z), y.max(x)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).max(Vector::<3, T, A>::new(z, x, y)),
                Vector::<3, T, A>::new(x.max(z), y.max(x), z.max(y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).max(Vector::<4, T, A>::new(z, x, y, z)),
                Vector::<4, T, A>::new(x.max(z), y.max(x), z.max(y), x.max(z)),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            assert_panic!(Vector::<2, T, A>::new(x, y).max(Vector::<2, T, A>::new(z, x)));
            assert_panic!(Vector::<3, T, A>::new(x, y, z).max(Vector::<3, T, A>::new(z, x, y)));
            assert_panic!(
                Vector::<4, T, A>::new(x, y, z, x).max(Vector::<4, T, A>::new(z, x, y, z))
            );
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).min(Vector::<2, T, A>::new(z, x)),
                Vector::<2, T, A>::new(x.min(z), y.min(x)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).min(Vector::<3, T, A>::new(z, x, y)),
                Vector::<3, T, A>::new(x.min(z), y.min(x), z.min(y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).min(Vector::<4, T, A>::new(z, x, y, z)),
                Vector::<4, T, A>::new(x.min(z), y.min(x), z.min(y), x.min(z)),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            assert_panic!(Vector::<2, T, A>::new(x, y).min(Vector::<2, T, A>::new(z, x)));
            assert_panic!(Vector::<3, T, A>::new(x, y, z).min(Vector::<3, T, A>::new(z, x, y)));
            assert_panic!(
                Vector::<4, T, A>::new(x, y, z, x).min(Vector::<4, T, A>::new(z, x, y, z))
            );
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() && x <= y && y <= z {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y)
                    .clamp(Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(y, z)),
                Vector::<2, T, A>::new(x.clamp(x, y), y.clamp(y, z)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).clamp(
                    Vector::<3, T, A>::new(x, y, x),
                    Vector::<3, T, A>::new(y, z, y)
                ),
                Vector::<3, T, A>::new(x.clamp(x, y), y.clamp(y, y), z.clamp(x, y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).clamp(
                    Vector::<4, T, A>::new(x, y, x, y),
                    Vector::<4, T, A>::new(y, z, y, z)
                ),
                Vector::<4, T, A>::new(x.clamp(x, y), y.clamp(y, z), z.clamp(x, y), x.clamp(y, z)),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            assert_panic!(
                Vector::<2, T, A>::new(x, y)
                    .clamp(Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(y, z))
            );
            assert_panic!(Vector::<3, T, A>::new(x, y, z).clamp(
                Vector::<3, T, A>::new(x, y, x),
                Vector::<3, T, A>::new(y, z, y)
            ));
            assert_panic!(Vector::<4, T, A>::new(x, y, z, x).clamp(
                Vector::<4, T, A>::new(x, y, x, y),
                Vector::<4, T, A>::new(y, z, y, z)
            ));
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).max_element(),
                x.max(y),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).max_element(),
                x.max(y).max(z),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).max_element(),
                x.max(y).max(z).max(x),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            if x.is_nan() || y.is_nan() {
                assert_panic!(Vector::<2, T, A>::new(x, y).max_element());
            }

            assert_panic!(Vector::<3, T, A>::new(x, y, z).max_element());
            assert_panic!(Vector::<4, T, A>::new(x, y, z, x).max_element());
        }

        if !x.is_nan() && !y.is_nan() && !z.is_nan() {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).min_element(),
                x.min(y),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).min_element(),
                x.min(y).min(z),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).min_element(),
                x.min(y).min(z).min(x),
                0.0 = -0.0
            );
        } else if cfg!(assertions) {
            if x.is_nan() || y.is_nan() {
                assert_panic!(Vector::<2, T, A>::new(x, y).min_element());
            }

            assert_panic!(Vector::<3, T, A>::new(x, y, z).min_element());
            assert_panic!(Vector::<4, T, A>::new(x, y, z, x).min_element());
        }

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).abs(),
            Vector::<2, T, A>::new(x.abs(), y.abs())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).abs(),
            Vector::<3, T, A>::new(x.abs(), y.abs(), z.abs())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).abs(),
            Vector::<4, T, A>::new(x.abs(), y.abs(), z.abs(), x.abs())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).signum(),
            Vector::<2, T, A>::new(x.signum(), y.signum())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).signum(),
            Vector::<3, T, A>::new(x.signum(), y.signum(), z.signum())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).signum(),
            Vector::<4, T, A>::new(x.signum(), y.signum(), z.signum(), x.signum())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).copysign(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.copysign(z), y.copysign(x))
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).copysign(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.copysign(z), y.copysign(x), z.copysign(y))
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).copysign(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(x.copysign(z), y.copysign(x), z.copysign(y), x.copysign(z))
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).floor(),
            Vector::<2, T, A>::new(x.floor(), y.floor())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).floor(),
            Vector::<3, T, A>::new(x.floor(), y.floor(), z.floor())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).floor(),
            Vector::<4, T, A>::new(x.floor(), y.floor(), z.floor(), x.floor())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).ceil(),
            Vector::<2, T, A>::new(x.ceil(), y.ceil())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).ceil(),
            Vector::<3, T, A>::new(x.ceil(), y.ceil(), z.ceil())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).ceil(),
            Vector::<4, T, A>::new(x.ceil(), y.ceil(), z.ceil(), x.ceil())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).round(),
            Vector::<2, T, A>::new(x.round(), y.round())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).round(),
            Vector::<3, T, A>::new(x.round(), y.round(), z.round())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).round(),
            Vector::<4, T, A>::new(x.round(), y.round(), z.round(), x.round())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).trunc(),
            Vector::<2, T, A>::new(x.trunc(), y.trunc())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).trunc(),
            Vector::<3, T, A>::new(x.trunc(), y.trunc(), z.trunc())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).trunc(),
            Vector::<4, T, A>::new(x.trunc(), y.trunc(), z.trunc(), x.trunc())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).fract(),
            Vector::<2, T, A>::new(x.fract(), y.fract())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).fract(),
            Vector::<3, T, A>::new(x.fract(), y.fract(), z.fract())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).fract(),
            Vector::<4, T, A>::new(x.fract(), y.fract(), z.fract(), x.fract())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y)
                .mul_add(Vector::<2, T, A>::new(z, x), Vector::<2, T, A>::new(y, z)),
            Vector::<2, T, A>::new(x.mul_add(z, y), y.mul_add(x, z))
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).mul_add(
                Vector::<3, T, A>::new(z, x, y),
                Vector::<3, T, A>::new(x, y, z)
            ),
            Vector::<3, T, A>::new(x.mul_add(z, x), y.mul_add(x, y), z.mul_add(y, z))
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).mul_add(
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(x, y, z, x)
            ),
            Vector::<4, T, A>::new(
                x.mul_add(z, x),
                y.mul_add(x, y),
                z.mul_add(y, z),
                x.mul_add(z, x)
            )
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).div_euclid(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.div_euclid(z), y.div_euclid(x))
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).div_euclid(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.div_euclid(z), y.div_euclid(x), z.div_euclid(y))
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).div_euclid(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.div_euclid(z),
                y.div_euclid(x),
                z.div_euclid(y),
                x.div_euclid(z)
            )
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).rem_euclid(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.rem_euclid(z), y.rem_euclid(x))
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).rem_euclid(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.rem_euclid(z), y.rem_euclid(x), z.rem_euclid(y))
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).rem_euclid(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.rem_euclid(z),
                y.rem_euclid(x),
                z.rem_euclid(y),
                x.rem_euclid(z)
            )
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).sqrt(),
            Vector::<2, T, A>::new(x.sqrt(), y.sqrt())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).sqrt(),
            Vector::<3, T, A>::new(x.sqrt(), y.sqrt(), z.sqrt())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).sqrt(),
            Vector::<4, T, A>::new(x.sqrt(), y.sqrt(), z.sqrt(), x.sqrt())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).sin(),
            Vector::<2, T, A>::new(x.sin(), y.sin()),
            tol = T::EPSILON * x.abs().max(y.abs())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).sin(),
            Vector::<3, T, A>::new(x.sin(), y.sin(), z.sin()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).sin(),
            Vector::<4, T, A>::new(x.sin(), y.sin(), z.sin(), x.sin()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).cos(),
            Vector::<2, T, A>::new(x.cos(), y.cos()),
            tol = T::EPSILON * x.abs().max(y.abs())
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).cos(),
            Vector::<3, T, A>::new(x.cos(), y.cos(), z.cos()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).cos(),
            Vector::<4, T, A>::new(x.cos(), y.cos(), z.cos(), x.cos()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).tan(),
            Vector::<2, T, A>::new(x.tan(), y.tan()),
            ulps <= 4
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).tan(),
            Vector::<3, T, A>::new(x.tan(), y.tan(), z.tan()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).tan(),
            Vector::<4, T, A>::new(x.tan(), y.tan(), z.tan(), x.tan()),
            tol = T::EPSILON * x.abs().max(y.abs()).max(z.abs())
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).asin(),
            Vector::<2, T, A>::new(x.asin(), y.asin()),
            ulps <= 4
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).asin(),
            Vector::<3, T, A>::new(x.asin(), y.asin(), z.asin()),
            tol = T::EPSILON * 2.0 * x.abs().max(y.abs()).max(z.abs()).max(1.0)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).asin(),
            Vector::<4, T, A>::new(x.asin(), y.asin(), z.asin(), x.asin()),
            tol = T::EPSILON * 2.0 * x.abs().max(y.abs()).max(z.abs()).max(1.0)
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).acos(),
            Vector::<2, T, A>::new(x.acos(), y.acos()),
            ulps <= 4
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).acos(),
            Vector::<3, T, A>::new(x.acos(), y.acos(), z.acos()),
            tol = T::EPSILON * 2.0 * x.abs().max(y.abs()).max(z.abs()).max(1.0)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).acos(),
            Vector::<4, T, A>::new(x.acos(), y.acos(), z.acos(), x.acos()),
            tol = T::EPSILON * 2.0 * x.abs().max(y.abs()).max(z.abs()).max(1.0)
        );

        assert_float_eq!(
            Vector::<2, T, A>::new(x, y).atan(),
            Vector::<2, T, A>::new(x.atan(), y.atan()),
            ulps <= 4
        );
        assert_float_eq!(
            Vector::<3, T, A>::new(x, y, z).atan(),
            Vector::<3, T, A>::new(x.atan(), y.atan(), z.atan()),
            tol = T::EPSILON * 2.0 * x.abs().max(y.abs()).max(z.abs()).max(1.0)
        );
        assert_float_eq!(
            Vector::<4, T, A>::new(x, y, z, x).atan(),
            Vector::<4, T, A>::new(x.atan(), y.atan(), z.atan(), x.atan()),
            tol = T::EPSILON * 2.0 * x.abs().max(y.abs()).max(z.abs()).max(1.0)
        );

        if let Some(try_normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).normalize(),
                try_normalize,
                ulps <= 1
            );
        } else if cfg!(assertions) {
            assert_panic!(Vector::<2, T, A>::new(x, y).normalize());
        }

        if let Some(try_normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).normalize(),
                try_normalize,
                ulps <= 1
            );
        } else if cfg!(assertions) {
            assert_panic!(Vector::<3, T, A>::new(x, y, z).normalize());
        }

        if let Some(try_normalize) = Vector::<4, T, A>::new(x, y, z, x).try_normalize() {
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, x).normalize(),
                try_normalize,
                ulps <= 1
            );
        } else if cfg!(assertions) {
            assert_panic!(Vector::<4, T, A>::new(x, y, z, x).normalize());
        }
    }
}
