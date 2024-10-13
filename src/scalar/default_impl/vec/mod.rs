use crate::vec::*;

use super::*;

mod array;

impl<T: ScalarDefaultImpl> ScalarVec for T {}

impl<T: ScalarDefaultImpl> ScalarVecApiImpl<2, VecPacked> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApiImpl<3, VecPacked> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApiImpl<4, VecPacked> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApiImpl<2, VecAligned> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApiImpl<3, VecAligned> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApiImpl<4, VecAligned> for T {}
