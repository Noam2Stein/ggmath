use derive_where::derive_where;

use super::{scalar::*, vector::*};

mod from_into;
mod new;
mod ops;

#[derive_where(Clone, Copy)]
#[derive_where(Debug, Eq, Hash; T)]
pub struct Quaternion<T: Scalar, A: VecAlignment> {
    inner: Vector<4, T, A>,
}

pub type Quat<T> = Quaternion<T, VecAligned>;
pub type QuatP<T> = Quaternion<T, VecPacked>;
