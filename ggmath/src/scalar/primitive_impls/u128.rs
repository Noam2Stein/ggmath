use super::*;

inner_vectors!(u128(16));

impl Scalar for u128 {}

impl ScalarDefault for u128 {}
impl ScalarPartialEq for u128 {}
impl ScalarPartialOrd for u128 {}

impl ScalarAdd for u128 {}
impl ScalarSub for u128 {}
impl ScalarMul for u128 {}
impl ScalarDiv for u128 {}
impl ScalarRem for u128 {}
impl ScalarAddAssign for u128 {}
impl ScalarSubAssign for u128 {}
impl ScalarMulAssign for u128 {}
impl ScalarDivAssign for u128 {}
impl ScalarRemAssign for u128 {}

impl ScalarNot for u128 {}
impl ScalarBitAnd for u128 {}
impl ScalarBitOr for u128 {}
impl ScalarBitXor for u128 {}

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

impl ScalarAbsDiff for u128 {}
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
impl ScalarDot for u128 {}

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
