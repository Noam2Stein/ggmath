use crate::{
    scalar::*,
    vector::{alignment::VecAlignment, length::*, *},
};

pub struct Boundaries<const N: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<N>: VecLen<N>,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}
