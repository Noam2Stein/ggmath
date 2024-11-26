use super::*;

inner_vectors!(i32(4));

impl Scalar for i32 {}

impl ScalarDefault for i32 {}
impl ScalarPartialEq for i32 {}
impl ScalarPartialOrd for i32 {}

impl ScalarNeg for i32 {}
impl ScalarAdd for i32 {}
impl ScalarSub for i32 {}
impl ScalarMul for i32 {}
impl ScalarDiv for i32 {}
impl ScalarRem for i32 {}
impl ScalarAddAssign for i32 {}
impl ScalarSubAssign for i32 {}
impl ScalarMulAssign for i32 {}
impl ScalarDivAssign for i32 {}
impl ScalarRemAssign for i32 {}

impl ScalarNot for i32 {}
impl ScalarBitAnd for i32 {}
impl ScalarBitOr for i32 {}
impl ScalarBitXor for i32 {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for i32 {}
            impl ScalarShr<$rhs> for i32 {}
            impl ScalarShlAssign<$rhs> for i32 {}
            impl ScalarShrAssign<$rhs> for i32 {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff for i32 {}
impl AbsDiff for i32 {
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

impl ScalarCSum for i32 {}
impl ScalarDot for i32 {}

impl ScalarRound for i32 {
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

impl ScalarNum for i32 {}

impl ScalarSigned for i32 {
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
