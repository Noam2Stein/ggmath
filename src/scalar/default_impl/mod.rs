use crate::vec::{array::ScalarVecArrayApi, ScalarInnerVecs, VecAligned};

use super::*;

mod vec;

pub trait ScalarDefaultImpl:
    Construct
    + ScalarInnerVecs
    + ScalarVecArrayApi<2, VecAligned>
    + ScalarVecArrayApi<3, VecAligned>
    + ScalarVecArrayApi<4, VecAligned>
{
}

impl<T: ScalarDefaultImpl> Scalar for T {}
