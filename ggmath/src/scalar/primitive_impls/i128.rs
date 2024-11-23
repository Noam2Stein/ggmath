use super::*;

inner_vectors!(i128(16));

impl Scalar for i128 {}

impl ScalarDefault for i128 {}
impl ScalarPartialEq<i128> for i128 {}
impl ScalarPartialOrd for i128 {}

impl ScalarNeg for i128 {}
impl ScalarAdd<i128> for i128 {}
impl ScalarSub<i128> for i128 {}
impl ScalarMul<i128> for i128 {}
impl ScalarDiv<i128> for i128 {}
impl ScalarRem<i128> for i128 {}
impl ScalarAddAssign<i128> for i128 {}
impl ScalarSubAssign<i128> for i128 {}
impl ScalarMulAssign<i128> for i128 {}
impl ScalarDivAssign<i128> for i128 {}
impl ScalarRemAssign<i128> for i128 {}

impl ScalarNot for i128 {}
impl ScalarBitAnd<i128> for i128 {}
impl ScalarBitOr<i128> for i128 {}
impl ScalarBitXor<i128> for i128 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i128 {}
            impl ScalarShr<$rhs> for i128 {}
            impl ScalarShlAssign<$rhs> for i128 {}
            impl ScalarShrAssign<$rhs> for i128 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<i128> for i128 {}
impl AbsDiff for i128 {
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

impl ScalarCSum for i128 {}
impl ScalarDot<i128> for i128 {}

impl ScalarNum for i128 {}
