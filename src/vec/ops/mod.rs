use std::ops::*;

use super::*;
use crate::element::ops::*;

mod assign_ops;
mod component_dot;
mod component_sum;
mod rhs_ops;
mod self_ops;
pub use assign_ops::*;
pub use rhs_ops::*;
pub use self_ops::*;

pub(super) trait VecNumOps<const N: usize>: VecNumSelfOps<N> + VecNumRhsOps<N>
where
    MaybeVecNum<N>: VecNum<N>,
{
}
impl VecNumOps<2> for MaybeVecNum<2> {}
impl VecNumOps<3> for MaybeVecNum<3> {}
impl VecNumOps<4> for MaybeVecNum<4> {}
