use std::fmt::{Debug, Display};

use super::*;

impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for item in self.as_array()[..N - 1] {
            write!(f, "{item}, ")?;
        }
        write!(f, "{}", self.as_array()[N - 1])?;

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for item in self.as_array()[..N - 1] {
            write!(f, "{item:?}, ")?;
        }
        write!(f, "{:?}", self.as_array()[N - 1])?;

        write!(f, ")")?;

        Ok(())
    }
}
