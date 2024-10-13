use super::*;

#[allow(private_bounds)]
pub trait VecStorage: Seal + VecStorageInnerVecs + VecStorageApi {}
pub trait VecStorageInnerVecs: Sized {
    type InnerVec<const N: usize, T: ScalarInnerVecs>: InnerConstruct
    where
        ScalarCount<N>: VecLenInnerVec;
}

pub struct VecPacked;
impl Seal for VecPacked {}
impl VecStorage for VecPacked {}
impl VecStorageInnerVecs for VecPacked {
    type InnerVec<const N: usize, T: ScalarInnerVecs> = [T; N] where ScalarCount<N>: VecLenInnerVec;
}

pub struct VecAligned;
impl Seal for VecAligned {}
impl VecStorage for VecAligned {}
impl VecStorageInnerVecs for VecAligned {
    type InnerVec<const N: usize, T: ScalarInnerVecs> =
        <ScalarCount<N> as VecLenInnerVec>::InnerAlignedVec<T> where ScalarCount<N>: VecLenInnerVec;
}

trait Seal {}
