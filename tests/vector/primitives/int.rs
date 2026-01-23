use ggmath::Alignment;
use itertools::iproduct;

pub fn int_tests<A: Alignment>() {
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

        assert_eq!(-vec2!(x, y), vec2!(-x, -y));
        assert_eq!(-vec3!(x, y, z), vec3!(-x, -y, -z));
        assert_eq!(-vec4!(x, y, z, x), vec4!(-x, -y, -z, -x));

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
    }
}
