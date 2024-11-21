use super::*;

inner_vectors!(u64(8));

impl Scalar for u64 {}

impl ScalarDefault for u64 {}
impl ScalarPartialEq<u64> for u64 {}
impl ScalarPartialOrd for u64 {}

impl ScalarAdd<u64> for u64 {}
impl ScalarSub<u64> for u64 {}
impl ScalarMul<u64> for u64 {}
impl ScalarDiv<u64> for u64 {}
impl ScalarRem<u64> for u64 {}
impl ScalarAddAssign<u64> for u64 {}
impl ScalarSubAssign<u64> for u64 {}
impl ScalarMulAssign<u64> for u64 {}
impl ScalarDivAssign<u64> for u64 {}
impl ScalarRemAssign<u64> for u64 {}

impl ScalarNot for u64 {}
impl ScalarBitAnd<u64> for u64 {}
impl ScalarBitOr<u64> for u64 {}
impl ScalarBitXor<u64> for u64 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u64 {}
            impl ScalarShr<$rhs> for u64 {}
            impl ScalarShlAssign<$rhs> for u64 {}
            impl ScalarShrAssign<$rhs> for u64 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<u64> for u64 {}
impl AbsDiff for u64 {
    type Output = Self;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Self) -> Self::Output {
        if self > rhs {
            self - rhs
        } else {
            rhs - self
        }
    }
}

impl ScalarNum for u64 {}