use std::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use ggmath::{Affine, Alignment, Matrix, Scalar, Vector};

pub fn test<T, A: Alignment>(x: T, y: T, z: T, w: T)
where
    T: Scalar + Debug + Display + PartialEq + PartialOrd + Default + UnwindSafe + RefUnwindSafe,
{
    assert!(y > x && w > z);

    assert!(
        size_of::<Affine<2, T, A>>() == size_of::<Matrix<2, T, A>>() + size_of::<Vector<2, T, A>>()
            || size_of::<Affine<2, T, A>>() == size_of::<Vector<2, T, A>>() * 4
    );
    assert!(
        size_of::<Affine<3, T, A>>() == size_of::<Matrix<3, T, A>>() + size_of::<Vector<3, T, A>>()
            || size_of::<Matrix<3, T, A>>() == size_of::<[T; 9]>()
                && size_of::<Vector<3, T, A>>() == size_of::<[T; 3]>()
                && size_of::<Affine<3, T, A>>() == size_of::<[T; 16]>()
    );
    assert!(
        size_of::<Affine<4, T, A>>() == size_of::<Matrix<4, T, A>>() + size_of::<Vector<4, T, A>>()
    );

    assert!(
        align_of::<Affine<2, T, A>>() == align_of::<Matrix<2, T, A>>()
            || align_of::<Affine<2, T, A>>() == size_of::<Vector<2, T, A>>() * 4
    );
    assert!(
        align_of::<Affine<3, T, A>>() == align_of::<Matrix<3, T, A>>()
            || size_of::<Matrix<3, T, A>>() == size_of::<[T; 9]>()
                && size_of::<Vector<3, T, A>>() == size_of::<[T; 3]>()
                && align_of::<Affine<3, T, A>>() == size_of::<[T; 16]>()
    );
    assert!(align_of::<Affine<4, T, A>>() == align_of::<Matrix<4, T, A>>());

    assert_eq!(
        Affine::<2, T, A>::from_cols(
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        )
        .to_col_array(),
        [
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        ]
    );
    assert_eq!(
        Affine::<3, T, A>::from_cols(
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        )
        .to_col_array(),
        [
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_cols(
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        )
        .to_col_array(),
        [
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_col_array(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        ])
        .to_col_array(),
        [
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        ]
    );
    assert_eq!(
        Affine::<3, T, A>::from_col_array(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        ])
        .to_col_array(),
        [
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_col_array(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ])
        .to_col_array(),
        [
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_mat_translation(
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z)
            ]),
            Vector::<2, T, A>::new(z, x)
        )
        .to_col_array(),
        [
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        ]
    );
    assert_eq!(
        Affine::<3, T, A>::from_mat_translation(
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ]),
            Vector::<3, T, A>::new(z, y, x)
        )
        .to_col_array(),
        [
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_mat_translation(
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z)
            ]),
            Vector::<4, T, A>::new(z, z, y, x)
        )
        .to_col_array(),
        [
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_mat_translation(
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z)
            ]),
            Vector::<2, T, A>::new(z, x)
        )
        .matrix,
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z)
        ])
    );
    assert_eq!(
        Affine::<3, T, A>::from_mat_translation(
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ]),
            Vector::<3, T, A>::new(z, y, x)
        )
        .matrix,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
    );
    assert_eq!(
        Affine::<4, T, A>::from_mat_translation(
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z)
            ]),
            Vector::<4, T, A>::new(z, z, y, x)
        )
        .matrix,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z)
        ])
    );

    assert_eq!(
        Affine::<2, T, A>::from_mat_translation(
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z)
            ]),
            Vector::<2, T, A>::new(z, x)
        )
        .translation,
        Vector::<2, T, A>::new(z, x)
    );
    assert_eq!(
        Affine::<3, T, A>::from_mat_translation(
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ]),
            Vector::<3, T, A>::new(z, y, x)
        )
        .translation,
        Vector::<3, T, A>::new(z, y, x)
    );
    assert_eq!(
        Affine::<4, T, A>::from_mat_translation(
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z)
            ]),
            Vector::<4, T, A>::new(z, z, y, x)
        )
        .translation,
        Vector::<4, T, A>::new(z, z, y, x)
    );

    let mut a = Affine::<2, T, A>::from_mat_translation(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
        ]),
        Vector::<2, T, A>::new(z, x),
    );
    a.matrix = Matrix::<2, T, A>::from_columns(&[
        Vector::<2, T, A>::new(x, x),
        Vector::<2, T, A>::new(x, z),
    ]);
    assert_eq!(
        a,
        Affine::<2, T, A>::from_mat_translation(
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, x),
                Vector::<2, T, A>::new(x, z)
            ]),
            Vector::<2, T, A>::new(z, x)
        )
    );

    let mut a = Affine::<3, T, A>::from_mat_translation(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
        ]),
        Vector::<3, T, A>::new(z, y, x),
    );
    a.matrix = Matrix::<3, T, A>::from_columns(&[
        Vector::<3, T, A>::new(x, y, x),
        Vector::<3, T, A>::new(y, y, x),
        Vector::<3, T, A>::new(y, x, y),
    ]);
    assert_eq!(
        a,
        Affine::<3, T, A>::from_mat_translation(
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, x),
                Vector::<3, T, A>::new(y, y, x),
                Vector::<3, T, A>::new(y, x, y)
            ]),
            Vector::<3, T, A>::new(z, y, x),
        )
    );

    let mut a = Affine::<4, T, A>::from_mat_translation(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
        ]),
        Vector::<4, T, A>::new(z, z, y, x),
    );
    a.matrix = Matrix::<4, T, A>::from_columns(&[
        Vector::<4, T, A>::new(x, y, x, x),
        Vector::<4, T, A>::new(y, x, x, y),
        Vector::<4, T, A>::new(z, x, y, y),
        Vector::<4, T, A>::new(z, x, x, z),
    ]);
    assert_eq!(
        a,
        Affine::<4, T, A>::from_mat_translation(
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, x, x),
                Vector::<4, T, A>::new(y, x, x, y),
                Vector::<4, T, A>::new(z, x, y, y),
                Vector::<4, T, A>::new(z, x, x, z)
            ]),
            Vector::<4, T, A>::new(z, z, y, x),
        )
    );

    let mut a = Affine::<2, T, A>::from_mat_translation(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
        ]),
        Vector::<2, T, A>::new(z, x),
    );
    a.translation = Vector::<2, T, A>::new(y, y);
    assert_eq!(
        a,
        Affine::<2, T, A>::from_mat_translation(
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z)
            ]),
            Vector::<2, T, A>::new(y, y)
        )
    );

    let mut a = Affine::<3, T, A>::from_mat_translation(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
        ]),
        Vector::<3, T, A>::new(z, y, x),
    );
    a.translation = Vector::<3, T, A>::new(y, x, y);
    assert_eq!(
        a,
        Affine::<3, T, A>::from_mat_translation(
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ]),
            Vector::<3, T, A>::new(y, x, y)
        )
    );

    let mut a = Affine::<4, T, A>::from_mat_translation(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
        ]),
        Vector::<4, T, A>::new(z, z, y, x),
    );
    a.translation = Vector::<4, T, A>::new(y, y, z, y);
    assert_eq!(
        a,
        Affine::<4, T, A>::from_mat_translation(
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z)
            ]),
            Vector::<4, T, A>::new(y, y, z, y)
        )
    );

    assert_eq!(
        Affine::<2, T, A>::from_cols(
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        )
        .as_col_array_ref(),
        &[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        ]
    );
    assert_eq!(
        Affine::<3, T, A>::from_cols(
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        )
        .as_col_array_ref(),
        &[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_cols(
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        )
        .as_col_array_ref(),
        &[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_cols(
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        )
        .as_col_array_mut(),
        &mut [
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(z, x)
        ]
    );
    assert_eq!(
        Affine::<3, T, A>::from_cols(
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        )
        .as_col_array_mut(),
        &mut [
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y),
            Vector::<3, T, A>::new(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_cols(
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        )
        .as_col_array_mut(),
        &mut [
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(z, y, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ]
    );

    assert_eq!(
        format!(
            "{:?}",
            Affine::<2, T, A>::from_cols(
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z),
                Vector::<2, T, A>::new(z, x)
            )
        ),
        format!("[({x:?}, {y:?}), ({y:?}, {z:?}), ({z:?}, {x:?})]")
    );
    assert_eq!(
        format!(
            "{:?}",
            Affine::<3, T, A>::from_cols(
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y),
                Vector::<3, T, A>::new(z, y, x)
            )
        ),
        format!(
            "[({x:?}, {y:?}, {z:?}), ({y:?}, {z:?}, {x:?}), ({z:?}, {x:?}, {y:?}), ({z:?}, {y:?}, {x:?})]"
        )
    );
    assert_eq!(
        format!(
            "{:?}",
            Affine::<4, T, A>::from_cols(
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z),
                Vector::<4, T, A>::new(z, z, y, x)
            )
        ),
        format!(
            "[({x:?}, {y:?}, {z:?}, {x:?}), ({y:?}, {z:?}, {x:?}, {y:?}), ({z:?}, {x:?}, {y:?}, {z:?}), ({z:?}, {y:?}, {x:?}, {z:?}), ({z:?}, {z:?}, {y:?}, {x:?})]"
        )
    );

    assert_eq!(
        format!(
            "{}",
            Affine::<2, T, A>::from_cols(
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(y, z),
                Vector::<2, T, A>::new(z, x)
            )
        ),
        format!("[({x}, {y}), ({y}, {z}), ({z}, {x})]")
    );
    assert_eq!(
        format!(
            "{}",
            Affine::<3, T, A>::from_cols(
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y),
                Vector::<3, T, A>::new(z, y, x)
            )
        ),
        format!("[({x}, {y}, {z}), ({y}, {z}, {x}), ({z}, {x}, {y}), ({z}, {y}, {x})]")
    );
    assert_eq!(
        format!(
            "{}",
            Affine::<4, T, A>::from_cols(
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(z, y, x, z),
                Vector::<4, T, A>::new(z, z, y, x)
            )
        ),
        format!(
            "[({x}, {y}, {z}, {x}), ({y}, {z}, {x}, {y}), ({z}, {x}, {y}, {z}), ({z}, {y}, {x}, {z}), ({z}, {z}, {y}, {x})]"
        )
    );
}
