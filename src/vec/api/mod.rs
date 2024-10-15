//! Scalar supertraits for vector API

use super::*;

mod array;
mod or_scalar;
pub use array::*;
pub use or_scalar::*;

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
    array::VecStorageArrayApi<2> + array::VecStorageArrayApi<3> + array::VecStorageArrayApi<4>
{
}
impl VecAlignmentApi for VecPacked {}
impl VecAlignmentApi for VecAligned {}
