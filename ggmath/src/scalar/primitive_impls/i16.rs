use super::*;

inner_vectors!(i16(2));

impl Scalar for i16 {}

impl ScalarDefault for i16 {}
impl ScalarPartialEq<i16> for i16 {}
impl ScalarPartialOrd for i16 {}

impl ScalarNeg for i16 {}
impl ScalarAdd<i16> for i16 {}
impl ScalarSub<i16> for i16 {}
impl ScalarMul<i16> for i16 {}
impl ScalarDiv<i16> for i16 {}
impl ScalarRem<i16> for i16 {}
impl ScalarAddAssign<i16> for i16 {}
impl ScalarSubAssign<i16> for i16 {}
impl ScalarMulAssign<i16> for i16 {}
impl ScalarDivAssign<i16> for i16 {}
impl ScalarRemAssign<i16> for i16 {}

impl ScalarNot for i16 {}
impl ScalarBitAnd<i16> for i16 {}
impl ScalarBitOr<i16> for i16 {}
impl ScalarBitXor<i16> for i16 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i16 {}
            impl ScalarShr<$rhs> for i16 {}
            impl ScalarShlAssign<$rhs> for i16 {}
            impl ScalarShrAssign<$rhs> for i16 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<i16> for i16 {}
impl AbsDiff for i16 {
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

impl ScalarCSum for i16 {}
impl ScalarDot<i16> for i16 {}

impl ScalarNum for i16 {}
