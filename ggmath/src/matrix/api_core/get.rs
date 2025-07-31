use super::*;

// Get

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns the value at the specified column and row indices,
    /// or `None` if the indices are out of bounds.
    #[inline(always)]
    pub const fn get_cell(self, column_idx: usize, row_idx: usize) -> Option<T> {
        if column_idx < C && row_idx < R {
            Some(unsafe { self.get_cell_unchecked(column_idx, row_idx) })
        } else {
            None
        }
    }

    /// Returns the column at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub const fn get_column(self, column_idx: usize) -> Option<Vector<R, T, A>> {
        if column_idx < C {
            Some(unsafe { self.get_column_unchecked(column_idx) })
        } else {
            None
        }
    }

    /// Returns the row at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub const fn get_row(self, row_idx: usize) -> Option<Vector<C, T, A>> {
        if row_idx < R {
            Some(unsafe { self.get_row_unchecked(row_idx) })
        } else {
            None
        }
    }
}

// Get Unchecked

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns the value at the specified column and row indices,
    /// without bounds checking.
    #[inline(always)]
    pub const unsafe fn get_cell_unchecked(self, column_idx: usize, row_idx: usize) -> T {
        unsafe { *self.get_cell_ref_unchecked(column_idx, row_idx) }
    }

    /// Returns the column at the specified index,
    /// without bounds checking.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is column major.
    #[inline(always)]
    pub const unsafe fn get_column_unchecked(self, column_idx: usize) -> Vector<R, T, A> {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(mat) => unsafe {
                *mat.get_column_ref_unchecked(column_idx)
            },

            ResolvedMatrix::RowMajor(mat) => {
                let mut column_array = [mat.inner[0].index(0); R];

                let mut r = 0;
                while r < R {
                    column_array[r] = unsafe { mat.inner[r].get_unchecked(column_idx) };

                    r += 1;
                }

                Vector::from_array(column_array)
            }
        }
    }

    /// Returns the row at the specified index,
    /// without bounds checking.
    ///
    /// This works regardless of if the matrix is column major or row major,
    /// but is faster if the matrix is row major.
    #[inline(always)]
    pub const unsafe fn get_row_unchecked(self, row_idx: usize) -> Vector<C, T, A> {
        match self.resolve() {
            ResolvedMatrix::RowMajor(mat) => unsafe { *mat.get_row_ref_unchecked(row_idx) },

            ResolvedMatrix::ColumnMajor(mat) => {
                let mut row_array = [mat.inner[0].index(0); C];

                let mut c = 0;
                while c < C {
                    row_array[c] = unsafe { mat.inner[c].get_unchecked(row_idx) };

                    c += 1;
                }

                Vector::from_array(row_array)
            }
        }
    }
}

