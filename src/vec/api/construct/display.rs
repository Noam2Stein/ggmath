use std::fmt::{self, Display, Formatter};

use super::*;

impl<const N: usize, S: VecStorage, T: Scalar> Display for Vector<N, S, T>
where
    ScalarCount<N>: VecLen<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
