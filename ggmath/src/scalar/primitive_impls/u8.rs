use super::*;

inner_vectors!(u8(1));

impl Scalar for u8 {}

impl ScalarDefault for u8 {}
impl ScalarPartialEq<u8> for u8 {}
impl ScalarPartialOrd for u8 {}

impl ScalarAdd<u8> for u8 {}
impl ScalarSub<u8> for u8 {}
impl ScalarMul<u8> for u8 {}
impl ScalarDiv<u8> for u8 {}
impl ScalarRem<u8> for u8 {}
impl ScalarAddAssign<u8> for u8 {}
impl ScalarSubAssign<u8> for u8 {}
impl ScalarMulAssign<u8> for u8 {}
impl ScalarDivAssign<u8> for u8 {}
impl ScalarRemAssign<u8> for u8 {}

impl ScalarNot for u8 {}
impl ScalarBitAnd<u8> for u8 {}
impl ScalarBitOr<u8> for u8 {}
impl ScalarBitXor<u8> for u8 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u8 {}
            impl ScalarShr<$rhs> for u8 {}
            impl ScalarShlAssign<$rhs> for u8 {}
            impl ScalarShrAssign<$rhs> for u8 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<u8> for u8 {}
impl AbsDiff for u8 {
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

impl ScalarCSum for u8 {}
impl ScalarDot<u8> for u8 {}

impl ScalarRound for u8 {
    #[inline(always)]
    fn ceil(self) -> Self {
        self
    }
    #[inline(always)]
    fn floor(self) -> Self {
        self
    }
    #[inline(always)]
    fn round(self) -> Self {
        self
    }
    fn trunc(self) -> Self {
        self
    }
}

impl ScalarNum for u8 {}
