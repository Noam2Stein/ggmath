use super::*;

#[allow(private_bounds)]
pub trait VecLen<const N: usize>: Seal + VecLenInnerVec + VecLenApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
pub trait VecLenInnerVec {
    type InnerAlignedVec<T: ScalarAlignedVecs>: InnerConstruct;
}

pub struct ScalarCount<const VALUE: usize>;
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<4> {}
impl Seal for ScalarCount<3> {}
impl VecLen<2> for ScalarCount<2> {}
impl VecLen<3> for ScalarCount<3> {}
impl VecLen<4> for ScalarCount<4> {}
impl VecLenInnerVec for ScalarCount<2> {
    type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec2;
}
impl VecLenInnerVec for ScalarCount<3> {
    type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec4;
}
impl VecLenInnerVec for ScalarCount<4> {
    type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec4;
}

trait Seal {}
