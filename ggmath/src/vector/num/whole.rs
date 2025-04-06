use newnum::WholeEquivalent;

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + WholeEquivalent<Whole: Scalar>, A: VecAlignment> WholeEquivalent
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Whole = Vector<N, T::Whole, A>;
}
