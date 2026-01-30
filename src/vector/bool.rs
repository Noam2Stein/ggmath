use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

impl<const N: usize, A: Alignment> Vector<N, bool, A>
where
    Length<N>: SupportedLength,
{
    /// Returns `true` if all of the vector's components are `true`.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        match N {
            2 => self[0] && self[1],
            3 => self[0] && self[1] && self[2],
            4 => self[0] && self[1] && self[2] && self[3],
            _ => unreachable!(),
        }
    }

    /// Returns `true` if any of the vector's components are `true`.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        match N {
            2 => self[0] || self[1],
            3 => self[0] || self[1] || self[2],
            4 => self[0] || self[1] || self[2] || self[3],
            _ => unreachable!(),
        }
    }

    /// Selects between the components of `if_true` and `if_false` based on the
    /// boolean values of the vector.
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
