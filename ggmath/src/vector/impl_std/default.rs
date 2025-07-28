use super::*;

impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn default() -> Self {
        Self {
            array: [T::default(); N],
            _align: Default::default(),
        }
    }
}
