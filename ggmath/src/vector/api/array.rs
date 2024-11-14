use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    pub fn map<TOutput: Scalar>(self, f: impl FnMut(T) -> TOutput) -> Vector<N, TOutput, A> {
        Vector::from_array(self.into_array().map(f))
    }
}