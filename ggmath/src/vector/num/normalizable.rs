use newnum::*;

use crate::{scalar::*, vector::*};

impl<const N: usize, T: Scalar + Normalizable<BoolMapped: Scalar> + Zero, A: VecAlignment>
    Normalizable for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    fn signum(self) -> Self
    where
        Self: Zero,
    {
        self.map(T::signum)
    }
    fn signumf(self) -> Self {
        self.map(T::signumf)
    }
}
