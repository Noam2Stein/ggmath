use std::fmt::{Debug, Display};

use super::*;

pub trait ScalarNum:
    Scalar
    + ScalarDefault
    + ScalarPartialEq<Self>
    + ScalarPartialOrd
    + Debug
    + Display
    + ScalarAdd<Self, Output = Self>
    + ScalarSub<Self, Output = Self>
    + ScalarMul<Self, Output = Self>
    + ScalarDiv<Self, Output = Self>
    + ScalarRem<Self, Output = Self>
    + ScalarAddAssign<Self>
    + ScalarSubAssign<Self>
    + ScalarMulAssign<Self>
    + ScalarDivAssign<Self>
    + ScalarRemAssign<Self>
    + ScalarAbsDiff<Self, Output = Self>
    + Num
    + NumCast
    + NumRef
    + NumAssign
    + NumAssignRef
{
}
