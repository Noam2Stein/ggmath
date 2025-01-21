use newnum::FloatingEquivalent;

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + FloatingEquivalent<Floating: Scalar>, A: VecAlignment>
    FloatingEquivalent for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Floating = Vector<N, T::Floating, A>;

    fn floating(self) -> Self::Floating {
        self.map(FloatingEquivalent::floating)
    }
}
