use super::*;

pub mod array;
pub mod construct;
pub mod or_scalar;

pub trait ScalarVecApiImpl<const N: usize, S: VecStorage>:
    inner::ScalarAlignedVecs + array::ScalarVecArrayApi<N, S>
where
    ScalarCount<N>: VecLen<N>,
{
}

#[allow(private_bounds)]
pub(super) trait VecLenApi<const N: usize>: array::VecLenArrayApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecLenApi<2> for ScalarCount<2> {}
impl VecLenApi<3> for ScalarCount<3> {}
impl VecLenApi<4> for ScalarCount<4> {}

#[allow(private_bounds)]
pub(super) trait VecStorageApi:
    array::VecStorageArrayApi<2> + array::VecStorageArrayApi<3> + array::VecStorageArrayApi<4>
{
}
impl VecStorageApi for VecPacked {}
impl VecStorageApi for VecAligned {}
