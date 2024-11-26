use super::*;

inner_vectors!(f32(4));

impl Scalar for f32 {}

impl ScalarDefault for f32 {}
impl ScalarPartialEq for f32 {}
impl ScalarPartialOrd for f32 {}

impl ScalarNeg for f32 {}
impl ScalarAdd for f32 {}
impl ScalarSub for f32 {}
impl ScalarMul for f32 {}
impl ScalarDiv for f32 {}
impl ScalarRem for f32 {}
impl ScalarAddAssign for f32 {}
impl ScalarSubAssign for f32 {}
impl ScalarMulAssign for f32 {}
impl ScalarDivAssign for f32 {}
impl ScalarRemAssign for f32 {}

impl ScalarAbsDiff for f32 {}
impl AbsDiff for f32 {
    type Output = Self;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Self) -> Self::Output {
        (self - rhs).abs()
    }
}

impl ScalarCSum for f32 {}
impl ScalarDot for f32 {}

impl ScalarRound for f32 {
    #[inline(always)]
    fn ceil(self) -> Self {
        self.ceil()
    }
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor()
    }
    #[inline(always)]
    fn round(self) -> Self {
        self.round()
    }
    fn trunc(self) -> Self {
        self.trunc()
    }
}

impl ScalarNum for f32 {}

impl ScalarSigned for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
    fn is_negative(self) -> bool {
        self.is_sign_negative()
    }
    fn is_positive(self) -> bool {
        self.is_sign_positive()
    }
    fn signum(self) -> Self {
        self.signum()
    }
}
