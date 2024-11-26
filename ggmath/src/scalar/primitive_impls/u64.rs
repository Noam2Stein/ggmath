use super::*;

inner_vectors!(u64(8));

impl Scalar for u64 {}

impl ScalarDefault for u64 {}
impl ScalarPartialEq for u64 {}
impl ScalarPartialOrd for u64 {}

impl ScalarAdd for u64 {}
impl ScalarSub for u64 {}
impl ScalarMul for u64 {}
impl ScalarDiv for u64 {}
impl ScalarRem for u64 {}
impl ScalarAddAssign for u64 {}
impl ScalarSubAssign for u64 {}
impl ScalarMulAssign for u64 {}
impl ScalarDivAssign for u64 {}
impl ScalarRemAssign for u64 {}

impl ScalarNot for u64 {}
impl ScalarBitAnd for u64 {}
impl ScalarBitOr for u64 {}
impl ScalarBitXor for u64 {}

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

impl ScalarAbsDiff for u64 {}
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

impl ScalarCSum for u64 {}
impl ScalarDot for u64 {}

impl ScalarRound for u64 {
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

impl ScalarNum for u64 {}
