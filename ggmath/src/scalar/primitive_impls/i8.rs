use super::*;

inner_vectors!(i8(1));

impl Scalar for i8 {}

impl ScalarDefault for i8 {}
impl ScalarPartialEq<i8> for i8 {}
impl ScalarPartialOrd for i8 {}

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

impl ScalarAbsDiff<i8> for i8 {}
impl AbsDiff for i8 {
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

impl ScalarCSum for i8 {}
impl ScalarDot<i8> for i8 {}

impl ScalarNum for i8 {}
