use super::*;

pub type InnerVector<const N: usize, T, S> = <S as VecStorageInnerVecs>::InnerVec<N, T>;

pub trait VecLenInnerVec: Seal {
    type InnerAlignedVec<T: ScalarAlignedVecs>: InnerConstruct;
}
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<4> {}
impl Seal for ScalarCount<3> {}
impl VecLenInnerVec for ScalarCount<2> {
    type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec2;
}
impl VecLenInnerVec for ScalarCount<3> {
    type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec4;
}
impl VecLenInnerVec for ScalarCount<4> {
    type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec4;
}

pub trait VecStorageInnerVecs: Seal {
    type InnerVec<const N: usize, T: ScalarAlignedVecs>: InnerConstruct
    where
        ScalarCount<N>: VecLenInnerVec;
}
impl Seal for VecPacked {}
impl VecStorageInnerVecs for VecPacked {
    type InnerVec<const N: usize, T: ScalarAlignedVecs> = [T; N] where ScalarCount<N>: VecLenInnerVec;
}
impl Seal for VecAligned {}
impl VecStorageInnerVecs for VecAligned {
    type InnerVec<const N: usize, T: ScalarAlignedVecs> =
        <ScalarCount<N> as VecLenInnerVec>::InnerAlignedVec<T> where ScalarCount<N>: VecLenInnerVec;
}

pub unsafe trait ScalarAlignedVecs: Construct {
    type InnerAlignedVec2: InnerConstruct;
    type InnerAlignedVec4: InnerConstruct;
}

pub use gomath_proc_macros::aligned_vecs;

trait Seal: Sized {}
