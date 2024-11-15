use std::fmt::{self, Debug, Display, Formatter};

use super::*;

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.into_array().map(|c| format!("{c:?}")).join(", ")
        )
    }
}

impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.into_array().map(|c| c.to_string()).join(", ")
        )
    }
}
