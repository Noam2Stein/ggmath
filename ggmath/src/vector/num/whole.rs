use newnum::Whole;

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Whole, A: VecAlignment> Whole for Vector<N, T, A> where
    ScalarCount<N>: VecLen
{
}
