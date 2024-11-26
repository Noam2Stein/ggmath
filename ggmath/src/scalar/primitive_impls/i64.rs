use super::*;

inner_vectors!(i64(8));

impl Scalar for i64 {}

impl ScalarDefault for i64 {}
impl ScalarPartialEq for i64 {}
impl ScalarPartialOrd for i64 {}

impl ScalarNeg for i64 {}
impl ScalarAdd for i64 {}
impl ScalarSub for i64 {}
impl ScalarMul for i64 {}
impl ScalarDiv for i64 {}
impl ScalarRem for i64 {}
impl ScalarAddAssign for i64 {}
impl ScalarSubAssign for i64 {}
impl ScalarMulAssign for i64 {}
impl ScalarDivAssign for i64 {}
impl ScalarRemAssign for i64 {}

impl ScalarNot for i64 {}
impl ScalarBitAnd for i64 {}
impl ScalarBitOr for i64 {}
impl ScalarBitXor for i64 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i64 {}
            impl ScalarShr<$rhs> for i64 {}
            impl ScalarShlAssign<$rhs> for i64 {}
            impl ScalarShrAssign<$rhs> for i64 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for i64 {}
impl AbsDiff for i64 {
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

impl ScalarCSum for i64 {}
impl ScalarDot for i64 {}

impl ScalarRound for i64 {
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

impl ScalarNum for i64 {}

impl ScalarSigned for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
    fn is_negative(self) -> bool {
        self.is_negative()
    }
    fn is_positive(self) -> bool {
        self.is_positive()
    }
    fn signum(self) -> Self {
        self.signum()
    }
}
