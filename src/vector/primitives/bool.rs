use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

impl<const N: usize, A: Alignment> Vector<N, bool, A>
where
    Length<N>: SupportedLength,
{
    /// Selects elements from `if_true` and `if_false` based on the values of
    /// the vector.
    #[inline]
    #[must_use]
    pub fn select<T: Scalar>(
        self,
        if_true: Vector<N, T, A>,
        if_false: Vector<N, T, A>,
    ) -> Vector<N, T, A> {
        Vector::from_fn(|i| if self[i] { if_true[i] } else { if_false[i] })
    }
}
