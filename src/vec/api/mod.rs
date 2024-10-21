//! Scalar supertraits for vector API

use super::*;

mod array;
pub use array::*;

mod construct;

/// Scalar supertrait for implementing the vector API for a specific <N, S>.
/// This is an empty trait with more supertraits for every related fn set.
pub trait ScalarVecApi<const N: usize, S: VecAlignment>:
    inner::ScalarInnerVecs + array::ScalarVecArrayApi<N, S>
where
    ScalarCount<N>: VecLen<N>,
{
}

#[allow(private_bounds)]
pub(super) trait VecLenApi<const N: usize>: array::VecLenArrayApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecLenApi<2> for ScalarCount<2> {}
impl VecLenApi<3> for ScalarCount<3> {}
impl VecLenApi<4> for ScalarCount<4> {}

#[allow(private_bounds)]
pub(super) trait VecAlignmentApi:
    array::VecAlignmentArrayApi<2> + array::VecAlignmentArrayApi<3> + array::VecAlignmentArrayApi<4>
{
}
impl VecAlignmentApi for VecPacked {}
impl VecAlignmentApi for VecAligned {}

mod prelude {
    pub use crate::{
        scalar::*,
        vec::{inner::*, *},
    };
}
