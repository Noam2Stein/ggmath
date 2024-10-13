use super::*;

pub type VectorOrScalar<const N: usize, S, T> =
    <ScalarCount<N> as VecLenOrOne>::VectorOrScalar<S, T>;

pub type InnerVectorOrScalar<const N: usize, S, T> =
    <ScalarCount<N> as VecLenOrOne>::InnerVectorOrScalar<S, T>;

#[allow(private_bounds)]
pub trait VecLenOrOne: Seal {
    type VectorOrScalar<S: VecStorage, T: Scalar>: Construct;
    type InnerVectorOrScalar<S: VecStorage, T: Scalar>: InnerConstruct;
}
impl Seal for ScalarCount<1> {}
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<3> {}
impl Seal for ScalarCount<4> {}
impl VecLenOrOne for ScalarCount<1> {
    type VectorOrScalar<S: VecStorage, T: Scalar> = T;
    type InnerVectorOrScalar<S: VecStorage, T: Scalar> = T;
}
impl VecLenOrOne for ScalarCount<2> {
    type VectorOrScalar<S: VecStorage, T: Scalar> = Vector<2, S, T>;
    type InnerVectorOrScalar<S: VecStorage, T: Scalar> = InnerVector<2, S, T>;
}
impl VecLenOrOne for ScalarCount<3> {
    type VectorOrScalar<S: VecStorage, T: Scalar> = Vector<3, S, T>;
    type InnerVectorOrScalar<S: VecStorage, T: Scalar> = InnerVector<3, S, T>;
}
impl VecLenOrOne for ScalarCount<4> {
    type VectorOrScalar<S: VecStorage, T: Scalar> = Vector<4, S, T>;
    type InnerVectorOrScalar<S: VecStorage, T: Scalar> = InnerVector<4, S, T>;
}

trait Seal {}
