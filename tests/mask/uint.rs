use ggmath::{Alignment, Mask};
use itertools::iproduct;

pub fn test<A: Alignment>() {
    const ARGS: [T; 9] = [0, 1, 2, 10, 100, 200, T::MAX / 2, T::MAX - 1, T::MAX];

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
                ggmath::Vector::<3, T, A>::from(($($arg,)*))
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
    }
}
