use crate::{
    Affine, Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    constants::{Nan, One, Zero},
};

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// All zeros.
    ///
    /// Transforms any finite vector to zero.
    pub const ZERO: Self = Self::from_mat_translation(Matrix::ZERO, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero + One,
{
    /// The identity transform.
    ///
    /// Corresponds to no transformation.
    pub const IDENTITY: Self = Self::from_mat_translation(Matrix::IDENTITY, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// All NaN (Not a Number).
    pub const NAN: Self = Self::from_mat_translation(Matrix::NAN, Vector::NAN);
}
