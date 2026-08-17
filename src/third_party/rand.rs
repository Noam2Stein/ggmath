use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::{
    Affine, Alignment, Length, Mask, Matrix, Projective, Quaternion, Scalar, SupportedLength,
    Vector, length::TwoOrThree, utils::specialize_23,
};

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
        Matrix::from_rows(&rng.random::<[Vector<N, T, A>; N]>())
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
        Affine::from_matrix_translation(
            rng.random::<Matrix<N, T, A>>(),
            rng.random::<Vector<N, T, A>>(),
        )
    }
}

impl<const N: usize, T, A: Alignment> Distribution<Projective<N, T, A>> for StandardUniform
where
    Length<N>: TwoOrThree,
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Projective<N, T, A> {
        specialize_23!(Projective::<N, T, A>::sample_backend((rng,)))
    }
}

impl<T, A: Alignment> Projective<2, T, A>
where
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline(always)]
    fn sample_backend<R: Rng + ?Sized>((rng,): (&mut R,)) -> Self {
        Self(rng.random::<Matrix<3, T, A>>())
    }
}

impl<T, A: Alignment> Projective<3, T, A>
where
    T: Scalar,
    StandardUniform: Distribution<T>,
{
    #[inline(always)]
    fn sample_backend<R: Rng + ?Sized>((rng,): (&mut R,)) -> Self {
        Self(rng.random::<Matrix<4, T, A>>())
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

    use crate::{Affine, Mask, Matrix, Projective, Quaternion, Vector, test_utils::for_types};

    #[test]
    fn test_vector() {
        for_types!(|A| {
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
        for_types!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Matrix<2, f32, A>>(),
                Matrix::from_rows(&rng().random())
            );
            assert_eq!(
                rng().random::<Matrix<3, f32, A>>(),
                Matrix::from_rows(&rng().random())
            );
            assert_eq!(
                rng().random::<Matrix<4, f32, A>>(),
                Matrix::from_rows(&rng().random())
            );
        });
    }

    #[test]
    fn test_affine() {
        for_types!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Affine<2, f32, A>>(),
                Affine::<2, f32, A>::from_rows(&rng().random())
            );
            assert_eq!(
                rng().random::<Affine<3, f32, A>>(),
                Affine::<3, f32, A>::from_rows(&rng().random())
            );
            assert_eq!(
                rng().random::<Affine<4, f32, A>>(),
                Affine::<4, f32, A>::from_rows(&rng().random())
            );
        });
    }

    #[test]
    fn test_projective() {
        for_types!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Projective<2, f32, A>>(),
                Projective(rng().random())
            );
            assert_eq!(
                rng().random::<Projective<3, f32, A>>(),
                Projective(rng().random())
            );
        });
    }

    #[test]
    fn test_quaternion() {
        for_types!(|A| {
            let rng = || StdRng::from_seed([0; 32]);

            assert_eq!(
                rng().random::<Quaternion<f32, A>>(),
                Quaternion::from_array(rng().random())
            );
        });
    }

    #[test]
    fn test_mask() {
        for_types!(|T: PrimitiveNumber, A| {
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
