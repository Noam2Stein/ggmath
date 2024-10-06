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

vecnum_trait!(
    pub(super) trait VecNumSwizzle: VecNumGet + VecNumWith {}
);
