use ggmath::{Affine, Alignment, Matrix, Vector, mat2, mat3, mat4, vec2, vec3, vec4};
use itertools::iproduct;

use crate::assert_float_eq;

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
        Affine::<2, T, A>::ZERO,
        Affine::<2, T, A>::from_mat_translation(Matrix::ZERO, Vector::ZERO)
    );
    assert_float_eq!(
        Affine::<3, T, A>::ZERO,
        Affine::<3, T, A>::from_mat_translation(Matrix::ZERO, Vector::ZERO)
    );
    assert_float_eq!(
        Affine::<4, T, A>::ZERO,
        Affine::<4, T, A>::from_mat_translation(Matrix::ZERO, Vector::ZERO)
    );

    assert_float_eq!(
        Affine::<2, T, A>::IDENTITY,
        Affine::<2, T, A>::from_mat_translation(Matrix::IDENTITY, Vector::ZERO)
    );
    assert_float_eq!(
        Affine::<3, T, A>::IDENTITY,
        Affine::<3, T, A>::from_mat_translation(Matrix::IDENTITY, Vector::ZERO)
    );
    assert_float_eq!(
        Affine::<4, T, A>::IDENTITY,
        Affine::<4, T, A>::from_mat_translation(Matrix::IDENTITY, Vector::ZERO)
    );

    assert_float_eq!(
        Affine::<2, T, A>::NAN,
        Affine::<2, T, A>::from_mat_translation(Matrix::NAN, Vector::NAN)
    );
    assert_float_eq!(
        Affine::<3, T, A>::NAN,
        Affine::<3, T, A>::from_mat_translation(Matrix::NAN, Vector::NAN)
    );
    assert_float_eq!(
        Affine::<4, T, A>::NAN,
        Affine::<4, T, A>::from_mat_translation(Matrix::NAN, Vector::NAN)
    );

    for (x, y, z) in iproduct!(ARGS, ARGS, ARGS) {
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

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(z, x)), vec2!(y, x))
                == Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, x), vec2!(z, x)), vec2!(y, x))
                == Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, x, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, x, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(z, x)), vec2!(x, x))
                == Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(x, x, y)),
                vec3!(z, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(x, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, x), vec2!(z, x)), vec2!(x, x))
                == Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, x, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(x, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y && x == z
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, x, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(x, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y && x == z
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(z, x)), vec2!(y, x))
                != Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            x.is_nan() || y.is_nan() || z.is_nan()
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan()
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan()
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, x), vec2!(z, x)), vec2!(y, x))
                != Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, x, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, x, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(z, x)), vec2!(x, x))
                != Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(x, x, y)),
                vec3!(z, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(x, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, x), vec2!(z, x)), vec2!(x, x))
                != Affine::<2, T, A>::from_mat_translation(
                    mat2!(vec2!(x, y), vec2!(z, x)),
                    vec2!(y, x)
                ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, x, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(x, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(z, x, y), vec3!(y, x, y)),
                vec3!(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y || x != z
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, x, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(x, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(z, x, y, x),
                    vec4!(y, x, y, z),
                    vec4!(y, y, x, y)
                ),
                vec4!(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y || x != z
        );

        assert_float_eq!(
            Affine::<2, T, A>::from_mat(mat2!(vec2!(x, y), vec2!(y, z))),
            Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(0.0))
        );
        assert_float_eq!(
            Affine::<3, T, A>::from_mat(mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))),
            Affine::<3, T, A>::from_mat_translation(
                mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
                vec3!(0.0)
            )
        );
        assert_float_eq!(
            Affine::<4, T, A>::from_mat(mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z)
            )),
            Affine::<4, T, A>::from_mat_translation(
                mat4!(
                    vec4!(x, y, z, x),
                    vec4!(y, z, x, y),
                    vec4!(z, x, y, z),
                    vec4!(z, y, x, z)
                ),
                vec4!(0.0)
            )
        );

        assert_float_eq!(
            Affine::<2, T, A>::from_translation(vec2!(z, x)),
            Affine::<2, T, A>::from_mat_translation(Matrix::IDENTITY, vec2!(z, x))
        );
        assert_float_eq!(
            Affine::<3, T, A>::from_translation(vec3!(z, y, x)),
            Affine::<3, T, A>::from_mat_translation(Matrix::IDENTITY, vec3!(z, y, x))
        );
        assert_float_eq!(
            Affine::<4, T, A>::from_translation(vec4!(z, z, y, x)),
            Affine::<4, T, A>::from_mat_translation(Matrix::IDENTITY, vec4!(z, z, y, x))
        );
    }
}
