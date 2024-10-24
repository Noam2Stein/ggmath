mod core;
mod default;
pub use core::*;
pub use default::*;

use super::{ScalarCount, VecLen};

pub(super) trait VecLenInterfaces<const N: usize>: VecLenCore<N> + VecLenDefault<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
pub(super) trait VecAlignmentInterfaces<const N: usize>:
    VecAlignmentCore<N> + VecAlignmentDefault<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