// Get Ref

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a reference to the value at the specified column and row indices,
    /// or `None` if the indices are out of bounds.
    #[inline(always)]
    pub const fn get_cell_ref(&self, column_idx: usize, row_idx: usize) -> Option<&T> {
        if column_idx < C && row_idx < R {
            Some(unsafe { self.get_cell_ref_unchecked(column_idx, row_idx) })
        } else {
            None
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, ColMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a reference to the column at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_column_ref(&self, column_idx: usize) -> Option<&Vector<R, T, A>> {
        if column_idx < C {
            Some(unsafe { self.get_column_ref_unchecked(column_idx) })
        } else {
            None
        }
    }

    /// Returns a reference to two consecutive columns of the matrix,
    /// starting at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_2_columns_ref(
        &self,
        column_idx: usize,
    ) -> Option<&Matrix<2, R, T, A, ColMajor>> {
        self.get_matc_ref(column_idx)
    }

    /// Returns a reference to three consecutive columns of the matrix,
    /// starting at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_3_columns_ref(
        &self,
        column_idx: usize,
    ) -> Option<&Matrix<3, R, T, A, ColMajor>> {
        self.get_matc_ref(column_idx)
    }

    /// Returns a reference to four consecutive columns of the matrix,
    /// starting at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_4_columns_ref(
        &self,
        column_idx: usize,
    ) -> Option<&Matrix<4, R, T, A, ColMajor>> {
        self.get_matc_ref(column_idx)
    }

    /// Returns a reference to a generic number of consecutive columns of the matrix,
    /// starting at the specified column index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    ///
    /// This is a generic version of [`Matrix::get_2_columns_ref`],
    /// [`Matrix::get_3_columns_ref`], and [`Matrix::get_4_columns_ref`].
    #[inline(always)]
    pub const fn get_matc_ref<const C_OUT: usize>(
        &self,
        column_idx: usize,
    ) -> Option<&Matrix<C_OUT, R, T, A, ColMajor>>
    where
        Usize<C_OUT>: VecLen,
    {
        if column_idx + C_OUT <= C {
            Some(unsafe { self.get_matc_ref_unchecked(column_idx) })
        } else {
            None
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, RowMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a reference to the row at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_row_ref(&self, row_idx: usize) -> Option<&Vector<C, T, A>> {
        if row_idx < R {
            Some(unsafe { self.get_row_ref_unchecked(row_idx) })
        } else {
            None
        }
    }

    /// Returns a reference to two consecutive rows of the matrix,
    /// starting at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_2_rows_ref(&self, row_idx: usize) -> Option<&Matrix<C, 2, T, A, RowMajor>> {
        self.get_matr_ref(row_idx)
    }

    /// Returns a reference to three consecutive rows of the matrix,
    /// starting at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_3_rows_ref(&self, row_idx: usize) -> Option<&Matrix<C, 3, T, A, RowMajor>> {
        self.get_matr_ref(row_idx)
    }

    /// Returns a reference to four consecutive rows of the matrix,
    /// starting at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_4_rows_ref(&self, row_idx: usize) -> Option<&Matrix<C, 4, T, A, RowMajor>> {
        self.get_matr_ref(row_idx)
    }

    /// Returns a reference to a generic number of consecutive rows of the matrix,
    /// starting at the specified row index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    ///
    /// This is a generic version of [`Matrix::get_2_rows_ref`],
    /// [`Matrix::get_3_rows_ref`], and [`Matrix::get_4_rows_ref`].
    #[inline(always)]
    pub const fn get_matr_ref<const R_OUT: usize>(
        &self,
        row_idx: usize,
    ) -> Option<&Matrix<C, R_OUT, T, A, RowMajor>>
    where
        Usize<R_OUT>: VecLen,
    {
        if row_idx + R_OUT <= R {
            Some(unsafe { self.get_matr_ref_unchecked(row_idx) })
        } else {
            None
        }
    }
}

