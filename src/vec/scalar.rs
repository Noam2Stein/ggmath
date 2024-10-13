use super::*;

pub trait ScalarVec:
    ScalarInnerVecs
    + ScalarVecApiImpl<2, VecPacked>
    + ScalarVecApiImpl<3, VecPacked>
    + ScalarVecApiImpl<4, VecPacked>
    + ScalarVecApiImpl<2, VecAligned>
    + ScalarVecApiImpl<3, VecAligned>
    + ScalarVecApiImpl<4, VecAligned>
{
}
pub unsafe trait ScalarInnerVecs: Construct {
    type InnerAlignedVec2: InnerConstruct;
    type InnerAlignedVec3: InnerConstruct;
    type InnerAlignedVec4: InnerConstruct;
}
