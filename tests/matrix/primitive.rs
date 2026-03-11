use std::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use ggmath::{Alignment, Matrix, Scalar, Vector};

use crate::assert_panic;

pub fn test<T, A: Alignment>(x: T, y: T, z: T, w: T)
where
    T: Scalar + Debug + Display + PartialEq + PartialOrd + Default + UnwindSafe + RefUnwindSafe,
{
    assert!(y > x && w > z);

    assert!(size_of::<Matrix<2, T, A>>() == size_of::<Vector<2, T, A>>() * 2);
    assert!(
        size_of::<Matrix<3, T, A>>() == size_of::<Vector<3, T, A>>() * 3
            || size_of::<Matrix<3, T, A>>() == size_of::<Vector<3, T, A>>() * 4
    );
    assert!(size_of::<Matrix<4, T, A>>() == size_of::<Vector<4, T, A>>() * 4);

    assert!(
        align_of::<Matrix<2, T, A>>() == align_of::<T>()
            || align_of::<Matrix<2, T, A>>() == align_of::<Vector<2, T, A>>()
            || align_of::<Matrix<2, T, A>>() == size_of::<Vector<2, T, A>>() * 2
    );
    assert!(
        align_of::<Matrix<3, T, A>>() == align_of::<T>()
            || align_of::<Matrix<3, T, A>>() == align_of::<Vector<3, T, A>>()
            || align_of::<Matrix<3, T, A>>() == size_of::<Vector<3, T, A>>() * 4
    );
    assert!(
        align_of::<Matrix<4, T, A>>() == align_of::<T>()
            || align_of::<Matrix<4, T, A>>() == align_of::<Vector<4, T, A>>()
            || align_of::<Matrix<4, T, A>>() == size_of::<Vector<4, T, A>>() * 4
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .as_columns(),
        &[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .as_columns(),
        &[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .as_columns(),
        &[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .as_columns(),
        &[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .as_columns(),
        &[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .as_columns(),
        &[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .align()
        .to_alignment::<A>()
        .as_columns(),
        &[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .align()
        .to_alignment::<A>()
        .as_columns(),
        &[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .align()
        .to_alignment::<A>()
        .as_columns(),
        &[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .unalign()
        .to_alignment::<A>()
        .as_columns(),
        &[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .unalign()
        .to_alignment::<A>()
        .as_columns(),
        &[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .unalign()
        .to_alignment::<A>()
        .as_columns(),
        &[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .as_columns(),
        &[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .as_columns(),
        &[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .as_columns(),
        &[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .as_columns_mut(),
        &mut [Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .as_columns_mut(),
        &mut [
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .as_columns_mut(),
        &mut [
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .x_axis,
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .y_axis,
        Vector::<2, T, A>::new(z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .x_axis,
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .y_axis,
        Vector::<3, T, A>::new(y, z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .z_axis,
        Vector::<3, T, A>::new(z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .x_axis,
        Vector::<4, T, A>::new(x, y, z, x)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .y_axis,
        Vector::<4, T, A>::new(y, z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .z_axis,
        Vector::<4, T, A>::new(z, x, y, z)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .w_axis,
        Vector::<4, T, A>::new(y, x, z, x)
    );

    assert_eq!(
        &mut Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .x_axis,
        &mut Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        &mut Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .y_axis,
        &mut Vector::<2, T, A>::new(z, x)
    );
    assert_eq!(
        &mut Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .x_axis,
        &mut Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        &mut Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .y_axis,
        &mut Vector::<3, T, A>::new(y, z, x)
    );
    assert_eq!(
        &mut Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .z_axis,
        &mut Vector::<3, T, A>::new(z, x, y)
    );
    assert_eq!(
        &mut Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .x_axis,
        &mut Vector::<4, T, A>::new(x, y, z, x)
    );
    assert_eq!(
        &mut Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .y_axis,
        &mut Vector::<4, T, A>::new(y, z, x, y)
    );
    assert_eq!(
        &mut Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .z_axis,
        &mut Vector::<4, T, A>::new(z, x, y, z)
    );
    assert_eq!(
        &mut Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .w_axis,
        &mut Vector::<4, T, A>::new(y, x, z, x)
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .column(0),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .column(1),
        Vector::<2, T, A>::new(z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column(0),
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column(1),
        Vector::<3, T, A>::new(y, z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column(2),
        Vector::<3, T, A>::new(z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column(0),
        Vector::<4, T, A>::new(x, y, z, x)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column(1),
        Vector::<4, T, A>::new(y, z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column(2),
        Vector::<4, T, A>::new(z, x, y, z)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column(3),
        Vector::<4, T, A>::new(y, x, z, x)
    );

    assert_panic!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .column(2)
    );
    assert_panic!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column(3)
    );
    assert_panic!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column(4)
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .column_mut(0),
        &mut Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .column_mut(1),
        &mut Vector::<2, T, A>::new(z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column_mut(0),
        &mut Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column_mut(1),
        &mut Vector::<3, T, A>::new(y, z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column_mut(2),
        &mut Vector::<3, T, A>::new(z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column_mut(0),
        &mut Vector::<4, T, A>::new(x, y, z, x)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column_mut(1),
        &mut Vector::<4, T, A>::new(y, z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column_mut(2),
        &mut Vector::<4, T, A>::new(z, x, y, z)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column_mut(3),
        &mut Vector::<4, T, A>::new(y, x, z, x)
    );

    assert_panic!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ])
        .column_mut(2)
    );
    assert_panic!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .column_mut(3)
    );
    assert_panic!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ])
        .column_mut(4)
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, z),
            Vector::<2, T, A>::new(y, x)
        ])
        .row(0),
        Vector::<2, T, A>::new(x, y)
    );
    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, z),
            Vector::<2, T, A>::new(y, x)
        ])
        .row(1),
        Vector::<2, T, A>::new(z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .row(0),
        Vector::<3, T, A>::new(x, y, z)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .row(1),
        Vector::<3, T, A>::new(y, z, x)
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .row(2),
        Vector::<3, T, A>::new(z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, y),
            Vector::<4, T, A>::new(y, z, x, x),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x)
        ])
        .row(0),
        Vector::<4, T, A>::new(x, y, z, x)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, y),
            Vector::<4, T, A>::new(y, z, x, x),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x)
        ])
        .row(1),
        Vector::<4, T, A>::new(y, z, x, y)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, y),
            Vector::<4, T, A>::new(y, z, x, x),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x)
        ])
        .row(2),
        Vector::<4, T, A>::new(z, x, y, z)
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, y),
            Vector::<4, T, A>::new(y, z, x, x),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x)
        ])
        .row(3),
        Vector::<4, T, A>::new(y, x, z, x)
    );

    assert_panic!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, z),
            Vector::<2, T, A>::new(y, x)
        ])
        .row(2)
    );
    assert_panic!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ])
        .row(3)
    );
    assert_panic!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, y),
            Vector::<4, T, A>::new(y, z, x, x),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x)
        ])
        .row(4)
    );

    let mut m = Matrix::<2, T, A>::from_columns(&[
        Vector::<2, T, A>::new(x, z),
        Vector::<2, T, A>::new(y, x),
    ]);
    m.set_row(0, Vector::<2, T, A>::new(y, x));
    assert_eq!(
        m,
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(y, z),
            Vector::<2, T, A>::new(x, x)
        ])
    );
    m.set_row(1, Vector::<2, T, A>::new(x, z));
    assert_eq!(
        m,
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(y, x),
            Vector::<2, T, A>::new(x, z)
        ])
    );
    assert_panic!(m.clone().set_row(2, Vector::<2, T, A>::new(x, y)));

    let mut m = Matrix::<3, T, A>::from_columns(&[
        Vector::<3, T, A>::new(x, y, z),
        Vector::<3, T, A>::new(y, z, x),
        Vector::<3, T, A>::new(z, x, y),
    ]);
    m.set_row(0, Vector::<3, T, A>::new(y, x, x));
    assert_eq!(
        m,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, y, z),
            Vector::<3, T, A>::new(x, z, x),
            Vector::<3, T, A>::new(x, x, y)
        ])
    );
    m.set_row(1, Vector::<3, T, A>::new(x, x, z));
    assert_eq!(
        m,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, x, z),
            Vector::<3, T, A>::new(x, x, x),
            Vector::<3, T, A>::new(x, z, y)
        ])
    );
    m.set_row(2, Vector::<3, T, A>::new(x, y, x));
    assert_eq!(
        m,
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, x, x),
            Vector::<3, T, A>::new(x, x, y),
            Vector::<3, T, A>::new(x, z, x)
        ])
    );
    assert_panic!(m.clone().set_row(3, Vector::<3, T, A>::new(x, y, z)));

    let mut m = Matrix::<4, T, A>::from_columns(&[
        Vector::<4, T, A>::new(x, y, z, y),
        Vector::<4, T, A>::new(y, z, x, x),
        Vector::<4, T, A>::new(z, x, y, z),
        Vector::<4, T, A>::new(x, y, z, x),
    ]);
    m.set_row(0, Vector::<4, T, A>::new(y, x, x, z));
    assert_eq!(
        m,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, y, z, y),
            Vector::<4, T, A>::new(x, z, x, x),
            Vector::<4, T, A>::new(x, x, y, z),
            Vector::<4, T, A>::new(z, y, z, x)
        ])
    );
    m.set_row(1, Vector::<4, T, A>::new(x, y, z, z));
    assert_eq!(
        m,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, x, z, y),
            Vector::<4, T, A>::new(x, y, x, x),
            Vector::<4, T, A>::new(x, z, y, z),
            Vector::<4, T, A>::new(z, z, z, x)
        ])
    );
    m.set_row(2, Vector::<4, T, A>::new(x, y, x, y));
    assert_eq!(
        m,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, x, x, y),
            Vector::<4, T, A>::new(x, y, y, x),
            Vector::<4, T, A>::new(x, z, x, z),
            Vector::<4, T, A>::new(z, z, y, x)
        ])
    );
    m.set_row(3, Vector::<4, T, A>::new(x, y, y, z));
    assert_eq!(
        m,
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, x, x, x),
            Vector::<4, T, A>::new(x, y, y, y),
            Vector::<4, T, A>::new(x, z, x, y),
            Vector::<4, T, A>::new(z, z, y, z)
        ])
    );
    assert_panic!(m.clone().set_row(4, Vector::<4, T, A>::new(x, y, z, x)));

    assert_eq!(
        format!(
            "{:?}",
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, x)
            ])
        ),
        format!("[({x:?}, {y:?}), ({z:?}, {x:?})]")
    );
    assert_eq!(
        format!(
            "{:?}",
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ])
        ),
        format!("[({x:?}, {y:?}, {z:?}), ({y:?}, {z:?}, {x:?}), ({z:?}, {x:?}, {y:?})]")
    );
    assert_eq!(
        format!(
            "{:?}",
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(y, x, z, x)
            ])
        ),
        format!(
            "[({x:?}, {y:?}, {z:?}, {x:?}), ({y:?}, {z:?}, {x:?}, {y:?}), ({z:?}, {x:?}, {y:?}, {z:?}), ({y:?}, {x:?}, {z:?}, {x:?})]"
        )
    );

    assert_eq!(
        format!(
            "{}",
            Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, x)
            ])
        ),
        format!("[({x}, {y}), ({z}, {x})]")
    );
    assert_eq!(
        format!(
            "{}",
            Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(z, x, y)
            ])
        ),
        format!("[({x}, {y}, {z}), ({y}, {z}, {x}), ({z}, {x}, {y})]")
    );
    assert_eq!(
        format!(
            "{}",
            Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, x),
                Vector::<4, T, A>::new(y, z, x, y),
                Vector::<4, T, A>::new(z, x, y, z),
                Vector::<4, T, A>::new(y, x, z, x)
            ])
        ),
        format!(
            "[({x}, {y}, {z}, {x}), ({y}, {z}, {x}, {y}), ({z}, {x}, {y}, {z}), ({y}, {x}, {z}, {x})]"
        )
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]) == Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]),
        true
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]) == Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]),
        true
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) == Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]),
        true
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(y, y),
            Vector::<2, T, A>::new(z, x)
        ]) == Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]),
        false
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]) == Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]),
        false
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) == Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]),
        false
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, y)
        ]) == Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]),
        false
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, y),
            Vector::<3, T, A>::new(z, x, y)
        ]) == Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, y),
            Vector::<3, T, A>::new(z, x, x)
        ]),
        false
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) == Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, y)
        ]),
        false
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, y)
        ]) == Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(y, x),
            Vector::<2, T, A>::new(y, x)
        ]),
        false
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, y),
            Vector::<3, T, A>::new(z, x, y)
        ]) == Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, x, x),
            Vector::<3, T, A>::new(z, y, x),
            Vector::<3, T, A>::new(x, y, z)
        ]),
        false
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) == Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, y, y, z)
        ]),
        false
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]) != Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]),
        false
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]) != Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]),
        false
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) != Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]),
        false
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(y, y),
            Vector::<2, T, A>::new(z, x)
        ]) != Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]),
        true
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]) != Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, x),
            Vector::<3, T, A>::new(z, x, y)
        ]),
        true
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) != Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]),
        true
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, y)
        ]) != Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, x)
        ]),
        true
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, y),
            Vector::<3, T, A>::new(z, x, y)
        ]) != Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, y),
            Vector::<3, T, A>::new(z, x, x)
        ]),
        true
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) != Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, y)
        ]),
        true
    );

    assert_eq!(
        Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(x, y),
            Vector::<2, T, A>::new(z, y)
        ]) != Matrix::<2, T, A>::from_columns(&[
            Vector::<2, T, A>::new(y, x),
            Vector::<2, T, A>::new(y, x)
        ]),
        true
    );
    assert_eq!(
        Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(x, y, z),
            Vector::<3, T, A>::new(y, z, y),
            Vector::<3, T, A>::new(z, x, y)
        ]) != Matrix::<3, T, A>::from_columns(&[
            Vector::<3, T, A>::new(y, x, x),
            Vector::<3, T, A>::new(z, y, x),
            Vector::<3, T, A>::new(x, y, z)
        ]),
        true
    );
    assert_eq!(
        Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, x, y, z),
            Vector::<4, T, A>::new(y, x, z, x)
        ]) != Matrix::<4, T, A>::from_columns(&[
            Vector::<4, T, A>::new(y, x, y, z),
            Vector::<4, T, A>::new(x, y, z, x),
            Vector::<4, T, A>::new(y, z, x, y),
            Vector::<4, T, A>::new(z, y, y, z)
        ]),
        true
    );
}
