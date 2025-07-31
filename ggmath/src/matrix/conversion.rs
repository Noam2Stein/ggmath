use std::{array, mem::transmute_copy};

use super::*;

// T

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Converts the matrix to a different scalar type using the `From` trait.
    pub fn to_t<T2: Scalar + From<T>>(self) -> Matrix<C, R, T2, A, M> {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(m) => Matrix::from_columns(m.columns().map(|v| v.to_t())),
            ResolvedMatrix::RowMajor(m) => Matrix::from_rows(m.rows().map(|v| v.to_t())),
        }
    }
}

// A

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Aligns the matrix to [`VecAligned`] vectors.
    #[inline(always)]
    pub fn align(self) -> Matrix<C, R, T, VecAligned, M> {
        self.to_layout()
    }

    /// Unaligns the matrix to [`VecPacked`] vectors.
    ///
    /// This is always a zero cost operation.
    #[inline(always)]
    pub fn unalign(self) -> Matrix<C, R, T, VecAligned, M> {
        self.to_layout()
    }
}

// M

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Converts the matrix to a column major matrix.
    #[inline(always)]
    pub const fn col_major(self) -> Matrix<C, R, T, A, ColMajor> {
        Matrix::from_columns(self.columns())
    }

    /// Converts the matrix to a row major matrix.
    #[inline(always)]
    pub const fn row_major(self) -> Matrix<C, R, T, A, RowMajor> {
        Matrix::from_rows(self.rows())
    }
}

// Layout

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Converts the matrix to the specified memory-layout generics.
    ///
    /// This can convert between `VecAligned` and `VecPacked` matrices,
    /// and between `ColMajor` and `RowMajor` matrices.
    #[inline(always)]
    pub const fn to_layout<AOutput: VecAlignment, MOutput: MatMajorAxis>(
        self,
    ) -> Matrix<C, R, T, AOutput, MOutput> {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(self_) => Matrix::from_columns(self_.inner),
            ResolvedMatrix::RowMajor(self_) => Matrix::from_rows(self_.inner),
        }
    }
}

// Array

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Constructs a matrix by mapping column and row indices to their values using function `f`.
    ///
    /// The first argument of `f` is the column index,
    /// and the second argument is the row index.
    #[inline(always)]
    pub fn from_fn(mut f: impl FnMut(usize, usize) -> T) -> Self {
        match M::ENUM {
            MatMajorAxisEnum::ColumnMajor => Self::from_column_fn(|c| Vector::from_fn(|r| f(c, r))),
            MatMajorAxisEnum::RowMajor => Self::from_row_fn(|r| Vector::from_fn(|c| f(c, r))),
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Constructs a matrix from an array of column vectors.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub const fn from_columns(columns: [Vector<R, T, impl VecAlignment>; C]) -> Self {
        match M::ENUM {
            MatMajorAxisEnum::ColumnMajor => {
                let mut aligned_columns = [Vector::from_array([columns[0].index(0); R]); C];

                let mut c = 0;
                while c < C {
                    aligned_columns[c] = columns[c].to_layout();

                    c += 1;
                }

                unsafe {
                    transmute_copy::<Matrix<C, R, T, A, ColMajor>, Matrix<C, R, T, A, M>>(&Matrix {
                        inner: aligned_columns,
                    })
                }
            }

            MatMajorAxisEnum::RowMajor => {
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

    /// Constructs a matrix from an array of column arrays.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub const fn from_column_arrays(columns: [[T; R]; C]) -> Self {
        Self::from_columns(unsafe {
            transmute_copy::<[[T; R]; C], [Vector<R, T, VecPacked>; C]>(&columns)
        })
    }

    /// Constructs a matrix by mapping column indices to their values using function `f`.
    /// This is equivalent to [`std::array::from_fn`].
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub fn from_column_fn(f: impl FnMut(usize) -> Vector<R, T, A>) -> Self {
        Self::from_columns(array::from_fn(f))
    }

    /// Constructs a matrix by mapping column indices to their values using function `f`.
    /// This is equivalent to [`std::array::from_fn`].
    ///
    /// Unlike [`Matrix::from_column_fn`],
    /// this function asks for column arrays instead of column vectors.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub fn from_column_array_fn(f: impl FnMut(usize) -> [T; R]) -> Self {
        Self::from_column_arrays(array::from_fn(f))
    }

    /// Returns the columns of the matrix as an array of column vectors.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub const fn columns(self) -> [Vector<R, T, A>; C] {
        self.to_layout::<A, ColMajor>().inner
    }

    /// Returns the columns of the matrix as an array of column arrays.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub const fn column_arrays(self) -> [[T; R]; C] {
        unsafe {
            transmute_copy::<[Vector<R, T, VecPacked>; C], [[T; R]; C]>(
                &self.to_layout::<VecPacked, ColMajor>().inner,
            )
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Constructs a matrix from an array of row vectors.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub const fn from_rows(rows: [Vector<C, T, impl VecAlignment>; R]) -> Self {
        match M::ENUM {
            MatMajorAxisEnum::ColumnMajor => {
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
                    transmute_copy::<Matrix<C, R, T, A, ColMajor>, Matrix<C, R, T, A, M>>(&Matrix {
                        inner: columns,
                    })
                }
            }

            MatMajorAxisEnum::RowMajor => {
                let mut aligned_rows = [Vector::from_array([rows[0].index(0); C]); R];

                let mut r = 0;
                while r < R {
                    aligned_rows[r] = rows[r].to_layout();

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

    /// Constructs a matrix from an array of row arrays.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub const fn from_row_arrays(rows: [[T; C]; R]) -> Self {
        Self::from_rows(unsafe {
            transmute_copy::<[[T; C]; R], [Vector<C, T, VecPacked>; R]>(&rows)
        })
    }

    /// Constructs a matrix by mapping row indices to their values using function `f`.
    /// This is equivalent to [`std::array::from_fn`].
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub fn from_row_fn(f: impl FnMut(usize) -> Vector<C, T, A>) -> Self {
        Self::from_rows(array::from_fn(f))
    }

    /// Constructs a matrix by mapping row indices to their values using function `f`.
    /// This is equivalent to [`std::array::from_fn`].
    ///
    /// Unlike [`Matrix::from_row_fn`],
    /// this function asks for row arrays instead of row vectors.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub fn from_row_array_fn(f: impl FnMut(usize) -> [T; C]) -> Self {
        Self::from_row_arrays(array::from_fn(f))
    }

    /// Returns the rows of the matrix as an array of row vectors.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub const fn rows(self) -> [Vector<C, T, A>; R] {
        self.to_layout::<A, RowMajor>().inner
    }

    /// Returns the rows of the matrix as an array of row arrays.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub const fn row_arrays(self) -> [[T; C]; R] {
        unsafe {
            transmute_copy::<[Vector<C, T, VecPacked>; R], [[T; C]; R]>(
                &self.to_layout::<VecPacked, RowMajor>().inner,
            )
        }
    }
}
