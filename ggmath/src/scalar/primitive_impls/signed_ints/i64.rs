use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(i64(8));

impl Scalar for i64 {}

impl ScalarDefault for i64 {}

impl ScalarNeg for i64 {}
impl ScalarAdd<i64> for i64 {}
impl ScalarSub<i64> for i64 {}
impl ScalarMul<i64> for i64 {}
impl ScalarDiv<i64> for i64 {}
impl ScalarRem<i64> for i64 {}
impl ScalarAddAssign<i64> for i64 {}
impl ScalarSubAssign<i64> for i64 {}
impl ScalarMulAssign<i64> for i64 {}
impl ScalarDivAssign<i64> for i64 {}
impl ScalarRemAssign<i64> for i64 {}

impl ScalarNot for i64 {}
impl ScalarBitAnd<i64> for i64 {}
impl ScalarBitOr<i64> for i64 {}
impl ScalarBitXor<i64> for i64 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i64 {}
            impl ScalarShr<$rhs> for i64 {}
            impl ScalarShlAssign<$rhs> for i64 {}
            impl ScalarShrAssign<$rhs> for i64 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarNum for i64 {}
