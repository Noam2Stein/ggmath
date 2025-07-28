use super::*;

pub trait VecLen {
    type Alignment<T: Scalar>: AlignTrait;
}

impl VecLen for Usize<2> {
    type Alignment<T: Scalar> = T::Vec2Alignment;
}
impl VecLen for Usize<3> {
    type Alignment<T: Scalar> = T::Vec3Alignment;
}
impl VecLen for Usize<4> {
    type Alignment<T: Scalar> = T::Vec4Alignment;
}
