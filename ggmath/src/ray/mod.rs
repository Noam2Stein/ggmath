use crate::{scalar::*, vector::*};

pub struct Ray<const N: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<N>: VecLen,
{
    pub start: Vector<N, T, A>,
    pub direction: Vector<N, T, A>,
}

pub type Ray2<T> = Ray<2, T, VecAligned>;
pub type Ray3<T> = Ray<3, T, VecAligned>;
pub type Ray4<T> = Ray<4, T, VecAligned>;

pub type Ray2P<T> = Ray<2, T, VecPacked>;
pub type Ray3P<T> = Ray<3, T, VecPacked>;
pub type Ray4P<T> = Ray<4, T, VecPacked>;
