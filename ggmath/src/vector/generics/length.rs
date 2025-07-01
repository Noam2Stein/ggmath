use super::*;

pub struct MaybeVecLen<const N: usize>;

pub trait VecLen {
    type Alignment<T: Scalar>: Construct;
}

impl VecLen for MaybeVecLen<2> {
    type Alignment<T: Scalar> = T::Vec2Alignment;
}
impl VecLen for MaybeVecLen<3> {
    type Alignment<T: Scalar> = T::Vec3Alignment;
}
impl VecLen for MaybeVecLen<4> {
    type Alignment<T: Scalar> = T::Vec4Alignment;
}
