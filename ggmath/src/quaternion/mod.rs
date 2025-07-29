use super::{scalar::*, vector::*};

mod from_into;
mod new;

pub struct Quaternion<T: Scalar, A: VecAlignment> {
    inner: Vector<4, T, A>,
}

pub type Quat<T> = Quaternion<T, VecAligned>;
pub type QuatP<T> = Quaternion<T, VecPacked>;
