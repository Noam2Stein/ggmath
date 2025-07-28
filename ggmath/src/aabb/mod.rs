use derive_where::derive_where;

use super::{construct::*, scalar::*, vector::*, *};

mod api;
mod generics;
mod impl_std;
pub use api::*;
pub use generics::*;

#[derive_where(Clone, Copy, PartialEq)]
pub struct Aabb<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr>
where
    Usize<N>: VecLen,
{
    inner: R::InnerAabb<N, T, A>,
}

// 2D

pub type Rect<T> = Aabb<2, T, VecAligned, AabbCornered>;
pub type RectP<T> = Aabb<2, T, VecPacked, AabbCornered>;

pub type RectC<T> = Aabb<2, T, VecAligned, AabbCentered>;
pub type RectCP<T> = Aabb<2, T, VecPacked, AabbCentered>;

pub type RectM<T> = Aabb<2, T, VecAligned, AabbMinMaxed>;
pub type RectMP<T> = Aabb<2, T, VecPacked, AabbMinMaxed>;

// 3D

pub type Box<T> = Aabb<3, T, VecAligned, AabbCornered>;
pub type BoxP<T> = Aabb<3, T, VecPacked, AabbCornered>;

pub type BoxC<T> = Aabb<3, T, VecAligned, AabbCentered>;
pub type BoxCP<T> = Aabb<3, T, VecPacked, AabbCentered>;

pub type BoxM<T> = Aabb<3, T, VecAligned, AabbMinMaxed>;
pub type BoxMP<T> = Aabb<3, T, VecPacked, AabbMinMaxed>;

// 4D

pub type Aabb4<T> = Aabb<4, T, VecAligned, AabbCornered>;
pub type Aabb4P<T> = Aabb<4, T, VecPacked, AabbCornered>;

pub type Aabb4C<T> = Aabb<4, T, VecAligned, AabbCentered>;
pub type Aabb4CP<T> = Aabb<4, T, VecPacked, AabbCentered>;

pub type Aabb4M<T> = Aabb<4, T, VecAligned, AabbMinMaxed>;
pub type Aabb4MP<T> = Aabb<4, T, VecPacked, AabbMinMaxed>;
