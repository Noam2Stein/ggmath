use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(i32(4));

impl Scalar for i32 {}

impl ScalarDefault for i32 {}
impl ScalarPartialEq<i32> for i32 {}
impl ScalarPartialOrd for i32 {}

impl ScalarNeg for i32 {}
impl ScalarAdd<i32> for i32 {}
impl ScalarSub<i32> for i32 {}
impl ScalarMul<i32> for i32 {}
impl ScalarDiv<i32> for i32 {}
impl ScalarRem<i32> for i32 {}
impl ScalarAddAssign<i32> for i32 {}
impl ScalarSubAssign<i32> for i32 {}
impl ScalarMulAssign<i32> for i32 {}
impl ScalarDivAssign<i32> for i32 {}
impl ScalarRemAssign<i32> for i32 {}

impl ScalarNot for i32 {}
impl ScalarBitAnd<i32> for i32 {}
impl ScalarBitOr<i32> for i32 {}
impl ScalarBitXor<i32> for i32 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i32 {}
            impl ScalarShr<$rhs> for i32 {}
            impl ScalarShlAssign<$rhs> for i32 {}
            impl ScalarShrAssign<$rhs> for i32 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarNum for i32 {}
