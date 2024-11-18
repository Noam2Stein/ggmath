use super::*;

pub struct ColumnMajor;

impl MatrixMajorAxis for ColumnMajor {
    type
     InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment> = [Vector<R, T, A>; C]where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>;

    fn from_columns<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        columns: [Vector<R, T, A>; C],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Matrix { inner: columns }
    }
    fn from_columns_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        columns: [[T; R]; C],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Matrix {
            inner: columns.map(|column| Vector::from_array(column)),
        }
    }
    fn from_rows<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        rows: [Vector<C, T, A>; R],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Matrix {
            inner: array::from_fn(|column_index| {
                Vector::from_array(rows.map(|row| row[column_index]))
            }),
        }
    }
    fn from_rows_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        rows: [[T; C]; R],
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Matrix {
            inner: array::from_fn(|column_index| {
                Vector::from_array(rows.map(|row| row[column_index]))
            }),
        }
    }

    fn into_columns<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [Vector<R, T, A>; C]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat.inner
    }
    fn into_columns_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [[T; R]; C]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat.inner.map(|column| column.into_array())
    }
    fn into_rows<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [Vector<C, T, A>; R]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        array::from_fn(|row_index| Self::get_row(mat, row_index).unwrap())
    }
    fn into_rows_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> [[T; C]; R]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        array::from_fn(|row_index| Self::get_row_array(mat, row_index).unwrap())
    }

    fn get_column<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<Vector<R, T, A>>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat.inner.get(index).copied()
    }
    fn get_column_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<[T; R]>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat.inner.get(index).map(|column| column.into_array())
    }
    fn get_row<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<Vector<C, T, A>>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        if index >= R {
            None
        } else {
            Some(Vector::from_array(array::from_fn(|column_index| {
                mat.inner[column_index][index]
            })))
        }
    }
    fn get_row_array<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Option<[T; C]>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        if index >= R {
            None
        } else {
            Some(array::from_fn(|column_index| {
                mat.inner[column_index][index]
            }))
        }
    }

    unsafe fn get_column_unchecked<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Vector<R, T, A>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        *mat.inner.get_unchecked(index)
    }
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
        ScalarCount<R>: VecLen<R>,
    {
        mat.inner.get_unchecked(index).into_array()
    }
    unsafe fn get_row_unchecked<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> Vector<C, T, A>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Vector::from_array(array::from_fn(|column_index| {
            mat.inner[column_index].get_unchecked(index)
        }))
    }
    unsafe fn get_row_array_unchecked<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
        index: usize,
    ) -> [T; C]
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        array::from_fn(|column_index| mat.inner[column_index].get_unchecked(index))
    }

    fn into_alignment<
        const C: usize,
        const R: usize,
        T: Scalar,
        A: VecAlignment,
        AOutput: VecAlignment,
    >(
        mat: Matrix<C, R, T, A, Self>,
    ) -> Matrix<C, R, T, AOutput, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Matrix {
            inner: mat.inner.map(|column| column.into_alignment()),
        }
    }

    fn into_column_major<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> Matrix<C, R, T, A, ColumnMajor>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat
    }
    fn into_row_major<const C: usize, const R: usize, T: Scalar, A: VecAlignment>(
        mat: Matrix<C, R, T, A, Self>,
    ) -> Matrix<C, R, T, A, RowMajor>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        Matrix {
            inner: array::from_fn(|row_index| Self::get_row(mat, row_index).unwrap()),
        }
    }
    fn from_major_axis<
        const C: usize,
        const R: usize,
        T: Scalar,
        A: VecAlignment,
        MInput: MatrixMajorAxis,
    >(
        mat: Matrix<C, R, T, A, MInput>,
    ) -> Matrix<C, R, T, A, Self>
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat.into_column_major()
    }

    fn eq<const C: usize, const R: usize, T: ScalarPartialEq<Rhs>, A: VecAlignment, Rhs: Scalar>(
        mat: &Matrix<C, R, T, A, Self>,
        other: &Matrix<C, R, Rhs, A, Self>,
    ) -> bool
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        mat.inner.eq(&other.inner)
    }
}
