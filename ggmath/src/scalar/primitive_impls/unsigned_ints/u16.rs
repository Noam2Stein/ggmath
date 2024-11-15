use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(u16(2));

impl Scalar for u16 {}

impl ScalarDefault for u16 {}

impl ScalarAdd<u16> for u16 {}
impl ScalarSub<u16> for u16 {}
impl ScalarMul<u16> for u16 {}
impl ScalarDiv<u16> for u16 {}
impl ScalarRem<u16> for u16 {}
impl ScalarAddAssign<u16> for u16 {}
impl ScalarSubAssign<u16> for u16 {}
impl ScalarMulAssign<u16> for u16 {}
impl ScalarDivAssign<u16> for u16 {}
impl ScalarRemAssign<u16> for u16 {}

impl ScalarNot for u16 {}
impl ScalarBitAnd<u16> for u16 {}
impl ScalarBitOr<u16> for u16 {}
impl ScalarBitXor<u16> for u16 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u16 {}
            impl ScalarShr<$rhs> for u16 {}
            impl ScalarShlAssign<$rhs> for u16 {}
            impl ScalarShrAssign<$rhs> for u16 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarNum for u16 {}
