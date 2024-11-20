use std::array;

use ggmath::{
    matrix::{major_axis::*, Matrix},
    vector::{alignment::*, length::*, Vector},
};
use ggmath_testing::testable_scalar::*;

pub fn test_matrix<T: TestableScalar>() {
    test_c_r_t::<2, 2, T>();
    test_c_r_t::<2, 3, T>();
    test_c_r_t::<2, 4, T>();

    test_c_r_t::<3, 2, T>();
    test_c_r_t::<3, 3, T>();
    test_c_r_t::<3, 4, T>();

    test_c_r_t::<4, 2, T>();
    test_c_r_t::<4, 3, T>();
    test_c_r_t::<4, 4, T>();
}
fn test_c_r_t<const C: usize, const R: usize, T: TestableScalar>()
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    test_c_r_t_a_m::<C, R, T, VecAligned, ColumnMajor>();
    test_c_r_t_a_m::<C, R, T, VecAligned, RowMajor>();

    test_c_r_t_a_m::<C, R, T, VecPacked, ColumnMajor>();
    test_c_r_t_a_m::<C, R, T, VecPacked, RowMajor>();
}

fn test_c_r_t_a_m<
    const C: usize,
    const R: usize,
    T: TestableScalar,
    A: VecAlignment,
    M: MatrixMajorAxis,
>()
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    let rows_array: [[T; C]; R] = array::from_fn(|row_index| {
        array::from_fn(|column_index| T::special_value(row_index * C + column_index))
    });
    let columns_array: [[T; R]; C] = array::from_fn(|column_index| {
        array::from_fn(|row_index| rows_array[row_index][column_index])
    });

    let rows = rows_array.map(|row_array| Vector::<C, T, A>::from_array(row_array));
    let columns = columns_array.map(|column_array| Vector::<R, T, A>::from_array(column_array));

    let matrix = Matrix::<C, R, T, A, M>::from_rows(rows);

    assert_eq!(matrix, Matrix::from_columns(columns));

    assert_eq!(matrix.into_rows(), rows);
    assert_eq!(matrix.into_columns(), columns);
    assert_eq!(matrix.into_rows_array(), rows_array);
    assert_eq!(matrix.into_columns_array(), columns_array);
    assert_eq!(matrix.into_aligned().into_rows_array(), rows_array);
    assert_eq!(matrix.into_packed().into_rows_array(), rows_array);
    assert_eq!(matrix.into_column_major().into_columns(), columns);
    assert_eq!(matrix.into_row_major().into_rows(), rows);
}
