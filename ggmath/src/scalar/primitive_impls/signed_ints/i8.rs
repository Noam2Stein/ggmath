use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(i8(1));

impl Scalar for i8 {}

impl ScalarNeg for i8 {}
impl ScalarAdd<i8> for i8 {}
impl ScalarSub<i8> for i8 {}
impl ScalarMul<i8> for i8 {}
impl ScalarDiv<i8> for i8 {}
impl ScalarRem<i8> for i8 {}
impl ScalarAddAssign<i8> for i8 {}
impl ScalarSubAssign<i8> for i8 {}
impl ScalarMulAssign<i8> for i8 {}
impl ScalarDivAssign<i8> for i8 {}
impl ScalarRemAssign<i8> for i8 {}

impl ScalarNot for i8 {}
impl ScalarBitAnd<i8> for i8 {}
impl ScalarBitOr<i8> for i8 {}
impl ScalarBitXor<i8> for i8 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i8 {}
            impl ScalarShr<$rhs> for i8 {}
            impl ScalarShlAssign<$rhs> for i8 {}
            impl ScalarShrAssign<$rhs> for i8 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
