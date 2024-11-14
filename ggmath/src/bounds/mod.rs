use crate::{
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub struct Boundaries<const N: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<N>: VecLen<N>,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}

pub type Bounds2<T> = Boundaries<2, T, VecAligned>;
pub type Bounds3<T> = Boundaries<3, T, VecAligned>;
pub type Bounds4<T> = Boundaries<4, T, VecAligned>;

pub type Bounds2P<T> = Boundaries<2, T, VecPacked>;
pub type Bounds3P<T> = Boundaries<3, T, VecPacked>;
pub type Bounds4P<T> = Boundaries<4, T, VecPacked>;
