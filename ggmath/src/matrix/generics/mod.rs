use super::*;

mod resolve;
pub use resolve::*;

mod alignment;
mod major_axis;
pub use major_axis::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
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
    pub const fn to_storage<AOutput: VecAlignment, MOutput: MatrixMajorAxis>(
        self,
    ) -> Matrix<C, R, T, AOutput, MOutput> {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(self_) => Matrix::from_columns(self_.inner),
            ResolvedMatrix::RowMajor(self_) => Matrix::from_rows(self_.inner),
        }
    }
}
