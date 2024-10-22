//! Scalar supertraits for vector API

use super::*;

mod array;
mod swizzle;
pub use array::*;
pub use swizzle::*;

mod construct;

/// Scalar supertrait for implementing the vector API for a specific <N, S>.
/// This is an empty trait with more supertraits for every related fn set.
pub trait ScalarVecApi<const N: usize, A: VecAlignment>:
    inner::ScalarInnerVecs + ScalarVecArrayApi<N, A> + ScalarVecSwizzleApi<N, A>
where
    ScalarCount<N>: VecLen<N>,
{
}

#[allow(private_bounds)]
pub(super) trait VecLenApi<const N: usize>: VecLenArrayApi<N> + VecLenSwizzleApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecLenApi<2> for ScalarCount<2> {}
impl VecLenApi<3> for ScalarCount<3> {}
impl VecLenApi<4> for ScalarCount<4> {}

#[allow(private_bounds)]
pub(super) trait VecAlignmentApi<const N: usize>:
    VecAlignmentArrayApi<N> + VecAlignmentSwizzleApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecAlignmentApi<2> for VecPacked {}
impl VecAlignmentApi<3> for VecPacked {}
impl VecAlignmentApi<4> for VecPacked {}
impl VecAlignmentApi<2> for VecAligned {}
impl VecAlignmentApi<3> for VecAligned {}
impl VecAlignmentApi<4> for VecAligned {}

mod prelude {
    pub use crate::{
        scalar::*,
        vec::{inner::*, *},
    };
}
