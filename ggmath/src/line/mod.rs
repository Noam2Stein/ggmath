use crate::{
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub struct Line<const N: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<N>: VecLen<N>,
{
    pub start: Vector<N, T, A>,
    pub end: Vector<N, T, A>,
}

pub type Line2<T> = Line<2, T, VecAligned>;
pub type Line3<T> = Line<3, T, VecAligned>;
pub type Line4<T> = Line<4, T, VecAligned>;

pub type Line2P<T> = Line<2, T, VecPacked>;
pub type Line3P<T> = Line<3, T, VecPacked>;
pub type Line4P<T> = Line<4, T, VecPacked>;
