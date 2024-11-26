use super::*;

inner_vectors!(u16(2));

impl Scalar for u16 {}

impl ScalarDefault for u16 {}
impl ScalarPartialEq for u16 {}
impl ScalarPartialOrd for u16 {}

impl ScalarAdd for u16 {}
impl ScalarSub for u16 {}
impl ScalarMul for u16 {}
impl ScalarDiv for u16 {}
impl ScalarRem for u16 {}
impl ScalarAddAssign for u16 {}
impl ScalarSubAssign for u16 {}
impl ScalarMulAssign for u16 {}
impl ScalarDivAssign for u16 {}
impl ScalarRemAssign for u16 {}

impl ScalarNot for u16 {}
impl ScalarBitAnd for u16 {}
impl ScalarBitOr for u16 {}
impl ScalarBitXor for u16 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u16 {}
            impl ScalarShr<$rhs> for u16 {}
            impl ScalarShlAssign<$rhs> for u16 {}
            impl ScalarShrAssign<$rhs> for u16 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for u16 {}
impl AbsDiff for u16 {
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

impl ScalarCSum for u16 {}
impl ScalarDot for u16 {}

impl ScalarRound for u16 {
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

impl ScalarNum for u16 {}
