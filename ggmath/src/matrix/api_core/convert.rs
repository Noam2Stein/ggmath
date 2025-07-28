use std::{array, mem::transmute_copy};

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    pub const fn from_columns(columns: [Vector<R, T, impl VecAlignment>; C]) -> Self {
        match M::ENUM {
            MatrixMajorAxisEnum::ColumnMajor => {
                let mut aligned_columns = [Vector::from_array([columns[0].index(0); R]); C];

                let mut c = 0;
                while c < C {
                    aligned_columns[c] = columns[c].to_storage();

                    c += 1;
                }

                unsafe {
                    transmute_copy::<Matrix<C, R, T, A, ColumnMajor>, Matrix<C, R, T, A, M>>(
                        &Matrix {
                            inner: aligned_columns,
                        },
                    )
                }
            }

            MatrixMajorAxisEnum::RowMajor => {
                let mut rows = [Vector::from_array([columns[0].index(0); C]); R];

                let mut r = 0;
                while r < R {
                    let mut c = 0;
                    while c < C {
                        *rows[r].index_mut(c) = columns[c].index(r);

                        c += 1;
                    }

                    r += 1;
                }

                unsafe {
                    transmute_copy::<Matrix<C, R, T, A, RowMajor>, Matrix<C, R, T, A, M>>(&Matrix {
                        inner: rows,
                    })
                }
            }
        }
    }

    #[inline(always)]
    pub const fn from_column_arrays(columns: [[T; R]; C]) -> Self {
        Self::from_columns(unsafe {
            transmute_copy::<[[T; R]; C], [Vector<R, T, VecPacked>; C]>(&columns)
        })
    }

    #[inline(always)]
    pub fn from_column_fn(f: impl FnMut(usize) -> Vector<R, T, A>) -> Self {
        Self::from_columns(array::from_fn(f))
    }

    #[inline(always)]
    pub fn from_column_array_fn(f: impl FnMut(usize) -> [T; R]) -> Self {
        Self::from_column_arrays(array::from_fn(f))
    }

    #[inline(always)]
    pub const fn columns(self) -> [Vector<R, T, A>; C] {
        self.to_storage::<A, ColumnMajor>().inner
    }

    #[inline(always)]
    pub const fn column_arrays(self) -> [[T; R]; C] {
        unsafe {
            transmute_copy::<[Vector<R, T, VecPacked>; C], [[T; R]; C]>(
                &self.to_storage::<VecPacked, ColumnMajor>().inner,
            )
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    pub const fn from_rows(rows: [Vector<C, T, impl VecAlignment>; R]) -> Self {
        match M::ENUM {
            MatrixMajorAxisEnum::ColumnMajor => {
                let mut columns = [Vector::from_array([rows[0].index(0); R]); C];

                let mut c = 0;
                while c < C {
                    let mut r = 0;
                    while r < R {
                        *columns[c].index_mut(r) = rows[r].index(c);

                        r += 1;
                    }

                    c += 1;
                }

                unsafe {
                    transmute_copy::<Matrix<C, R, T, A, ColumnMajor>, Matrix<C, R, T, A, M>>(
                        &Matrix { inner: columns },
                    )
                }
            }

            MatrixMajorAxisEnum::RowMajor => {
                let mut aligned_rows = [Vector::from_array([rows[0].index(0); C]); R];

                let mut r = 0;
                while r < R {
                    aligned_rows[r] = rows[r].to_storage();

                    r += 1;
                }

                unsafe {
                    transmute_copy::<Matrix<C, R, T, A, RowMajor>, Matrix<C, R, T, A, M>>(&Matrix {
                        inner: aligned_rows,
                    })
                }
            }
        }
    }

    #[inline(always)]
    pub const fn from_row_arrays(rows: [[T; C]; R]) -> Self {
        Self::from_rows(unsafe {
            transmute_copy::<[[T; C]; R], [Vector<C, T, VecPacked>; R]>(&rows)
        })
    }

    #[inline(always)]
    pub fn from_row_fn(f: impl FnMut(usize) -> Vector<C, T, A>) -> Self {
        Self::from_rows(array::from_fn(f))
    }

    #[inline(always)]
    pub fn from_row_array_fn(f: impl FnMut(usize) -> [T; C]) -> Self {
        Self::from_row_arrays(array::from_fn(f))
    }

    #[inline(always)]
    pub const fn rows(self) -> [Vector<C, T, A>; R] {
        self.to_storage::<A, RowMajor>().inner
    }

    #[inline(always)]
    pub const fn row_arrays(self) -> [[T; C]; R] {
        unsafe {
            transmute_copy::<[Vector<C, T, VecPacked>; R], [[T; C]; R]>(
                &self.to_storage::<VecPacked, RowMajor>().inner,
            )
        }
    }
}
