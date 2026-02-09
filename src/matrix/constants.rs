use crate::{
    Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    constants::{Nan, One, Zero},
};

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// A matrix with all cells set to `0.0`.
    pub const ZERO: Self = Self::from_col_array(&[Vector::ZERO; N]);
}

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero + One,
{
    /// The identity matrix, where diagonal cells are `1` and all other cells
    /// are `0`. Corresponds to no transformation.
    pub const IDENTITY: Self = Self::from_diagonal(Vector::ONE);
}

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// A matrix where all cells are NaNs (Not a Number).
    pub const NAN: Self = Self::from_col_array(&[Vector::NAN; N]);
}
