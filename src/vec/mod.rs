//! staticly-lengthed vectors of [scalars](scalar) where the length is between 2-4.

use crate::{construct::*, scalar::*};

mod len;
mod scalar;
mod storage;
pub use len::*;
pub use scalar::*;
pub use storage::*;

pub mod inner;

mod api;
pub use api::*;

#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, S: VecStorage>
where
    ScalarCount<N>: VecLen<N>,
{
    inner: inner::InnerVector<N, T, S>,
}

pub type Vector2<T, S> = Vector<2, T, S>;
pub type Vector3<T, S> = Vector<3, T, S>;
pub type Vector4<T, S> = Vector<4, T, S>;

pub type VecN<const N: usize, T> = Vector<N, VecAligned, T>;
pub type Vec2<T> = Vector<2, T, VecAligned>;
pub type Vec3<T> = Vector<3, T, VecAligned>;
pub type Vec4<T> = Vector<4, T, VecAligned>;

pub type VecNP<const N: usize, T> = Vector<N, VecPacked, T>;
pub type Vec2P<T> = Vector<2, T, VecPacked>;
pub type Vec3P<T> = Vector<3, T, VecPacked>;
pub type Vec4P<T> = Vector<4, T, VecPacked>;
