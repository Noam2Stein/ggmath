use ggmath::{Alignment, Mask, Vector};
use itertools::iproduct;

pub fn test<A: Alignment>() {
    const ARGS: [T; 15] = [
        0,
        1,
        -1,
        2,
        -2,
        10,
        -10,
        100,
        -100,
        T::MAX / 2,
        T::MIN / 2,
        T::MAX - 1,
        T::MIN + 1,
        T::MAX,
        T::MIN,
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
    }
}
