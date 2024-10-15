use super::*;

pub type VectorOrScalar<const N: usize, T, S> =
    <ScalarCount<N> as VecLenOrOne>::VectorOrScalar<T, S>;

pub type InnerVectorOrScalar<const N: usize, T, S> =
    <ScalarCount<N> as VecLenOrOne>::InnerVectorOrScalar<T, S>;

#[allow(private_bounds)]
pub trait VecLenOrOne: Seal {
    type VectorOrScalar<T: Scalar, S: VecAlignment>: Construct;
    type InnerVectorOrScalar<T: Scalar, S: VecAlignment>: InnerConstruct;
}
impl Seal for ScalarCount<1> {}
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<3> {}
impl Seal for ScalarCount<4> {}
impl VecLenOrOne for ScalarCount<1> {
    type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = T;
    type VectorOrScalar<T: Scalar, S: VecAlignment> = T;
}
impl VecLenOrOne for ScalarCount<2> {
    type VectorOrScalar<T: Scalar, S: VecAlignment> = Vector2<T, S>;
    type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = inner::InnerVector<2, T, S>;
}
impl VecLenOrOne for ScalarCount<3> {
    type VectorOrScalar<T: Scalar, S: VecAlignment> = Vector3<T, S>;
    type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = inner::InnerVector<3, T, S>;
}
impl VecLenOrOne for ScalarCount<4> {
    type VectorOrScalar<T: Scalar, S: VecAlignment> = Vector4<T, S>;
    type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = inner::InnerVector<4, T, S>;
}

trait Seal {}
