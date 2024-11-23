use super::*;

inner_vectors!(u128(16));

impl Scalar for u128 {}

impl ScalarDefault for u128 {}
impl ScalarPartialEq<u128> for u128 {}
impl ScalarPartialOrd for u128 {}

impl ScalarAdd<u128> for u128 {}
impl ScalarSub<u128> for u128 {}
impl ScalarMul<u128> for u128 {}
impl ScalarDiv<u128> for u128 {}
impl ScalarRem<u128> for u128 {}
impl ScalarAddAssign<u128> for u128 {}
impl ScalarSubAssign<u128> for u128 {}
impl ScalarMulAssign<u128> for u128 {}
impl ScalarDivAssign<u128> for u128 {}
impl ScalarRemAssign<u128> for u128 {}

impl ScalarNot for u128 {}
impl ScalarBitAnd<u128> for u128 {}
impl ScalarBitOr<u128> for u128 {}
impl ScalarBitXor<u128> for u128 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u128 {}
            impl ScalarShr<$rhs> for u128 {}
            impl ScalarShlAssign<$rhs> for u128 {}
            impl ScalarShrAssign<$rhs> for u128 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<u128> for u128 {}
impl AbsDiff for u128 {
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

impl ScalarCSum for u128 {}
impl ScalarDot<u128> for u128 {}

impl ScalarRound for u128 {
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

impl ScalarNum for u128 {}
