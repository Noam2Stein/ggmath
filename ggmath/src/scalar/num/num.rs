use std::fmt::{Debug, Display};

use super::*;

pub trait ScalarNum:
    Scalar
    + Default
    + PartialEq
    + PartialOrd
    + Debug
    + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ScalarAbsDiff<Self, Output = Self>
    + ScalarRound
    + Num
    + NumCast
    + NumRef
    + NumAssign
    + NumAssignRef
{
}
