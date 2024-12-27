use crate::{scalar::*, vector::*};

pub struct Quaternion<T: Scalar, A: VecAlignment>(pub Vector<4, T, A>);

pub type Quat<T> = Quaternion<T, VecAligned>;
pub type QuatP<T> = Quaternion<T, VecPacked>;
