use super::*;

pub type VectorOrScalar<const N: usize, S: VecStorage, T> =
    <ScalarCount<N> as VecLenOrOne>::Construct<S, T>;

pub type VecNInnerOrScalar<const N: usize, S: VecStorage, T> =
    <ScalarCount<N> as VecLenOrOne>::Construct<S, T>;

#[allow(private_bounds)]
pub trait VecLenOrOne: Seal {
    type Construct<S: VecStorage, T: Scalar>: Construct;
    type InnerConstruct<S: VecStorage, T: Scalar>: InnerConstruct;
}
impl Seal for ScalarCount<1> {}
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<3> {}
impl Seal for ScalarCount<4> {}
impl VecLenOrOne for ScalarCount<1> {
    type Construct<S: VecStorage, T: Scalar> = T;
    type InnerConstruct<S: VecStorage, T: Scalar> = T;
}
impl VecLenOrOne for ScalarCount<2> {
    type Construct<S: VecStorage, T: Scalar> = Vector<2, S, T>;
    type InnerConstruct<S: VecStorage, T: Scalar> = InnerVector<2, S, T>;
}
impl VecLenOrOne for ScalarCount<3> {
    type Construct<S: VecStorage, T: Scalar> = Vector<3, S, T>;
    type InnerConstruct<S: VecStorage, T: Scalar> = InnerVector<3, S, T>;
}
impl VecLenOrOne for ScalarCount<4> {
    type Construct<S: VecStorage, T: Scalar> = Vector<4, S, T>;
    type InnerConstruct<S: VecStorage, T: Scalar> = InnerVector<4, S, T>;
}

trait Seal {}
