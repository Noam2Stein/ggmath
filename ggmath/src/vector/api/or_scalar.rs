use super::*;

pub type VectorOrScalar<const N: usize, T, A> = <ScalarCount<N> as VecLenOr1>::VectorOrScalar<T, A>;

pub trait VecLenOr1 {
    type VectorOrScalar<T: Scalar, A: VecAlignment>: Construct;
}

impl<const N: usize> VecLenOr1 for ScalarCount<N>
where
    ScalarCount<N>: VecLen,
{
    type VectorOrScalar<T: Scalar, A: VecAlignment> = Vector<N, T, A>;
}

impl VecLenOr1 for ScalarCount<1> {
    type VectorOrScalar<T: Scalar, A: VecAlignment> = T;
}

trait Seal {}
impl<const N: usize> Seal for ScalarCount<N> {}
