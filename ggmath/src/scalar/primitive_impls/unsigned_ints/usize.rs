use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

#[cfg(target_pointer_width = "32")]
inner_vecs!(usize(4));

#[cfg(target_pointer_width = "64")]
inner_vecs!(usize(8));

impl Scalar for usize {}

impl ScalarDefault for usize {}

impl ScalarAdd<usize> for usize {}
impl ScalarSub<usize> for usize {}
impl ScalarMul<usize> for usize {}
impl ScalarDiv<usize> for usize {}
impl ScalarRem<usize> for usize {}
impl ScalarAddAssign<usize> for usize {}
impl ScalarSubAssign<usize> for usize {}
impl ScalarMulAssign<usize> for usize {}
impl ScalarDivAssign<usize> for usize {}
impl ScalarRemAssign<usize> for usize {}

impl ScalarNot for usize {}
impl ScalarBitAnd<usize> for usize {}
impl ScalarBitOr<usize> for usize {}
impl ScalarBitXor<usize> for usize {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for usize {}
            impl ScalarShr<$rhs> for usize {}
            impl ScalarShlAssign<$rhs> for usize {}
            impl ScalarShrAssign<$rhs> for usize {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarNum for usize {}
