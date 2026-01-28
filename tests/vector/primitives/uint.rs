use ggmath::Alignment;
use itertools::iproduct;

pub fn uint_tests<A: Alignment>() {
    const ARGS: [T; 9] = [0, 1, 2, 10, 100, 200, T::MAX / 2, T::MAX - 1, T::MAX];

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

        assert_eq!(!vec2!(x, y), vec2!(!x, !y));
        assert_eq!(!vec3!(x, y, z), vec3!(!x, !y, !z));
        assert_eq!(!vec4!(x, y, z, x), vec4!(!x, !y, !z, !x));

        assert_eq!(vec2!(x, y) + vec2!(z, x), vec2!(x + z, y + x));
        assert_eq!(vec3!(x, y, z) + vec3!(z, x, y), vec3!(x + z, y + x, z + y));
        assert_eq!(
            vec4!(x, y, z, x) + vec4!(z, x, y, z),
            vec4!(x + z, y + x, z + y, x + z)
        );

        assert_eq!(vec2!(x, y) - vec2!(z, x), vec2!(x - z, y - x));
        assert_eq!(vec3!(x, y, z) - vec3!(z, x, y), vec3!(x - z, y - x, z - y));
        assert_eq!(
            vec4!(x, y, z, x) - vec4!(z, x, y, z),
            vec4!(x - z, y - x, z - y, x - z)
        );

        assert_eq!(vec2!(x, y) * vec2!(z, x), vec2!(x * z, y * x));
        assert_eq!(vec3!(x, y, z) * vec3!(z, x, y), vec3!(x * z, y * x, z * y));
        assert_eq!(
            vec4!(x, y, z, x) * vec4!(z, x, y, z),
            vec4!(x * z, y * x, z * y, x * z)
        );

        assert_eq!(vec2!(x, y) / vec2!(z, x), vec2!(x / z, y / x));
        assert_eq!(vec3!(x, y, z) / vec3!(z, x, y), vec3!(x / z, y / x, z / y));
        assert_eq!(
            vec4!(x, y, z, x) / vec4!(z, x, y, z),
            vec4!(x / z, y / x, z / y, x / z)
        );

        assert_eq!(vec2!(x, y) % vec2!(z, x), vec2!(x % z, y % x));
        assert_eq!(vec3!(x, y, z) % vec3!(z, x, y), vec3!(x % z, y % x, z % y));
        assert_eq!(
            vec4!(x, y, z, x) % vec4!(z, x, y, z),
            vec4!(x % z, y % x, z % y, x % z)
        );

        assert_eq!(vec2!(x, y) << vec2!(z, x), vec2!(x << z, y << x));
        assert_eq!(
            vec3!(x, y, z) << vec3!(z, x, y),
            vec3!(x << z, y << x, z << y)
        );
        assert_eq!(
            vec4!(x, y, z, x) << vec4!(z, x, y, z),
            vec4!(x << z, y << x, z << y, x << z)
        );

        assert_eq!(vec2!(x, y) >> vec2!(z, x), vec2!(x >> z, y >> x));
        assert_eq!(
            vec3!(x, y, z) >> vec3!(z, x, y),
            vec3!(x >> z, y >> x, z >> y)
        );
        assert_eq!(
            vec4!(x, y, z, x) >> vec4!(z, x, y, z),
            vec4!(x >> z, y >> x, z >> y, x >> z)
        );

        assert_eq!(vec2!(x, y) & vec2!(z, x), vec2!(x & z, y & x));
        assert_eq!(vec3!(x, y, z) & vec3!(z, x, y), vec3!(x & z, y & x, z & y));
        assert_eq!(
            vec4!(x, y, z, x) & vec4!(z, x, y, z),
            vec4!(x & z, y & x, z & y, x & z)
        );

        assert_eq!(vec2!(x, y) | vec2!(z, x), vec2!(x | z, y | x));
        assert_eq!(vec3!(x, y, z) | vec3!(z, x, y), vec3!(x | z, y | x, z | y));
        assert_eq!(
            vec4!(x, y, z, x) | vec4!(z, x, y, z),
            vec4!(x | z, y | x, z | y, x | z)
        );

        assert_eq!(vec2!(x, y) ^ vec2!(z, x), vec2!(x ^ z, y ^ x));
        assert_eq!(vec3!(x, y, z) ^ vec3!(z, x, y), vec3!(x ^ z, y ^ x, z ^ y));
        assert_eq!(
            vec4!(x, y, z, x) ^ vec4!(z, x, y, z),
            vec4!(x ^ z, y ^ x, z ^ y, x ^ z)
        );

        assert_eq!(vec2!(x, y).max(vec2!(z, x)), vec2!(x.max(z), y.max(x)));
        assert_eq!(
            vec3!(x, y, z).max(vec3!(z, x, y)),
            vec3!(x.max(z), y.max(x), z.max(y))
        );
        assert_eq!(
            vec4!(x, y, z, x).max(vec4!(z, x, y, z)),
            vec4!(x.max(z), y.max(x), z.max(y), x.max(z))
        );

        assert_eq!(vec2!(x, y).min(vec2!(z, x)), vec2!(x.min(z), y.min(x)));
        assert_eq!(
            vec3!(x, y, z).min(vec3!(z, x, y)),
            vec3!(x.min(z), y.min(x), z.min(y))
        );
        assert_eq!(
            vec4!(x, y, z, x).min(vec4!(z, x, y, z)),
            vec4!(x.min(z), y.min(x), z.min(y), x.min(z))
        );

        assert_eq!(
            vec2!(x, y).clamp(vec2!(z, x), vec2!(y, z)),
            vec2!(x.clamp(z, y), y.clamp(x, z))
        );
        assert_eq!(
            vec3!(x, y, z).clamp(vec3!(z, x, y), vec3!(y, z, x)),
            vec3!(x.clamp(z, y), y.clamp(x, z), z.clamp(y, x))
        );
        assert_eq!(
            vec4!(x, y, z, x).clamp(vec4!(z, x, y, z), vec4!(y, z, x, y)),
            vec4!(x.clamp(z, y), y.clamp(x, z), z.clamp(y, x), x.clamp(z, y))
        );

        assert_eq!(
            vec2!(x, y).checked_add(vec2!(z, x)),
            (|| Some(vec2!(x.checked_add(z)?, y.checked_add(x)?)))()
        );
        assert_eq!(
            vec3!(x, y, z).checked_add(vec3!(z, x, y)),
            (|| Some(vec3!(
                x.checked_add(z)?,
                y.checked_add(x)?,
                z.checked_add(y)?
            )))()
        );
        assert_eq!(
            vec4!(x, y, z, x).checked_add(vec4!(z, x, y, z)),
            (|| Some(vec4!(
                x.checked_add(z)?,
                y.checked_add(x)?,
                z.checked_add(y)?,
                x.checked_add(z)?
            )))()
        );

        assert_eq!(
            vec2!(x, y).checked_sub(vec2!(z, x)),
            (|| Some(vec2!(x.checked_sub(z)?, y.checked_sub(x)?)))()
        );
        assert_eq!(
            vec3!(x, y, z).checked_sub(vec3!(z, x, y)),
            (|| Some(vec3!(
                x.checked_sub(z)?,
                y.checked_sub(x)?,
                z.checked_sub(y)?
            )))()
        );
        assert_eq!(
            vec4!(x, y, z, x).checked_sub(vec4!(z, x, y, z)),
            (|| Some(vec4!(
                x.checked_sub(z)?,
                y.checked_sub(x)?,
                z.checked_sub(y)?,
                x.checked_sub(z)?
            )))()
        );

        assert_eq!(
            vec2!(x, y).checked_mul(vec2!(z, x)),
            (|| Some(vec2!(x.checked_mul(z)?, y.checked_mul(x)?)))()
        );
        assert_eq!(
            vec3!(x, y, z).checked_mul(vec3!(z, x, y)),
            (|| Some(vec3!(
                x.checked_mul(z)?,
                y.checked_mul(x)?,
                z.checked_mul(y)?
            )))()
        );
        assert_eq!(
            vec4!(x, y, z, x).checked_mul(vec4!(z, x, y, z)),
            (|| Some(vec4!(
                x.checked_mul(z)?,
                y.checked_mul(x)?,
                z.checked_mul(y)?,
                x.checked_mul(z)?
            )))()
        );

        assert_eq!(
            vec2!(x, y).checked_div(vec2!(z, x)),
            (|| Some(vec2!(x.checked_div(z)?, y.checked_div(x)?)))()
        );
        assert_eq!(
            vec3!(x, y, z).checked_div(vec3!(z, x, y)),
            (|| Some(vec3!(
                x.checked_div(z)?,
                y.checked_div(x)?,
                z.checked_div(y)?
            )))()
        );
        assert_eq!(
            vec4!(x, y, z, x).checked_div(vec4!(z, x, y, z)),
            (|| Some(vec4!(
                x.checked_div(z)?,
                y.checked_div(x)?,
                z.checked_div(y)?,
                x.checked_div(z)?
            )))()
        );

        assert_eq!(
            vec2!(x, y).checked_rem(vec2!(z, x)),
            (|| Some(vec2!(x.checked_rem(z)?, y.checked_rem(x)?)))()
        );
        assert_eq!(
            vec3!(x, y, z).checked_rem(vec3!(z, x, y)),
            (|| Some(vec3!(
                x.checked_rem(z)?,
                y.checked_rem(x)?,
                z.checked_rem(y)?
            )))()
        );
        assert_eq!(
            vec4!(x, y, z, x).checked_rem(vec4!(z, x, y, z)),
            (|| Some(vec4!(
                x.checked_rem(z)?,
                y.checked_rem(x)?,
                z.checked_rem(y)?,
                x.checked_rem(z)?
            )))()
        );

        assert_eq!(
            vec2!(x, y).saturating_add(vec2!(z, x)),
            vec2!(x.saturating_add(z), y.saturating_add(x))
        );
        assert_eq!(
            vec3!(x, y, z).saturating_add(vec3!(z, x, y)),
            vec3!(
                x.saturating_add(z),
                y.saturating_add(x),
                z.saturating_add(y)
            )
        );
        assert_eq!(
            vec4!(x, y, z, x).saturating_add(vec4!(z, x, y, z)),
            vec4!(
                x.saturating_add(z),
                y.saturating_add(x),
                z.saturating_add(y),
                x.saturating_add(z)
            )
        );

        assert_eq!(
            vec2!(x, y).saturating_sub(vec2!(z, x)),
            vec2!(x.saturating_sub(z), y.saturating_sub(x))
        );
        assert_eq!(
            vec3!(x, y, z).saturating_sub(vec3!(z, x, y)),
            vec3!(
                x.saturating_sub(z),
                y.saturating_sub(x),
                z.saturating_sub(y)
            )
        );
        assert_eq!(
            vec4!(x, y, z, x).saturating_sub(vec4!(z, x, y, z)),
            vec4!(
                x.saturating_sub(z),
                y.saturating_sub(x),
                z.saturating_sub(y),
                x.saturating_sub(z)
            )
        );

        assert_eq!(
            vec2!(x, y).saturating_mul(vec2!(z, x)),
            vec2!(x.saturating_mul(z), y.saturating_mul(x))
        );
        assert_eq!(
            vec3!(x, y, z).saturating_mul(vec3!(z, x, y)),
            vec3!(
                x.saturating_mul(z),
                y.saturating_mul(x),
                z.saturating_mul(y)
            )
        );
        assert_eq!(
            vec4!(x, y, z, x).saturating_mul(vec4!(z, x, y, z)),
            vec4!(
                x.saturating_mul(z),
                y.saturating_mul(x),
                z.saturating_mul(y),
                x.saturating_mul(z)
            )
        );

        assert_eq!(
            vec2!(x, y).wrapping_add(vec2!(z, x)),
            vec2!(x.wrapping_add(z), y.wrapping_add(x))
        );
        assert_eq!(
            vec3!(x, y, z).wrapping_add(vec3!(z, x, y)),
            vec3!(x.wrapping_add(z), y.wrapping_add(x), z.wrapping_add(y))
        );
        assert_eq!(
            vec4!(x, y, z, x).wrapping_add(vec4!(z, x, y, z)),
            vec4!(
                x.wrapping_add(z),
                y.wrapping_add(x),
                z.wrapping_add(y),
                x.wrapping_add(z)
            )
        );

        assert_eq!(
            vec2!(x, y).wrapping_sub(vec2!(z, x)),
            vec2!(x.wrapping_sub(z), y.wrapping_sub(x))
        );
        assert_eq!(
            vec3!(x, y, z).wrapping_sub(vec3!(z, x, y)),
            vec3!(x.wrapping_sub(z), y.wrapping_sub(x), z.wrapping_sub(y))
        );
        assert_eq!(
            vec4!(x, y, z, x).wrapping_sub(vec4!(z, x, y, z)),
            vec4!(
                x.wrapping_sub(z),
                y.wrapping_sub(x),
                z.wrapping_sub(y),
                x.wrapping_sub(z)
            )
        );

        assert_eq!(
            vec2!(x, y).wrapping_mul(vec2!(z, x)),
            vec2!(x.wrapping_mul(z), y.wrapping_mul(x))
        );
        assert_eq!(
            vec3!(x, y, z).wrapping_mul(vec3!(z, x, y)),
            vec3!(x.wrapping_mul(z), y.wrapping_mul(x), z.wrapping_mul(y))
        );
        assert_eq!(
            vec4!(x, y, z, x).wrapping_mul(vec4!(z, x, y, z)),
            vec4!(
                x.wrapping_mul(z),
                y.wrapping_mul(x),
                z.wrapping_mul(y),
                x.wrapping_mul(z)
            )
        );
    }
}
