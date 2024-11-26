use super::*;

inner_vectors!(u32(4));

impl Scalar for u32 {}

impl ScalarDefault for u32 {}
impl ScalarPartialEq for u32 {}
impl ScalarPartialOrd for u32 {}

impl ScalarAdd for u32 {}
impl ScalarSub for u32 {}
impl ScalarMul for u32 {}
impl ScalarDiv for u32 {}
impl ScalarRem for u32 {}
impl ScalarAddAssign for u32 {}
impl ScalarSubAssign for u32 {}
impl ScalarMulAssign for u32 {}
impl ScalarDivAssign for u32 {}
impl ScalarRemAssign for u32 {}

impl ScalarNot for u32 {}
impl ScalarBitAnd for u32 {}
impl ScalarBitOr for u32 {}
impl ScalarBitXor for u32 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for u32 {}
            impl ScalarShr<$rhs> for u32 {}
            impl ScalarShlAssign<$rhs> for u32 {}
            impl ScalarShrAssign<$rhs> for u32 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for u32 {}
impl AbsDiff for u32 {
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

impl ScalarCSum for u32 {}
impl ScalarDot for u32 {}

impl ScalarRound for u32 {
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

impl ScalarNum for u32 {}
