use super::*;

inner_vectors!(i128(16));

impl Scalar for i128 {}

impl ScalarDefault for i128 {}
impl ScalarPartialEq for i128 {}
impl ScalarPartialOrd for i128 {}

impl ScalarNeg for i128 {}
impl ScalarAdd for i128 {}
impl ScalarSub for i128 {}
impl ScalarMul for i128 {}
impl ScalarDiv for i128 {}
impl ScalarRem for i128 {}
impl ScalarAddAssign for i128 {}
impl ScalarSubAssign for i128 {}
impl ScalarMulAssign for i128 {}
impl ScalarDivAssign for i128 {}
impl ScalarRemAssign for i128 {}

impl ScalarNot for i128 {}
impl ScalarBitAnd for i128 {}
impl ScalarBitOr for i128 {}
impl ScalarBitXor for i128 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i128 {}
            impl ScalarShr<$rhs> for i128 {}
            impl ScalarShlAssign<$rhs> for i128 {}
            impl ScalarShrAssign<$rhs> for i128 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for i128 {}
impl AbsDiff for i128 {
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

impl ScalarCSum for i128 {}
impl ScalarDot for i128 {}

impl ScalarRound for i128 {
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

impl ScalarNum for i128 {}

impl ScalarSigned for i128 {
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
