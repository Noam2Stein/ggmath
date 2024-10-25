mod core;
mod default;
pub use core::*;
pub use default::*;

use super::{ScalarCount, VecAligned, VecLen, VecPacked};

#[allow(private_bounds)]
pub(super) trait VecLenInterfaces<const N: usize>: VecLenCore<N> + VecLenDefault<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecLenInterfaces<2> for ScalarCount<2> {}
impl VecLenInterfaces<3> for ScalarCount<3> {}
impl VecLenInterfaces<4> for ScalarCount<4> {}

#[allow(private_bounds)]
pub(super) trait VecAlignmentInterfaces<const N: usize>:
    VecAlignmentCore<N> + VecAlignmentDefault<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecAlignmentInterfaces<2> for VecAligned {}
impl VecAlignmentInterfaces<3> for VecAligned {}
impl VecAlignmentInterfaces<4> for VecAligned {}
impl VecAlignmentInterfaces<2> for VecPacked {}
impl VecAlignmentInterfaces<3> for VecPacked {}
impl VecAlignmentInterfaces<4> for VecPacked {}