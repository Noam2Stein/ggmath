use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({})",
            self.to_array().map(|c| format!("{c:?}")).join(", ")
        )
    }
}

impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.to_array().map(|c| c.to_string()).join(", "))
    }
}
