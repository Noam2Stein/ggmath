use super::*;

mod get;
mod get_mut;
mod set;
mod with;
pub use get::*;
pub use with::*;

pub trait ElementVecSwizzle:
    ElementVecGet<2>
    + ElementVecGet<3>
    + ElementVecGet<4>
    + ElementVecWith<2>
    + ElementVecWith<3>
    + ElementVecWith<4>
{
}

pub(super) trait VecNumSwizzle<const N: usize>: VecNumGet<N>
where
    MaybeVecNum<N>: VecNum<N>,
{
}
impl VecNumSwizzle<2> for MaybeVecNum<2> {}
impl VecNumSwizzle<3> for MaybeVecNum<3> {}
impl VecNumSwizzle<4> for MaybeVecNum<4> {}
