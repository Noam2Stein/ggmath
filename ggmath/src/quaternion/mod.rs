use crate::{
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub struct Quaternion<const N: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<N>: VecLen,
{
    _inner: Vector<N, T, A>,
}

pub type Quat2<T> = Quaternion<2, T, VecAligned>;
pub type Quat3<T> = Quaternion<3, T, VecAligned>;
pub type Quat4<T> = Quaternion<4, T, VecAligned>;

pub type Quat2P<T> = Quaternion<2, T, VecPacked>;
pub type Quat3P<T> = Quaternion<3, T, VecPacked>;
pub type Quat4P<T> = Quaternion<4, T, VecPacked>;
