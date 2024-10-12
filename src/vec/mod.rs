use crate::{construct::*, scalar::*};

mod len;
mod scalar;
mod storage;
pub use len::*;
pub use scalar::*;
pub use storage::*;

mod api;
pub use api::*;

#[repr(transparent)]
pub struct Vector<const N: usize, S: VecStorage, T: Scalar>
where
    ScalarCount<N>: VecLen<N>,
{
    inner: InnerVector<N, S, T>,
}

pub type Vec2S<S, T> = Vector<2, S, T>;
pub type Vec3S<S, T> = Vector<3, S, T>;
pub type Vec4S<S, T> = Vector<4, S, T>;

pub type VecN<const N: usize, T> = Vector<N, VecAligned, T>;
pub type Vec2<T> = Vector<2, VecAligned, T>;
pub type Vec3<T> = Vector<3, VecAligned, T>;
pub type Vec4<T> = Vector<4, VecAligned, T>;

pub type VecNP<const N: usize, T> = Vector<N, VecPacked, T>;
pub type Vec2P<T> = Vector<2, VecPacked, T>;
pub type Vec3P<T> = Vector<3, VecPacked, T>;
pub type Vec4P<T> = Vector<4, VecPacked, T>;

pub type InnerVector<const N: usize, S, T> = <ScalarCount<N> as VecLen<N>>::InnerVector<S, T>;
