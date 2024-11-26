use super::*;

inner_vectors!(i8(1));

impl Scalar for i8 {}

impl ScalarDefault for i8 {}
impl ScalarPartialEq for i8 {}
impl ScalarPartialOrd for i8 {}

impl ScalarNeg for i8 {}
impl ScalarAdd for i8 {}
impl ScalarSub for i8 {}
impl ScalarMul for i8 {}
impl ScalarDiv for i8 {}
impl ScalarRem for i8 {}
impl ScalarAddAssign for i8 {}
impl ScalarSubAssign for i8 {}
impl ScalarMulAssign for i8 {}
impl ScalarDivAssign for i8 {}
impl ScalarRemAssign for i8 {}

impl ScalarNot for i8 {}
impl ScalarBitAnd for i8 {}
impl ScalarBitOr for i8 {}
impl ScalarBitXor for i8 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i8 {}
            impl ScalarShr<$rhs> for i8 {}
            impl ScalarShlAssign<$rhs> for i8 {}
            impl ScalarShrAssign<$rhs> for i8 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for i8 {}
impl AbsDiff for i8 {
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

impl ScalarCSum for i8 {}
impl ScalarDot for i8 {}

impl ScalarRound for i8 {
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

impl ScalarNum for i8 {}

impl ScalarSigned for i8 {
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
