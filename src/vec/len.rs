use super::*;

#[allow(private_bounds)]
pub trait VecLen<const N: usize>: Seal + inner::VecLenInnerVec + VecLenApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}

pub struct ScalarCount<const VALUE: usize>;
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<4> {}
impl Seal for ScalarCount<3> {}
impl VecLen<2> for ScalarCount<2> {}
impl VecLen<3> for ScalarCount<3> {}
impl VecLen<4> for ScalarCount<4> {}

trait Seal {}
