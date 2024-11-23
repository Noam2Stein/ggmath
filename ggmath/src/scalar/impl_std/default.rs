use super::*;

pub trait ScalarDefault: Scalar + Default {
    #[inline(always)]
    fn vector_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::splat(Self::default())
    }
}

impl<const N: usize, T: ScalarDefault, A: VecAlignment> Default for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn default() -> Self {
        T::vector_default()
    }
}
