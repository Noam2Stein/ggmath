use super::*;

#[allow(private_bounds)]
pub trait VecLen<const N: usize>: Seal + VecLenApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
    type InnerVector<S: VecStorageInnerVecs + ?Sized, T: ScalarInnerVecs>: InnerConstruct;
}
pub struct ScalarCount<const VALUE: usize>;
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<4> {}
impl Seal for ScalarCount<3> {}
impl VecLen<2> for ScalarCount<2> {
    type InnerVector<S: VecStorageInnerVecs + ?Sized, T: ScalarInnerVecs> = S::InnerVec2<T>;
}
impl VecLen<3> for ScalarCount<3> {
    type InnerVector<S: VecStorageInnerVecs + ?Sized, T: ScalarInnerVecs> = S::InnerVec3<T>;
}
impl VecLen<4> for ScalarCount<4> {
    type InnerVector<S: VecStorageInnerVecs + ?Sized, T: ScalarInnerVecs> = S::InnerVec4<T>;
}

trait Seal {}
