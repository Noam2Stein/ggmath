use ggmath::{Alignment, Mask};
use itertools::iproduct;

use crate::utils::vec3_with_padding;

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
        macro_rules! assert_eq {
            ($($arg:expr),+ $(,)?) => {
                std::assert_eq!($($arg),+, "x = {x:?}, y = {y:?}, z = {z:?}");
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

        assert_eq!(
            vec2!(x, y).eq_mask(vec2!(z, x)),
            Mask::<2, T, A>::new(x == z, y == x)
        );
        assert_eq!(
            vec3!(x, y, z).eq_mask(vec3!(z, x, y)),
            Mask::<3, T, A>::new(x == z, y == x, z == y)
        );
        assert_eq!(
            vec4!(x, y, z, x).eq_mask(vec4!(z, x, y, z)),
            Mask::<4, T, A>::new(x == z, y == x, z == y, x == z)
        );

        assert_eq!(
            vec2!(x, y).ne_mask(vec2!(z, x)),
            Mask::<2, T, A>::new(x != z, y != x)
        );
        assert_eq!(
            vec3!(x, y, z).ne_mask(vec3!(z, x, y)),
            Mask::<3, T, A>::new(x != z, y != x, z != y)
        );
        assert_eq!(
            vec4!(x, y, z, x).ne_mask(vec4!(z, x, y, z)),
            Mask::<4, T, A>::new(x != z, y != x, z != y, x != z)
        );

        assert_eq!(
            vec2!(x, y).lt_mask(vec2!(z, x)),
            Mask::<2, T, A>::new(x < z, y < x)
        );
        assert_eq!(
            vec3!(x, y, z).lt_mask(vec3!(z, x, y)),
            Mask::<3, T, A>::new(x < z, y < x, z < y)
        );
        assert_eq!(
            vec4!(x, y, z, x).lt_mask(vec4!(z, x, y, z)),
            Mask::<4, T, A>::new(x < z, y < x, z < y, x < z)
        );

        assert_eq!(
            vec2!(x, y).gt_mask(vec2!(z, x)),
            Mask::<2, T, A>::new(x > z, y > x)
        );
        assert_eq!(
            vec3!(x, y, z).gt_mask(vec3!(z, x, y)),
            Mask::<3, T, A>::new(x > z, y > x, z > y)
        );
        assert_eq!(
            vec4!(x, y, z, x).gt_mask(vec4!(z, x, y, z)),
            Mask::<4, T, A>::new(x > z, y > x, z > y, x > z)
        );

        assert_eq!(
            vec2!(x, y).le_mask(vec2!(z, x)),
            Mask::<2, T, A>::new(x <= z, y <= x)
        );
        assert_eq!(
            vec3!(x, y, z).le_mask(vec3!(z, x, y)),
            Mask::<3, T, A>::new(x <= z, y <= x, z <= y)
        );
        assert_eq!(
            vec4!(x, y, z, x).le_mask(vec4!(z, x, y, z)),
            Mask::<4, T, A>::new(x <= z, y <= x, z <= y, x <= z)
        );

        assert_eq!(
            vec2!(x, y).ge_mask(vec2!(z, x)),
            Mask::<2, T, A>::new(x >= z, y >= x)
        );
        assert_eq!(
            vec3!(x, y, z).ge_mask(vec3!(z, x, y)),
            Mask::<3, T, A>::new(x >= z, y >= x, z >= y)
        );
        assert_eq!(
            vec4!(x, y, z, x).ge_mask(vec4!(z, x, y, z)),
            Mask::<4, T, A>::new(x >= z, y >= x, z >= y, x >= z)
        );

        assert_eq!(
            vec2!(x, y).nan_mask(),
            Mask::<2, T, A>::new(x.is_nan(), y.is_nan())
        );
        assert_eq!(
            vec3!(x, y, z).nan_mask(),
            Mask::<3, T, A>::new(x.is_nan(), y.is_nan(), z.is_nan())
        );
        assert_eq!(
            vec4!(x, y, z, x).nan_mask(),
            Mask::<4, T, A>::new(x.is_nan(), y.is_nan(), z.is_nan(), x.is_nan())
        );

        assert_eq!(
            vec2!(x, y).finite_mask(),
            Mask::<2, T, A>::new(x.is_finite(), y.is_finite())
        );
        assert_eq!(
            vec3!(x, y, z).finite_mask(),
            Mask::<3, T, A>::new(x.is_finite(), y.is_finite(), z.is_finite())
        );
        assert_eq!(
            vec4!(x, y, z, x).finite_mask(),
            Mask::<4, T, A>::new(x.is_finite(), y.is_finite(), z.is_finite(), x.is_finite())
        );

        assert_eq!(
            vec2!(x, y).sign_positive_mask(),
            Mask::<2, T, A>::new(x.is_sign_positive(), y.is_sign_positive())
        );
        assert_eq!(
            vec3!(x, y, z).sign_positive_mask(),
            Mask::<3, T, A>::new(
                x.is_sign_positive(),
                y.is_sign_positive(),
                z.is_sign_positive()
            )
        );
        assert_eq!(
            vec4!(x, y, z, x).sign_positive_mask(),
            Mask::<4, T, A>::new(
                x.is_sign_positive(),
                y.is_sign_positive(),
                z.is_sign_positive(),
                x.is_sign_positive()
            )
        );

        assert_eq!(
            vec2!(x, y).sign_negative_mask(),
            Mask::<2, T, A>::new(x.is_sign_negative(), y.is_sign_negative())
        );
        assert_eq!(
            vec3!(x, y, z).sign_negative_mask(),
            Mask::<3, T, A>::new(
                x.is_sign_negative(),
                y.is_sign_negative(),
                z.is_sign_negative()
            )
        );
        assert_eq!(
            vec4!(x, y, z, x).sign_negative_mask(),
            Mask::<4, T, A>::new(
                x.is_sign_negative(),
                y.is_sign_negative(),
                z.is_sign_negative(),
                x.is_sign_negative()
            )
        );
    }
}
