use ggmath::{Alignment, Mask, Vector};
use itertools::iproduct;

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

        assert_eq!(
            Vector::<2, T, A>::new(x, y).eq_mask(Vector::<2, T, A>::new(z, x)),
            Mask::<2, T, A>::new(x == z, y == x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).eq_mask(Vector::<3, T, A>::new(z, x, y)),
            Mask::<3, T, A>::new(x == z, y == x, z == y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).eq_mask(Vector::<4, T, A>::new(z, x, y, z)),
            Mask::<4, T, A>::new(x == z, y == x, z == y, x == z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).ne_mask(Vector::<2, T, A>::new(z, x)),
            Mask::<2, T, A>::new(x != z, y != x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).ne_mask(Vector::<3, T, A>::new(z, x, y)),
            Mask::<3, T, A>::new(x != z, y != x, z != y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).ne_mask(Vector::<4, T, A>::new(z, x, y, z)),
            Mask::<4, T, A>::new(x != z, y != x, z != y, x != z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).lt_mask(Vector::<2, T, A>::new(z, x)),
            Mask::<2, T, A>::new(x < z, y < x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).lt_mask(Vector::<3, T, A>::new(z, x, y)),
            Mask::<3, T, A>::new(x < z, y < x, z < y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).lt_mask(Vector::<4, T, A>::new(z, x, y, z)),
            Mask::<4, T, A>::new(x < z, y < x, z < y, x < z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).gt_mask(Vector::<2, T, A>::new(z, x)),
            Mask::<2, T, A>::new(x > z, y > x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).gt_mask(Vector::<3, T, A>::new(z, x, y)),
            Mask::<3, T, A>::new(x > z, y > x, z > y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).gt_mask(Vector::<4, T, A>::new(z, x, y, z)),
            Mask::<4, T, A>::new(x > z, y > x, z > y, x > z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).le_mask(Vector::<2, T, A>::new(z, x)),
            Mask::<2, T, A>::new(x <= z, y <= x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).le_mask(Vector::<3, T, A>::new(z, x, y)),
            Mask::<3, T, A>::new(x <= z, y <= x, z <= y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).le_mask(Vector::<4, T, A>::new(z, x, y, z)),
            Mask::<4, T, A>::new(x <= z, y <= x, z <= y, x <= z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).ge_mask(Vector::<2, T, A>::new(z, x)),
            Mask::<2, T, A>::new(x >= z, y >= x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).ge_mask(Vector::<3, T, A>::new(z, x, y)),
            Mask::<3, T, A>::new(x >= z, y >= x, z >= y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).ge_mask(Vector::<4, T, A>::new(z, x, y, z)),
            Mask::<4, T, A>::new(x >= z, y >= x, z >= y, x >= z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).nan_mask(),
            Mask::<2, T, A>::new(x.is_nan(), y.is_nan())
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).nan_mask(),
            Mask::<3, T, A>::new(x.is_nan(), y.is_nan(), z.is_nan())
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).nan_mask(),
            Mask::<4, T, A>::new(x.is_nan(), y.is_nan(), z.is_nan(), x.is_nan())
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).finite_mask(),
            Mask::<2, T, A>::new(x.is_finite(), y.is_finite())
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).finite_mask(),
            Mask::<3, T, A>::new(x.is_finite(), y.is_finite(), z.is_finite())
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).finite_mask(),
            Mask::<4, T, A>::new(x.is_finite(), y.is_finite(), z.is_finite(), x.is_finite())
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).sign_positive_mask(),
            Mask::<2, T, A>::new(x.is_sign_positive(), y.is_sign_positive())
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).sign_positive_mask(),
            Mask::<3, T, A>::new(
                x.is_sign_positive(),
                y.is_sign_positive(),
                z.is_sign_positive()
            )
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).sign_positive_mask(),
            Mask::<4, T, A>::new(
                x.is_sign_positive(),
                y.is_sign_positive(),
                z.is_sign_positive(),
                x.is_sign_positive()
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).sign_negative_mask(),
            Mask::<2, T, A>::new(x.is_sign_negative(), y.is_sign_negative())
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).sign_negative_mask(),
            Mask::<3, T, A>::new(
                x.is_sign_negative(),
                y.is_sign_negative(),
                z.is_sign_negative()
            )
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).sign_negative_mask(),
            Mask::<4, T, A>::new(
                x.is_sign_negative(),
                y.is_sign_negative(),
                z.is_sign_negative(),
                x.is_sign_negative()
            )
        );
    }
}
