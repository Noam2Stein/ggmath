mod core;
mod default;
mod num;
mod ops;
pub use core::*;
pub use default::*;
pub use num::*;
pub use ops::*;

use super::{ScalarCount, VecAligned, VecLen, VecPacked};

pub(crate) mod scalar_traits {
    pub use super::*;
}

ggmath_proc_macros::interfaces_mod_traits!(
    Scalar

    ScalarDefault

    ScalarNeg
    ScalarNot

    ScalarAdd
    ScalarBitOr
    ScalarBitAnd
    ScalarBitXor
    ScalarDiv
    ScalarMul
    ScalarRem
    ScalarShl
    ScalarShr
    ScalarSub

    ScalarAddAssign
    ScalarBitAndAssign
    ScalarBitOrAssign
    ScalarBitXorAssign
    ScalarDivAssign
    ScalarMulAssign
    ScalarRemAssign
    ScalarShlAssign
    ScalarShrAssign
    ScalarSubAssign
);

impl VecLenInterfaces<2> for ScalarCount<2> {}
impl VecLenInterfaces<3> for ScalarCount<3> {}
impl VecLenInterfaces<4> for ScalarCount<4> {}

impl VecAlignmentInterfaces<2> for VecAligned {}
impl VecAlignmentInterfaces<3> for VecAligned {}
impl VecAlignmentInterfaces<4> for VecAligned {}
impl VecAlignmentInterfaces<2> for VecPacked {}
impl VecAlignmentInterfaces<3> for VecPacked {}
impl VecAlignmentInterfaces<4> for VecPacked {}
