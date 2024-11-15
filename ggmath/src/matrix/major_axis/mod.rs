use std::array;

use super::*;

mod column_major;
mod row_major;
pub use column_major::*;
pub use row_major::*;

#[allow(private_bounds)]
pub trait MatrixMajorAxis: Sized {
    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>: Construct
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;

    fn from_columns<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        columns: [Vector<R, T, A>; C],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn from_rows<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        rows: [Vector<C, T, A>; R],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn from_columns_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        columns: [[T; R]; C],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn from_rows_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        rows: [[T; C]; R],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;

    fn into_columns<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [Vector<R, T, A>; C]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn into_columns_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [[T; R]; C]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn into_rows<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [Vector<C, T, A>; R]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn into_rows_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [[T; C]; R]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;

    fn get_column<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<Vector<R, T, A>>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn get_row<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<Vector<C, T, A>>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn get_column_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<[T; R]>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn get_row_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<[T; C]>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;

    unsafe fn get_column_unchecked<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Vector<R, T, A>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    unsafe fn get_row_unchecked<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Vector<C, T, A>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    unsafe fn get_column_array_unchecked<
        const C: usize,
        const R: usize,
        T: Scalar,
        A: VecAlignment,
    >(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> [T; R]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    unsafe fn get_row_array_unchecked<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> [T; C]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;

    fn into_column_major<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> Matrix<C, R, T, A, ColumnMajor>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
    fn into_row_major<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> Matrix<C, R, T, A, RowMajor>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>;
}
