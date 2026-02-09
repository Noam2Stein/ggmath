use ggmath::{Alignment, Matrix, mat2, mat3, mat4, vec2, vec3, vec4};
use itertools::iproduct;

use crate::{assert_float_eq, utils::vec3_with_padding};

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

    assert_float_eq!(
        Matrix::<2, T, A>::ZERO,
        mat2!(vec2!(0.0, 0.0), vec2!(0.0, 0.0))
    );
    assert_float_eq!(
        Matrix::<3, T, A>::ZERO,
        mat3!(
            vec3!(0.0, 0.0, 0.0),
            vec3!(0.0, 0.0, 0.0),
            vec3!(0.0, 0.0, 0.0)
        )
    );
    assert_float_eq!(
        Matrix::<4, T, A>::ZERO,
        mat4!(
            vec4!(0.0, 0.0, 0.0, 0.0),
            vec4!(0.0, 0.0, 0.0, 0.0),
            vec4!(0.0, 0.0, 0.0, 0.0),
            vec4!(0.0, 0.0, 0.0, 0.0)
        )
    );

    assert_float_eq!(
        Matrix::<2, T, A>::IDENTITY,
        mat2!(vec2!(1.0, 0.0), vec2!(0.0, 1.0))
    );
    assert_float_eq!(
        Matrix::<3, T, A>::IDENTITY,
        mat3!(
            vec3!(1.0, 0.0, 0.0),
            vec3!(0.0, 1.0, 0.0),
            vec3!(0.0, 0.0, 1.0)
        )
    );
    assert_float_eq!(
        Matrix::<4, T, A>::IDENTITY,
        mat4!(
            vec4!(1.0, 0.0, 0.0, 0.0),
            vec4!(0.0, 1.0, 0.0, 0.0),
            vec4!(0.0, 0.0, 1.0, 0.0),
            vec4!(0.0, 0.0, 0.0, 1.0)
        )
    );

    assert_float_eq!(
        Matrix::<2, T, A>::NAN,
        mat2!(vec2!(T::NAN, T::NAN), vec2!(T::NAN, T::NAN))
    );
    assert_float_eq!(
        Matrix::<3, T, A>::NAN,
        mat3!(
            vec3!(T::NAN, T::NAN, T::NAN),
            vec3!(T::NAN, T::NAN, T::NAN),
            vec3!(T::NAN, T::NAN, T::NAN)
        )
    );
    assert_float_eq!(
        Matrix::<4, T, A>::NAN,
        mat4!(
            vec4!(T::NAN, T::NAN, T::NAN, T::NAN),
            vec4!(T::NAN, T::NAN, T::NAN, T::NAN),
            vec4!(T::NAN, T::NAN, T::NAN, T::NAN),
            vec4!(T::NAN, T::NAN, T::NAN, T::NAN)
        )
    );

    for (x, y, z) in iproduct!(ARGS, ARGS, ARGS) {
        macro_rules! assert_float_eq {
            ($($arg:tt)*) => {
                $crate::assert_float_eq!($($arg)+, "x = {x:?}, y = {y:?}, z = {z:?}");
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

        macro_rules! mat2 {
            ($($arg:expr),*$(,)?) => {
                ggmath::Matrix::<2, T, A>::from(($($arg,)*))
            };
        }

        macro_rules! mat3 {
            ($($arg:expr),*$(,)?) => {
                ggmath::Matrix::<3, T, A>::from(($($arg,)*))
            };
        }

        macro_rules! mat4 {
            ($($arg:expr),*$(,)?) => {
                ggmath::Matrix::<4, T, A>::from(($($arg,)*))
            };
        }

        assert_float_eq!(
            Matrix::<2, T, A>::from_diagonal(vec2!(x, y)),
            mat2!(vec2!(x, 0.0), vec2!(0.0, y))
        );
        assert_float_eq!(
            Matrix::<3, T, A>::from_diagonal(vec3!(x, y, z)),
            mat3!(vec3!(x, 0.0, 0.0), vec3!(0.0, y, 0.0), vec3!(0.0, 0.0, z))
        );
        assert_float_eq!(
            Matrix::<4, T, A>::from_diagonal(vec4!(x, y, z, x)),
            mat4!(
                vec4!(x, 0.0, 0.0, 0.0),
                vec4!(0.0, y, 0.0, 0.0),
                vec4!(0.0, 0.0, z, 0.0),
                vec4!(0.0, 0.0, 0.0, x)
            )
        );

        assert_float_eq!(
            mat2!(vec2!(x, y), vec2!(z, x + 1.0)).diagonal(),
            vec2!(x, x + 1.0)
        );
        assert_float_eq!(
            mat3!(
                vec3!(x, y, z),
                vec3!(z, x + 1.0, y * 2.0),
                vec3!(y, x + x, y * y)
            )
            .diagonal(),
            vec3!(x, x + 1.0, y * y)
        );
        assert_float_eq!(
            mat4!(
                vec4!(x, y, z, x + 1.5),
                vec4!(z, x + 1.0, y * 2.0, z / 7.0),
                vec4!(y, x + x, y * y, x + 1.3),
                vec4!(x.sqrt(), x * 1.2, y * x, x + 1.5),
            )
            .diagonal(),
            vec4!(x, x + 1.0, y * y, x + 1.5)
        );
    }
}
