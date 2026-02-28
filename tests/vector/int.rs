use ggmath::{Alignment, Vector};
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
            ($left:expr, $right:expr $(,)?) => {
                std::assert_eq!(
                    std::panic::catch_unwind(|| $left).map_err(|_| ()),
                    std::panic::catch_unwind(|| $right).map_err(|_| ()),
                    "x = {x:?}, y = {y:?}, z = {z:?}",
                );
            };
        }

        assert_eq!(
            -Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(-x, -y)
        );
        assert_eq!(
            -Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(-x, -y, -z)
        );
        assert_eq!(
            -Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(-x, -y, -z, -x)
        );

        assert_eq!(
            !Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(!x, !y)
        );
        assert_eq!(
            !Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(!x, !y, !z)
        );
        assert_eq!(
            !Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(!x, !y, !z, !x)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) + Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x + z, y + x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) + Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x + z, y + x, z + y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) + Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x + z, y + x, z + y, x + z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) - Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x - z, y - x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) - Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x - z, y - x, z - y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) - Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x - z, y - x, z - y, x - z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) * Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x * z, y * x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) * Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x * z, y * x, z * y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) * Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x * z, y * x, z * y, x * z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) / Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x / z, y / x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) / Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x / z, y / x, z / y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) / Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x / z, y / x, z / y, x / z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) % Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x % z, y % x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) % Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x % z, y % x, z % y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) % Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x % z, y % x, z % y, x % z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) << Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x << z, y << x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) << Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x << z, y << x, z << y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) << Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x << z, y << x, z << y, x << z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) >> Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x >> z, y >> x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) >> Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x >> z, y >> x, z >> y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) >> Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x >> z, y >> x, z >> y, x >> z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) & Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x & z, y & x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) & Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x & z, y & x, z & y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) & Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x & z, y & x, z & y, x & z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) | Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x | z, y | x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) | Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x | z, y | x, z | y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) | Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x | z, y | x, z | y, x | z)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y) ^ Vector::<2, T, A>::new(z, x),
            Vector::<2, T, A>::new(x ^ z, y ^ x)
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z) ^ Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(x ^ z, y ^ x, z ^ y)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x) ^ Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x ^ z, y ^ x, z ^ y, x ^ z)
        );

        if cfg!(assertions) {
            assert_eq!(
                Vector::<2, T, A>::new(x, y).element_sum(),
                x.checked_add(y).unwrap()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).element_sum(),
                x.checked_add(y).unwrap().checked_add(z).unwrap()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_sum(),
                x.checked_add(y)
                    .unwrap()
                    .checked_add(z)
                    .unwrap()
                    .checked_add(x)
                    .unwrap()
            );
        } else {
            assert_eq!(
                Vector::<2, T, A>::new(x, y).element_sum(),
                x.wrapping_add(y)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).element_sum(),
                x.wrapping_add(y).wrapping_add(z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_sum(),
                x.wrapping_add(y).wrapping_add(z).wrapping_add(x)
            );
        }

        if cfg!(assertions) {
            assert_eq!(
                Vector::<2, T, A>::new(x, y).element_product(),
                x.checked_mul(y).unwrap()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).element_product(),
                x.checked_mul(y).unwrap().checked_mul(z).unwrap()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_product(),
                x.checked_mul(y)
                    .unwrap()
                    .checked_mul(z)
                    .unwrap()
                    .checked_mul(x)
                    .unwrap()
            );
        } else {
            assert_eq!(
                Vector::<2, T, A>::new(x, y).element_product(),
                x.wrapping_mul(y)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).element_product(),
                x.wrapping_mul(y).wrapping_mul(z)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, x).element_product(),
                x.wrapping_mul(y).wrapping_mul(z).wrapping_mul(x)
            );
        }

        assert_eq!(
            Vector::<2, T, A>::new(x, y).max(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.max(z), y.max(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).max(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.max(z), y.max(x), z.max(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).max(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(x.max(z), y.max(x), z.max(y), x.max(z))
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).min(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.min(z), y.min(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).min(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.min(z), y.min(x), z.min(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).min(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(x.min(z), y.min(x), z.min(y), x.min(z))
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y)
                .clamp(Vector::<2, T, A>::new(z, x), Vector::<2, T, A>::new(y, z)),
            Vector::<2, T, A>::new(x.clamp(z, y), y.clamp(x, z))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).clamp(
                Vector::<3, T, A>::new(z, x, y),
                Vector::<3, T, A>::new(y, z, x)
            ),
            Vector::<3, T, A>::new(x.clamp(z, y), y.clamp(x, z), z.clamp(y, x))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).clamp(
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(y, z, x, y)
            ),
            Vector::<4, T, A>::new(x.clamp(z, y), y.clamp(x, z), z.clamp(y, x), x.clamp(z, y))
        );

        assert_eq!(Vector::<2, T, A>::new(x, y).max_element(), x.max(y));
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).max_element(),
            x.max(y).max(z)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).max_element(),
            x.max(y).max(z).max(x)
        );

        assert_eq!(Vector::<2, T, A>::new(x, y).min_element(), x.min(y));
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).min_element(),
            x.min(y).min(z)
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).min_element(),
            x.min(y).min(z).min(x)
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).abs(),
            Vector::<2, T, A>::new(x.abs(), y.abs())
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).abs(),
            Vector::<3, T, A>::new(x.abs(), y.abs(), z.abs())
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).abs(),
            Vector::<4, T, A>::new(x.abs(), y.abs(), z.abs(), x.abs())
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).signum(),
            Vector::<2, T, A>::new(x.signum(), y.signum())
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).signum(),
            Vector::<3, T, A>::new(x.signum(), y.signum(), z.signum())
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).signum(),
            Vector::<4, T, A>::new(x.signum(), y.signum(), z.signum(), x.signum())
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).checked_add(Vector::<2, T, A>::new(z, x)),
            (|| Some(Vector::<2, T, A>::new(x.checked_add(z)?, y.checked_add(x)?)))()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).checked_add(Vector::<3, T, A>::new(z, x, y)),
            (|| Some(Vector::<3, T, A>::new(
                x.checked_add(z)?,
                y.checked_add(x)?,
                z.checked_add(y)?
            )))()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).checked_add(Vector::<4, T, A>::new(z, x, y, z)),
            (|| Some(Vector::<4, T, A>::new(
                x.checked_add(z)?,
                y.checked_add(x)?,
                z.checked_add(y)?,
                x.checked_add(z)?
            )))()
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).checked_sub(Vector::<2, T, A>::new(z, x)),
            (|| Some(Vector::<2, T, A>::new(x.checked_sub(z)?, y.checked_sub(x)?)))()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).checked_sub(Vector::<3, T, A>::new(z, x, y)),
            (|| Some(Vector::<3, T, A>::new(
                x.checked_sub(z)?,
                y.checked_sub(x)?,
                z.checked_sub(y)?
            )))()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).checked_sub(Vector::<4, T, A>::new(z, x, y, z)),
            (|| Some(Vector::<4, T, A>::new(
                x.checked_sub(z)?,
                y.checked_sub(x)?,
                z.checked_sub(y)?,
                x.checked_sub(z)?
            )))()
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).checked_mul(Vector::<2, T, A>::new(z, x)),
            (|| Some(Vector::<2, T, A>::new(x.checked_mul(z)?, y.checked_mul(x)?)))()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).checked_mul(Vector::<3, T, A>::new(z, x, y)),
            (|| Some(Vector::<3, T, A>::new(
                x.checked_mul(z)?,
                y.checked_mul(x)?,
                z.checked_mul(y)?
            )))()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).checked_mul(Vector::<4, T, A>::new(z, x, y, z)),
            (|| Some(Vector::<4, T, A>::new(
                x.checked_mul(z)?,
                y.checked_mul(x)?,
                z.checked_mul(y)?,
                x.checked_mul(z)?
            )))()
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).checked_div(Vector::<2, T, A>::new(z, x)),
            (|| Some(Vector::<2, T, A>::new(x.checked_div(z)?, y.checked_div(x)?)))()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).checked_div(Vector::<3, T, A>::new(z, x, y)),
            (|| Some(Vector::<3, T, A>::new(
                x.checked_div(z)?,
                y.checked_div(x)?,
                z.checked_div(y)?
            )))()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).checked_div(Vector::<4, T, A>::new(z, x, y, z)),
            (|| Some(Vector::<4, T, A>::new(
                x.checked_div(z)?,
                y.checked_div(x)?,
                z.checked_div(y)?,
                x.checked_div(z)?
            )))()
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).checked_rem(Vector::<2, T, A>::new(z, x)),
            (|| Some(Vector::<2, T, A>::new(x.checked_rem(z)?, y.checked_rem(x)?)))()
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).checked_rem(Vector::<3, T, A>::new(z, x, y)),
            (|| Some(Vector::<3, T, A>::new(
                x.checked_rem(z)?,
                y.checked_rem(x)?,
                z.checked_rem(y)?
            )))()
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).checked_rem(Vector::<4, T, A>::new(z, x, y, z)),
            (|| Some(Vector::<4, T, A>::new(
                x.checked_rem(z)?,
                y.checked_rem(x)?,
                z.checked_rem(y)?,
                x.checked_rem(z)?
            )))()
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).saturating_add(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.saturating_add(z), y.saturating_add(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).saturating_add(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(
                x.saturating_add(z),
                y.saturating_add(x),
                z.saturating_add(y)
            )
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).saturating_add(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.saturating_add(z),
                y.saturating_add(x),
                z.saturating_add(y),
                x.saturating_add(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).saturating_sub(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.saturating_sub(z), y.saturating_sub(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).saturating_sub(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(
                x.saturating_sub(z),
                y.saturating_sub(x),
                z.saturating_sub(y)
            )
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).saturating_sub(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.saturating_sub(z),
                y.saturating_sub(x),
                z.saturating_sub(y),
                x.saturating_sub(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).saturating_mul(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.saturating_mul(z), y.saturating_mul(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).saturating_mul(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(
                x.saturating_mul(z),
                y.saturating_mul(x),
                z.saturating_mul(y)
            )
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).saturating_mul(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.saturating_mul(z),
                y.saturating_mul(x),
                z.saturating_mul(y),
                x.saturating_mul(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).saturating_div(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.saturating_div(z), y.saturating_div(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).saturating_div(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(
                x.saturating_div(z),
                y.saturating_div(x),
                z.saturating_div(y)
            )
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).saturating_div(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.saturating_div(z),
                y.saturating_div(x),
                z.saturating_div(y),
                x.saturating_div(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).wrapping_add(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.wrapping_add(z), y.wrapping_add(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).wrapping_add(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.wrapping_add(z), y.wrapping_add(x), z.wrapping_add(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).wrapping_add(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.wrapping_add(z),
                y.wrapping_add(x),
                z.wrapping_add(y),
                x.wrapping_add(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).wrapping_sub(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.wrapping_sub(z), y.wrapping_sub(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).wrapping_sub(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.wrapping_sub(z), y.wrapping_sub(x), z.wrapping_sub(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).wrapping_sub(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.wrapping_sub(z),
                y.wrapping_sub(x),
                z.wrapping_sub(y),
                x.wrapping_sub(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).wrapping_mul(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.wrapping_mul(z), y.wrapping_mul(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).wrapping_mul(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.wrapping_mul(z), y.wrapping_mul(x), z.wrapping_mul(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).wrapping_mul(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.wrapping_mul(z),
                y.wrapping_mul(x),
                z.wrapping_mul(y),
                x.wrapping_mul(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).wrapping_div(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.wrapping_div(z), y.wrapping_div(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).wrapping_div(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.wrapping_div(z), y.wrapping_div(x), z.wrapping_div(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).wrapping_div(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.wrapping_div(z),
                y.wrapping_div(x),
                z.wrapping_div(y),
                x.wrapping_div(z)
            )
        );

        assert_eq!(
            Vector::<2, T, A>::new(x, y).wrapping_rem(Vector::<2, T, A>::new(z, x)),
            Vector::<2, T, A>::new(x.wrapping_rem(z), y.wrapping_rem(x))
        );
        assert_eq!(
            Vector::<3, T, A>::new(x, y, z).wrapping_rem(Vector::<3, T, A>::new(z, x, y)),
            Vector::<3, T, A>::new(x.wrapping_rem(z), y.wrapping_rem(x), z.wrapping_rem(y))
        );
        assert_eq!(
            Vector::<4, T, A>::new(x, y, z, x).wrapping_rem(Vector::<4, T, A>::new(z, x, y, z)),
            Vector::<4, T, A>::new(
                x.wrapping_rem(z),
                y.wrapping_rem(x),
                z.wrapping_rem(y),
                x.wrapping_rem(z)
            )
        );
    }
}
