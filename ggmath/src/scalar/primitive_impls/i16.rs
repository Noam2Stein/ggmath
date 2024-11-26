use super::*;

inner_vectors!(i16(2));

impl Scalar for i16 {}

impl ScalarDefault for i16 {}
impl ScalarPartialEq for i16 {}
impl ScalarPartialOrd for i16 {}

impl ScalarNeg for i16 {}
impl ScalarAdd for i16 {}
impl ScalarSub for i16 {}
impl ScalarMul for i16 {}
impl ScalarDiv for i16 {}
impl ScalarRem for i16 {}
impl ScalarAddAssign for i16 {}
impl ScalarSubAssign for i16 {}
impl ScalarMulAssign for i16 {}
impl ScalarDivAssign for i16 {}
impl ScalarRemAssign for i16 {}

impl ScalarNot for i16 {}
impl ScalarBitAnd for i16 {}
impl ScalarBitOr for i16 {}
impl ScalarBitXor for i16 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i16 {}
            impl ScalarShr<$rhs> for i16 {}
            impl ScalarShlAssign<$rhs> for i16 {}
            impl ScalarShrAssign<$rhs> for i16 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for i16 {}
impl AbsDiff for i16 {
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

impl ScalarCSum for i16 {}
impl ScalarDot for i16 {}

impl ScalarRound for i16 {
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

impl ScalarNum for i16 {}

impl ScalarSigned for i16 {
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
