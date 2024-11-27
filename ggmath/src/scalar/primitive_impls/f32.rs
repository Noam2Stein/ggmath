use super::*;

inner_vectors!(f32(4));

impl Scalar for f32 {}

impl ScalarAbsDiff for f32 {}
impl AbsDiff for f32 {
    type Output = Self;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Self) -> Self::Output {
        (self - rhs).abs()
    }
}

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
