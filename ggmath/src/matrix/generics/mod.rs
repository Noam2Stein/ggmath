use super::*;

mod major_axis;
pub use major_axis::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    pub fn into_aligned(self) -> Matrix<C, R, T, VecAligned, M> {
        self.into_alignment()
    }
    #[inline(always)]
    pub fn into_packed(self) -> Matrix<C, R, T, VecAligned, M> {
        self.into_alignment()
    }
    #[inline(always)]
    pub fn into_alignment<AOutput: VecAlignment>(self) -> Matrix<C, R, T, AOutput, M> {
        Matrix::from_resolved_major_axis_fns(
            || {
                Matrix::from_columns_fn(|column_index| {
                    self.get_column(column_index).unwrap().into_alignment()
                })
            },
            || Matrix::from_rows_fn(|row_index| self.get_row(row_index).unwrap().into_alignment()),
        )
    }

    #[inline(always)]
    pub fn into_storage<AOutput: VecAlignment, MOutput: MatrixMajorAxis>(
        self,
    ) -> Matrix<C, R, T, AOutput, MOutput> {
        self.into_alignment().into_major_axis()
    }
}