// Get Ref Unchecked

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a reference to the value at the specified column and row indices,
    /// without bounds checking.
    #[inline(always)]
    pub const unsafe fn get_cell_ref_unchecked(&self, column_idx: usize, row_idx: usize) -> &T {
        unsafe {
            match self.resolve_ref() {
                ResolvedMatrixRef::ColumnMajor(mat) => mat
                    .get_column_ref_unchecked(column_idx)
                    .get_ref_unchecked(row_idx),

                ResolvedMatrixRef::RowMajor(mat) => mat
                    .get_row_ref_unchecked(row_idx)
                    .get_ref_unchecked(column_idx),
            }
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, ColMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a reference to the column at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_column_ref_unchecked(&self, column_idx: usize) -> &Vector<R, T, A> {
        unsafe {
            self.inner
                .as_ptr()
                .add(column_idx)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    /// Returns a reference to two consecutive columns of the matrix,
    /// starting at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_2_columns_ref_unchecked(
        &self,
        column_idx: usize,
    ) -> &Matrix<2, R, T, A, ColMajor> {
        unsafe { self.get_matc_ref_unchecked(column_idx) }
    }

    /// Returns a reference to three consecutive columns of the matrix,
    /// starting at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_3_columns_ref_unchecked(
        &self,
        column_idx: usize,
    ) -> &Matrix<3, R, T, A, ColMajor> {
        unsafe { self.get_matc_ref_unchecked(column_idx) }
    }

    /// Returns a reference to four consecutive columns of the matrix,
    /// starting at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_4_columns_ref_unchecked(
        &self,
        column_idx: usize,
    ) -> &Matrix<4, R, T, A, ColMajor> {
        unsafe { self.get_matc_ref_unchecked(column_idx) }
    }

    /// Returns a reference to a generic number of consecutive columns of the matrix,
    /// starting at the specified column index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    ///
    /// This is a generic version of [`Matrix::get_2_columns_ref_unchecked`],
    /// [`Matrix::get_3_columns_ref_unchecked`], and [`Matrix::get_4_columns_ref_unchecked`].
    #[inline(always)]
    pub const unsafe fn get_matc_ref_unchecked<const C_OUT: usize>(
        &self,
        column_idx: usize,
    ) -> &Matrix<C_OUT, R, T, A, ColMajor>
    where
        Usize<C_OUT>: VecLen,
    {
        unsafe {
            (self.inner.as_ptr() as *const Matrix<C_OUT, R, T, A, ColMajor>)
                .add(column_idx)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, RowMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a reference to the row at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_row_ref_unchecked(&self, row_idx: usize) -> &Vector<C, T, A> {
        unsafe { self.inner.as_ptr().add(row_idx).as_ref().unwrap_unchecked() }
    }

    /// Returns a reference to two consecutive rows of the matrix,
    /// starting at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_2_rows_ref_unchecked(
        &self,
        row_idx: usize,
    ) -> &Matrix<C, 2, T, A, RowMajor> {
        unsafe { self.get_matr_ref_unchecked(row_idx) }
    }

    /// Returns a reference to three consecutive rows of the matrix,
    /// starting at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_3_rows_ref_unchecked(
        &self,
        row_idx: usize,
    ) -> &Matrix<C, 3, T, A, RowMajor> {
        unsafe { self.get_matr_ref_unchecked(row_idx) }
    }

    /// Returns a reference to four consecutive rows of the matrix,
    /// starting at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_4_rows_ref_unchecked(
        &self,
        row_idx: usize,
    ) -> &Matrix<C, 4, T, A, RowMajor> {
        unsafe { self.get_matr_ref_unchecked(row_idx) }
    }

    /// Returns a reference to a generic number of consecutive rows of the matrix,
    /// starting at the specified row index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    ///
    /// This is a generic version of [`Matrix::get_2_rows_ref_unchecked`],
    /// [`Matrix::get_3_rows_ref_unchecked`], and [`Matrix::get_4_rows_ref_unchecked`].
    #[inline(always)]
    pub const unsafe fn get_matr_ref_unchecked<const R_OUT: usize>(
        &self,
        row_idx: usize,
    ) -> &Matrix<C, R_OUT, T, A, RowMajor>
    where
        Usize<R_OUT>: VecLen,
    {
        unsafe {
            (self.inner.as_ptr() as *const Matrix<C, R_OUT, T, A, RowMajor>)
                .add(row_idx)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

// Get Mut

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a mutable reference to the value at the specified column and row indices,
    /// or `None` if the indices are out of bounds.
    #[inline(always)]
    pub const fn get_cell_mut(&mut self, column_idx: usize, row_idx: usize) -> Option<&mut T> {
        if column_idx < C && row_idx < R {
            Some(unsafe { self.get_cell_mut_unchecked(column_idx, row_idx) })
        } else {
            None
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, ColMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a mutable reference to the column at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_column_mut(&mut self, column_idx: usize) -> Option<&mut Vector<R, T, A>> {
        if column_idx < C {
            Some(unsafe { self.get_column_mut_unchecked(column_idx) })
        } else {
            None
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, RowMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a mutable reference to the row at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const fn get_row_mut(&mut self, row_idx: usize) -> Option<&mut Vector<C, T, A>> {
        if row_idx < R {
            Some(unsafe { self.get_row_mut_unchecked(row_idx) })
        } else {
            None
        }
    }
}

// Get Mut Unchecked

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a mutable reference to the value at the specified column and row indices,
    /// without bounds checking.
    #[inline(always)]
    pub const unsafe fn get_cell_mut_unchecked(
        &mut self,
        column_idx: usize,
        row_idx: usize,
    ) -> &mut T {
        unsafe {
            match self.resolve_mut() {
                ResolvedMatrixMut::ColumnMajor(mat) => mat
                    .get_column_mut_unchecked(column_idx)
                    .get_mut_unchecked(row_idx),

                ResolvedMatrixMut::RowMajor(mat) => mat
                    .get_row_mut_unchecked(row_idx)
                    .get_mut_unchecked(column_idx),
            }
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, ColMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a mutable reference to the column at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be column major.
    /// This is because else the columns of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_column_mut_unchecked(
        &mut self,
        column_idx: usize,
    ) -> &mut Vector<R, T, A> {
        unsafe {
            self.inner
                .as_mut_ptr()
                .add(column_idx)
                .as_mut()
                .unwrap_unchecked()
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment> Matrix<C, R, T, A, RowMajor>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Returns a mutable reference to the row at the specified index,
    /// without bounds checking.
    ///
    /// This requires the matrix to be row major.
    /// This is because else the rows of the matrix would not be contiguous in memory.
    #[inline(always)]
    pub const unsafe fn get_row_mut_unchecked(&mut self, row_idx: usize) -> &mut Vector<C, T, A> {
        unsafe {
            self.inner
                .as_mut_ptr()
                .add(row_idx)
                .as_mut()
                .unwrap_unchecked()
        }
    }
}
