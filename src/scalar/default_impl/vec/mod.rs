use crate::vec::{api::*, inner::*, *};

use super::*;

mod array;
mod swizzle;

impl<T: ScalarDefaultImpl> ScalarVecApi<2, VecPacked> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApi<3, VecPacked> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApi<4, VecPacked> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApi<2, VecAligned> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApi<3, VecAligned> for T {}
impl<T: ScalarDefaultImpl> ScalarVecApi<4, VecAligned> for T {}
