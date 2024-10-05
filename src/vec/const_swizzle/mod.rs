use super::*;

mod cget;
mod cget_mut;
mod cset;
mod cwith;
pub use cget::*;
pub use cwith::*;

pub trait ElementVecConstSwizzle:
    ElementVecConstGet<2>
    + ElementVecConstGet<3>
    + ElementVecConstGet<4>
    + ElementVecConstWith<2>
    + ElementVecConstWith<3>
    + ElementVecConstWith<4>
{
}

pub(super) trait VecNumConstSwizzle<const N: usize>:
    VecNumConstGet<N> + VecNumConstWith<N>
where
    MaybeVecNum<N>: VecNum<N>,
{
}
impl VecNumConstSwizzle<2> for MaybeVecNum<2> {}
impl VecNumConstSwizzle<3> for MaybeVecNum<3> {}
impl VecNumConstSwizzle<4> for MaybeVecNum<4> {}
