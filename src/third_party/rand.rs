use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::{Affine, Alignment, Length, Mask, Matrix, Quaternion, Scalar, SupportedLength, Vector};

impl<const N: usize, T, A: Alignment> Distribution<Vector<N, T, A>> for StandardUniform
where
    Length<N>: SupportedLength,
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<N, T, A> {
        Vector::from_array(rng.random::<[T; N]>())
    }
}

impl<const N: usize, T, A: Alignment> Distribution<Matrix<N, T, A>> for StandardUniform
where
    Length<N>: SupportedLength,
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<N, T, A> {
        Matrix::from_columns(&rng.random::<[Vector<N, T, A>; N]>())
    }
}

impl<T, A: Alignment> Distribution<Quaternion<T, A>> for StandardUniform
where
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T, A> {
        Quaternion::from_array(rng.random::<[T; 4]>())
    }
}

impl<const N: usize, T, A: Alignment> Distribution<Affine<N, T, A>> for StandardUniform
where
    Length<N>: SupportedLength,
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Affine<N, T, A> {
        Affine::from_submatrix_translation(
            rng.random::<Matrix<N, T, A>>(),
            rng.random::<Vector<N, T, A>>(),
        )
    }
}

impl<const N: usize, T, A: Alignment> Distribution<Mask<N, T, A>> for StandardUniform
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mask<N, T, A> {
        Mask::from_array(rng.random())
    }
}

#[cfg(test)]
mod tests {
    use rand::{RngExt, SeedableRng, rngs::StdRng};

    use crate::{Affine, Mask, Matrix, Quaternion, Vector, utils::for_parameters};

    #[test]
    fn test_vector() {
        for_parameters!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Vector<2, f32, A>>(),
                Vector::from_array(rng().random())
            );
            assert_eq!(
                rng().random::<Vector<3, f32, A>>(),
                Vector::from_array(rng().random())
            );
            assert_eq!(
                rng().random::<Vector<4, f32, A>>(),
                Vector::from_array(rng().random())
            );
        });
    }

    #[test]
    fn test_matrix() {
        for_parameters!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Matrix<2, f32, A>>(),
                Matrix::from_columns(&rng().random())
            );
            assert_eq!(
                rng().random::<Matrix<3, f32, A>>(),
                Matrix::from_columns(&rng().random())
            );
            assert_eq!(
                rng().random::<Matrix<4, f32, A>>(),
                Matrix::from_columns(&rng().random())
            );
        });
    }

    #[test]
    fn test_quaternion() {
        for_parameters!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Quaternion<f32, A>>(),
                Quaternion::from_array(rng().random())
            );
        });
    }

    #[test]
    fn test_affine() {
        for_parameters!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Affine<2, f32, A>>(),
                Affine::<2, f32, A>::from_columns(&rng().random())
            );
            assert_eq!(
                rng().random::<Affine<3, f32, A>>(),
                Affine::<3, f32, A>::from_columns(&rng().random())
            );
            assert_eq!(
                rng().random::<Affine<4, f32, A>>(),
                Affine::<4, f32, A>::from_columns(&rng().random())
            );
        });
    }

    #[test]
    fn test_mask() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Mask<2, T, A>>(),
                Mask::from_array(rng().random())
            );
            assert_eq!(
                rng().random::<Mask<3, T, A>>(),
                Mask::from_array(rng().random())
            );
            assert_eq!(
                rng().random::<Mask<4, T, A>>(),
                Mask::from_array(rng().random())
            );
        });
    }
}
