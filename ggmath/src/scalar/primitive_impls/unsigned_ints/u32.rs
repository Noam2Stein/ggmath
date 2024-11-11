use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(u32(4));

impl Scalar for u32 {}

impl ScalarAdd<u32> for u32 {}
impl ScalarSub<u32> for u32 {}
impl ScalarMul<u32> for u32 {}
impl ScalarDiv<u32> for u32 {}
impl ScalarRem<u32> for u32 {}
impl ScalarAddAssign<u32> for u32 {}
impl ScalarSubAssign<u32> for u32 {}
impl ScalarMulAssign<u32> for u32 {}
impl ScalarDivAssign<u32> for u32 {}
impl ScalarRemAssign<u32> for u32 {}

impl ScalarNot for u32 {}
impl ScalarBitAnd<u32> for u32 {}
impl ScalarBitOr<u32> for u32 {}
impl ScalarBitXor<u32> for u32 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u32 {}
            impl ScalarShr<$rhs> for u32 {}
            impl ScalarShlAssign<$rhs> for u32 {}
            impl ScalarShrAssign<$rhs> for u32 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
