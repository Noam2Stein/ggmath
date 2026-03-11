use ggmath::{Affine, Alignment, Matrix, Vector};
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
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ) == Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan()
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, x),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ) == Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, x, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, x, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(x, x)
            ) == Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(x, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(x, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, x),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(x, x)
            ) == Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, x, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(x, y, x)
            ) == Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y && x == z
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, x, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(x, y, x, z)
            ) == Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            !x.is_nan() && !y.is_nan() && !z.is_nan() && x == y && x == z
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ) != Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan()
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan()
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan()
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, x),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ) != Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, x, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, x, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(x, x)
            ) != Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(x, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(x, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );

        assert_eq!(
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, x),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(x, x)
            ) != Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, x)
                ]),
                Vector::<2, T, A>::new(y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y
        );
        assert_eq!(
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, x, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(x, y, x)
            ) != Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, x, y),
                    Vector::<3, T, A>::new(y, x, y)
                ]),
                Vector::<3, T, A>::new(z, y, x)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y || x != z
        );
        assert_eq!(
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, x, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(x, y, x, z)
            ) != Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(z, x, y, x),
                    Vector::<4, T, A>::new(y, x, y, z),
                    Vector::<4, T, A>::new(y, y, x, y)
                ]),
                Vector::<4, T, A>::new(z, y, x, z)
            ),
            x.is_nan() || y.is_nan() || z.is_nan() || x != y || x != z
        );

        assert_float_eq!(
            Affine::<2, T, A>::from_mat(Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z)
            ])),
            Affine::<2, T, A>::from_mat_translation(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(y, z)
                ]),
                Vector::ZERO
            )
        );
        assert_float_eq!(
            Affine::<3, T, A>::from_mat(Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ])),
            Affine::<3, T, A>::from_mat_translation(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(y, z, x),
                    Vector::<3, T, A>::new(z, x, y)
                ]),
                Vector::ZERO
            )
        );
        assert_float_eq!(
            Affine::<4, T, A>::from_mat(Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z)
            ])),
            Affine::<4, T, A>::from_mat_translation(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, x),
                    Vector::<4, T, A>::new(y, z, x, y),
                    Vector::<4, T, A>::new(z, x, y, z),
                    Vector::<4, T, A>::new(z, y, x, z)
                ]),
                Vector::ZERO
            )
        );

        assert_float_eq!(
            Affine::<2, T, A>::from_translation(Vector::<2, T, A>::new(z, x)),
            Affine::<2, T, A>::from_mat_translation(Matrix::IDENTITY, Vector::<2, T, A>::new(z, x))
        );
        assert_float_eq!(
            Affine::<3, T, A>::from_translation(Vector::<3, T, A>::new(z, y, x)),
            Affine::<3, T, A>::from_mat_translation(
                Matrix::IDENTITY,
                Vector::<3, T, A>::new(z, y, x)
            )
        );
        assert_float_eq!(
            Affine::<4, T, A>::from_translation(Vector::<4, T, A>::new(z, z, y, x)),
            Affine::<4, T, A>::from_mat_translation(
                Matrix::IDENTITY,
                Vector::<4, T, A>::new(z, z, y, x)
            )
        );
    }
}
