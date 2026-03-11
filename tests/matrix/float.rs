use ggmath::{Alignment, Matrix, Vector};
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
        Matrix::<2, T, A>::ZERO,
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(0.0, 0.0),
            Vector::<2, T, A>::new(0.0, 0.0)
        ])
    );
    assert_float_eq!(
        Matrix::<3, T, A>::ZERO,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(0.0, 0.0, 0.0),
            Vector::<3, T, A>::new(0.0, 0.0, 0.0),
            Vector::<3, T, A>::new(0.0, 0.0, 0.0)
        ])
    );
    assert_float_eq!(
        Matrix::<4, T, A>::ZERO,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(0.0, 0.0, 0.0, 0.0),
            Vector::<4, T, A>::new(0.0, 0.0, 0.0, 0.0),
            Vector::<4, T, A>::new(0.0, 0.0, 0.0, 0.0),
            Vector::<4, T, A>::new(0.0, 0.0, 0.0, 0.0)
        ])
    );

    assert_float_eq!(
        Matrix::<2, T, A>::IDENTITY,
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(1.0, 0.0),
            Vector::<2, T, A>::new(0.0, 1.0)
        ])
    );
    assert_float_eq!(
        Matrix::<3, T, A>::IDENTITY,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(1.0, 0.0, 0.0),
            Vector::<3, T, A>::new(0.0, 1.0, 0.0),
            Vector::<3, T, A>::new(0.0, 0.0, 1.0)
        ])
    );
    assert_float_eq!(
        Matrix::<4, T, A>::IDENTITY,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(1.0, 0.0, 0.0, 0.0),
            Vector::<4, T, A>::new(0.0, 1.0, 0.0, 0.0),
            Vector::<4, T, A>::new(0.0, 0.0, 1.0, 0.0),
            Vector::<4, T, A>::new(0.0, 0.0, 0.0, 1.0)
        ])
    );

    assert_float_eq!(
        Matrix::<2, T, A>::NAN,
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(T::NAN, T::NAN),
            Vector::<2, T, A>::new(T::NAN, T::NAN)
        ])
    );
    assert_float_eq!(
        Matrix::<3, T, A>::NAN,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(T::NAN, T::NAN, T::NAN),
            Vector::<3, T, A>::new(T::NAN, T::NAN, T::NAN),
            Vector::<3, T, A>::new(T::NAN, T::NAN, T::NAN)
        ])
    );
    assert_float_eq!(
        Matrix::<4, T, A>::NAN,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(T::NAN, T::NAN, T::NAN, T::NAN),
            Vector::<4, T, A>::new(T::NAN, T::NAN, T::NAN, T::NAN),
            Vector::<4, T, A>::new(T::NAN, T::NAN, T::NAN, T::NAN),
            Vector::<4, T, A>::new(T::NAN, T::NAN, T::NAN, T::NAN)
        ])
    );

    for (x, y, z) in iproduct!(ARGS, ARGS, ARGS) {
        macro_rules! assert_float_eq {
            ($($arg:tt)*) => {
                $crate::assert_float_eq!($($arg)+, "x = {x:?}, y = {y:?}, z = {z:?}");
            };
        }

        assert_float_eq!(
            Matrix::<2, T, A>::from_diagonal(Vector::<2, T, A>::new(x, y)),
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, 0.0),
                Vector::<2, T, A>::new(0.0, y)
            ])
        );
        assert_float_eq!(
            Matrix::<3, T, A>::from_diagonal(Vector::<3, T, A>::new(x, y, z)),
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, 0.0, 0.0),
                Vector::<3, T, A>::new(0.0, y, 0.0),
                Vector::<3, T, A>::new(0.0, 0.0, z)
            ])
        );
        assert_float_eq!(
            Matrix::<4, T, A>::from_diagonal(Vector::<4, T, A>::new(x, y, z, x)),
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, 0.0, 0.0, 0.0),
                Vector::<4, T, A>::new(0.0, y, 0.0, 0.0),
                Vector::<4, T, A>::new(0.0, 0.0, z, 0.0),
                Vector::<4, T, A>::new(0.0, 0.0, 0.0, x)
            ])
        );

        assert_float_eq!(
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, x + 1.0)
            ])
            .diagonal(),
            Vector::<2, T, A>::new(x, x + 1.0)
        );
        assert_float_eq!(
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, x + 1.0, y * 2.0),
                Vector::<3, T, A>::new(y, x + x, y * y)
            ])
            .diagonal(),
            Vector::<3, T, A>::new(x, x + 1.0, y * y)
        );
        assert_float_eq!(
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x + 1.5),
                Vector::<4, T, A>::new(z, x + 1.0, y * 2.0, z / 7.0),
                Vector::<4, T, A>::new(y, x + x, y * y, x + 1.3),
                Vector::<4, T, A>::new(x.sqrt(), x * 1.2, y * x, x + 1.5),
            ])
            .diagonal(),
            Vector::<4, T, A>::new(x, x + 1.0, y * y, x + 1.5)
        );
    }
}
