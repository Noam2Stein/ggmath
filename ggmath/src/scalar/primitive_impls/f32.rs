use super::*;

inner_vectors!(f32(4));

impl Scalar for f32 {}

impl ScalarDefault for f32 {}
impl ScalarPartialEq<f32> for f32 {}
impl ScalarPartialOrd for f32 {}

impl ScalarNeg for f32 {}
impl ScalarAdd<f32> for f32 {}
impl ScalarSub<f32> for f32 {}
impl ScalarMul<f32> for f32 {}
impl ScalarDiv<f32> for f32 {}
impl ScalarRem<f32> for f32 {}
impl ScalarAddAssign<f32> for f32 {}
impl ScalarSubAssign<f32> for f32 {}
impl ScalarMulAssign<f32> for f32 {}
impl ScalarDivAssign<f32> for f32 {}
impl ScalarRemAssign<f32> for f32 {}

impl ScalarAbsDiff<f32> for f32 {}
impl AbsDiff for f32 {
    type Output = Self;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Self) -> Self::Output {
        (self - rhs).abs()
    }
}

impl ScalarCSum for f32 {}
impl ScalarDot<f32> for f32 {}

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
