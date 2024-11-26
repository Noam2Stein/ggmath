use super::*;

inner_vectors!(u8(1));

impl Scalar for u8 {}

impl ScalarDefault for u8 {}
impl ScalarPartialEq for u8 {}
impl ScalarPartialOrd for u8 {}

impl ScalarAdd for u8 {}
impl ScalarSub for u8 {}
impl ScalarMul for u8 {}
impl ScalarDiv for u8 {}
impl ScalarRem for u8 {}
impl ScalarAddAssign for u8 {}
impl ScalarSubAssign for u8 {}
impl ScalarMulAssign for u8 {}
impl ScalarDivAssign for u8 {}
impl ScalarRemAssign for u8 {}

impl ScalarNot for u8 {}
impl ScalarBitAnd for u8 {}
impl ScalarBitOr for u8 {}
impl ScalarBitXor for u8 {}

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

impl ScalarAbsDiff for u8 {}
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
impl ScalarDot for u8 {}

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
