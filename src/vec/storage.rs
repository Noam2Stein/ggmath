use super::*;

#[allow(private_bounds)]
pub trait VecStorage: Seal + VecStorageInnerVecs + VecStorageApi {}
pub(super) trait VecStorageInnerVecs: Sized + Seal {
    type InnerVec2<T: ScalarInnerVecs>: InnerConstruct;
    type InnerVec3<T: ScalarInnerVecs>: InnerConstruct;
    type InnerVec4<T: ScalarInnerVecs>: InnerConstruct;
}

pub struct VecPacked;
impl Seal for VecPacked {}
impl VecStorage for VecPacked {}
impl VecStorageInnerVecs for VecPacked {
    type InnerVec2<T: ScalarInnerVecs> = [T; 2];
    type InnerVec3<T: ScalarInnerVecs> = [T; 3];
    type InnerVec4<T: ScalarInnerVecs> = [T; 4];
}

pub struct VecAligned;
impl Seal for VecAligned {}
impl VecStorage for VecAligned {}
impl VecStorageInnerVecs for VecAligned {
    type InnerVec2<T: ScalarInnerVecs> = T::InnerAlignedVec2;
    type InnerVec3<T: ScalarInnerVecs> = T::InnerAlignedVec3;
    type InnerVec4<T: ScalarInnerVecs> = T::InnerAlignedVec4;
}
pub unsafe trait ScalarInnerVecs: Construct {
    type InnerAlignedVec2: InnerConstruct;
    type InnerAlignedVec3: InnerConstruct;
    type InnerAlignedVec4: InnerConstruct;
}

trait Seal {}
