use super::*;

pub trait ScalarVec:
    inner::ScalarAlignedVecs
    + ScalarVecApiImpl<2, VecPacked>
    + ScalarVecApiImpl<3, VecPacked>
    + ScalarVecApiImpl<4, VecPacked>
    + ScalarVecApiImpl<2, VecAligned>
    + ScalarVecApiImpl<3, VecAligned>
    + ScalarVecApiImpl<4, VecAligned>
{
}
